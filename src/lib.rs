use std::ffi::CString;
use std::mem;

use ext_php_rs::boxed::ZBox;
use ext_php_rs::ffi::{zend_hash_index_update, zend_hash_next_index_insert, zend_hash_str_update};
use ext_php_rs::internal::function::PhpFunction;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ArrayKey, ZendHashTable, Zval};

pub mod toon;
use toon::ToonValue;

#[php_function]
#[php(name = "toon_decode")]
pub fn rust_toon_decode(input: String) -> PhpResult<Zval> {
    match toon::parse(&input) {
        Ok(val) => toon_value_to_zval(val),
        Err(e) => Err(PhpException::default(e.to_string())),
    }
}

#[php_function]
#[php(name = "toon_encode")]
pub fn rust_toon_encode(input: &Zval) -> PhpResult<String> {
    let val = zval_to_toon_value(input)?;
    Ok(toon::encode(&val))
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function((_internal_rust_toon_decode::FUNCTION_ENTRY)())
        .function((_internal_rust_toon_encode::FUNCTION_ENTRY)())
}

// --- Helpers ---

const MAX_RECURSION_DEPTH: usize = 60;

fn toon_value_to_zval(val: ToonValue) -> PhpResult<Zval> {
    toon_value_to_zval_impl(val, 0)
}

fn toon_value_to_zval_impl(val: ToonValue, depth: usize) -> PhpResult<Zval> {
    if depth > MAX_RECURSION_DEPTH {
        return Err(PhpException::default(
            "Recursion depth limit exceeded".to_string(),
        ));
    }

    let mut zval = Zval::new();
    match val {
        ToonValue::Null => zval.set_null(),
        ToonValue::Bool(b) => zval.set_bool(b),
        ToonValue::Int(i) => zval.set_long(i),
        ToonValue::Float(f) => zval.set_double(f),
        ToonValue::String(s) => zval.set_string(&s, false)?,
        ToonValue::Array(arr) => {
            let ht = build_php_list(arr, depth + 1)?;
            zval.set_hashtable(ht);
        }
        ToonValue::Map(map) => {
            let ht = build_php_map(map, depth + 1)?;
            zval.set_hashtable(ht);
        }
    }
    Ok(zval)
}

fn zval_to_toon_value(zval: &Zval) -> PhpResult<ToonValue> {
    zval_to_toon_value_impl(zval, 0)
}

fn zval_to_toon_value_impl(zval: &Zval, depth: usize) -> PhpResult<ToonValue> {
    if depth > MAX_RECURSION_DEPTH {
        return Err(PhpException::default(
            "Recursion depth limit exceeded".to_string(),
        ));
    }

    if zval.is_null() {
        return Ok(ToonValue::Null);
    }
    if zval.is_true() {
        return Ok(ToonValue::Bool(true));
    }
    if zval.is_false() {
        return Ok(ToonValue::Bool(false));
    }
    if zval.is_long() {
        return Ok(ToonValue::Int(zval.long().unwrap_or(0)));
    }
    if zval.is_double() {
        return Ok(ToonValue::Float(zval.double().unwrap_or(0.0)));
    }
    if zval.is_string() {
        return Ok(ToonValue::String(zval.string().unwrap_or_default()));
    }
    if zval.is_array() {
        let ht = zval.array().unwrap();
        let len = ht.len();

        // Detect list candidacy and defer allocating map storage until required
        let mut expected_idx = 0usize;
        let mut is_list_candidate = true;
        let mut list_items: Vec<ToonValue> = Vec::with_capacity(len);
        let mut map_entries: Option<Vec<(String, ToonValue)>> = None;

        for (k, v) in ht.iter() {
            let mut treat_as_list_entry = is_list_candidate;

            if treat_as_list_entry {
                match k {
                    ArrayKey::Long(idx) if idx == expected_idx as i64 => {
                        expected_idx += 1;
                    }
                    _ => {
                        treat_as_list_entry = false;
                        is_list_candidate = false;
                    }
                }
            }

            let val = zval_to_toon_value_impl(v, depth + 1)?;
            
            // If the value is a Map, it cannot be represented in an inline list (in this format),
            // so the container must become a Map.
            // Lists containing Lists are fine.
            if matches!(val, ToonValue::Map(_)) {
                if treat_as_list_entry {
                    treat_as_list_entry = false;
                    is_list_candidate = false;
                }
            }

            if treat_as_list_entry {
                list_items.push(val);
                continue;
            }

            let entries = map_entries.get_or_insert_with(|| {
                let mut vec = Vec::with_capacity(len);
                for (idx, prev_val) in list_items.drain(..).enumerate() {
                    vec.push((idx.to_string(), prev_val));
                }
                vec
            });

            let key_str = match k {
                ArrayKey::Long(idx) => idx.to_string(),
                ArrayKey::String(s) => s,
                ArrayKey::Str(s) => s.to_string(),
            };
            entries.push((key_str, val));
        }

        if is_list_candidate {
            return Ok(ToonValue::Array(list_items));
        }

        if let Some(entries) = map_entries {
            return Ok(ToonValue::Map(entries));
        }

        // Empty PHP arrays default to empty lists
        return Ok(ToonValue::Array(Vec::new()));
    }

    // Fallback
    Ok(ToonValue::String(zval.string().unwrap_or_default()))
}

fn build_php_list(items: Vec<ToonValue>, depth: usize) -> PhpResult<ZBox<ZendHashTable>> {
    let mut ht = ZendHashTable::with_capacity(clamped_capacity(items.len()));
    for item in items {
        let mut child = toon_value_to_zval_impl(item, depth)?;
        unsafe {
            let result = zend_hash_next_index_insert(&mut *ht, std::ptr::addr_of_mut!(child));
            if result.is_null() {
                return Err(PhpException::default(
                    "Failed to insert into PHP list".to_string(),
                ));
            }
        }
        mem::forget(child);
    }
    Ok(ht)
}

fn build_php_map(
    entries: Vec<(String, ToonValue)>,
    depth: usize,
) -> PhpResult<ZBox<ZendHashTable>> {
    let mut ht = ZendHashTable::with_capacity(clamped_capacity(entries.len()));
    for (key, value) in entries {
        let mut child = toon_value_to_zval_impl(value, depth)?;
        
        // Optimization: Only attempt to parse as integer if it looks like one.
        // This avoids expensive parsing for common string keys.
        let maybe_index = if !key.is_empty() && (key.as_bytes()[0] == b'-' || key.as_bytes()[0].is_ascii_digit()) {
            key.parse::<i64>().ok()
        } else {
            None
        };

        unsafe {
            let result = if let Some(idx) = maybe_index {
                #[allow(clippy::cast_sign_loss)]
                zend_hash_index_update(&mut *ht, idx as u64, std::ptr::addr_of_mut!(child))
            } else {
                let c_key = CString::new(key.as_str())
                    .map_err(|_| PhpException::default("Map key contains null byte".to_string()))?;
                zend_hash_str_update(
                    &mut *ht,
                    c_key.as_ptr(),
                    key.len(),
                    std::ptr::addr_of_mut!(child),
                )
            };

            if result.is_null() {
                return Err(PhpException::default(
                    "Failed to insert into PHP map".to_string(),
                ));
            }
        }
        mem::forget(child);
    }
    Ok(ht)
}

fn clamped_capacity(len: usize) -> u32 {
    let max = u32::MAX as usize;
    if len > max {
        u32::MAX
    } else {
        len as u32
    }
}
