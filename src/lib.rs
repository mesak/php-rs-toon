use ext_php_rs::prelude::*;
use ext_php_rs::types::{Zval, ArrayKey};
use ext_php_rs::internal::function::PhpFunction;

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

const MAX_RECURSION_DEPTH: usize = 100;

fn toon_value_to_zval(val: ToonValue) -> PhpResult<Zval> {
    toon_value_to_zval_impl(val, 0)
}

fn toon_value_to_zval_impl(val: ToonValue, depth: usize) -> PhpResult<Zval> {
    if depth > MAX_RECURSION_DEPTH {
        return Err(PhpException::default("Recursion depth limit exceeded".to_string()));
    }
    
    let mut zval = Zval::new();
    match val {
        ToonValue::Null => zval.set_null(),
        ToonValue::Bool(b) => zval.set_bool(b),
        ToonValue::Int(i) => zval.set_long(i),
        ToonValue::Float(f) => zval.set_double(f),
        ToonValue::String(s) => zval.set_string(&s, false)?,
        ToonValue::Array(arr) => {
            let mut vec = Vec::with_capacity(arr.len());
            for item in arr {
                vec.push(toon_value_to_zval_impl(item, depth + 1)?);
            }
            zval.set_array(vec).map_err(|e| PhpException::default(e.to_string()))?;
        }
        ToonValue::Map(map) => {
            let mut entries = Vec::with_capacity(map.len());
            for (k, v) in map {
                entries.push((k, toon_value_to_zval_impl(v, depth + 1)?));
            }
            zval.set_array(entries).map_err(|e| PhpException::default(e.to_string()))?;
        }
    }
    Ok(zval)
}

fn zval_to_toon_value(zval: &Zval) -> PhpResult<ToonValue> {
    zval_to_toon_value_impl(zval, 0)
}

fn zval_to_toon_value_impl(zval: &Zval, depth: usize) -> PhpResult<ToonValue> {
    if depth > MAX_RECURSION_DEPTH {
        return Err(PhpException::default("Recursion depth limit exceeded".to_string()));
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
        
        // Check if it's a sequential array (list) or associative (map)
        // PHP arrays are always ordered maps, but TOON distinguishes between lists and maps.
        // Heuristic: if all keys are sequential integers starting from 0, treat as Array.
        // Otherwise treat as Map.
        
        let mut is_list = true;
        let mut expected_idx = 0;
        let mut entries = Vec::with_capacity(len);
        let mut has_complex = false;
        
        // Single pass: check list conditions and complex types simultaneously
        // Optimized: single pattern match for key checking and conversion
        for (k, v) in ht.iter() {
            let val = zval_to_toon_value_impl(v, depth + 1)?;
            
            // Check if value is complex (Map or Array) during iteration
            if !has_complex && matches!(val, ToonValue::Map(_) | ToonValue::Array(_)) {
                has_complex = true;
            }
            
            // Single pattern match combines list check and key conversion
            let key_str = match k {
                ArrayKey::Long(idx) => {
                    // Check if array is sequential during key extraction
                    if is_list {
                        if idx != expected_idx as i64 {
                            is_list = false;
                        } else {
                            expected_idx += 1;
                        }
                    }
                    idx.to_string()
                }
                ArrayKey::String(s) => {
                    is_list = false;
                    s
                }
                ArrayKey::Str(s) => {
                    is_list = false;
                    s
                }
            };
            entries.push((key_str, val));
        }
        
        // Decision: return Array or Map
        if is_list && !entries.is_empty() && !has_complex {
            // Extract values without cloning (move ownership)
            let items = entries.into_iter().map(|(_, v)| v).collect();
            return Ok(ToonValue::Array(items));
        } else if entries.is_empty() {
            // Empty PHP array defaults to empty list
            return Ok(ToonValue::Array(Vec::new()));
        }
        
        return Ok(ToonValue::Map(entries));
    }
    
    // Fallback
    Ok(ToonValue::String(zval.string().unwrap_or_default()))
}
