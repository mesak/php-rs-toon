#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{Zval, ZendHashTable, ArrayKey};
use std::collections::HashMap;
pub mod toon {
    use std::fmt;
    pub enum ToonValue {
        Null,
        Bool(bool),
        Int(i64),
        Float(f64),
        String(String),
        Array(Vec<ToonValue>),
        Map(Vec<(String, ToonValue)>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ToonValue {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ToonValue::Null => ::core::fmt::Formatter::write_str(f, "Null"),
                ToonValue::Bool(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Bool",
                        &__self_0,
                    )
                }
                ToonValue::Int(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Int",
                        &__self_0,
                    )
                }
                ToonValue::Float(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Float",
                        &__self_0,
                    )
                }
                ToonValue::String(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "String",
                        &__self_0,
                    )
                }
                ToonValue::Array(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Array",
                        &__self_0,
                    )
                }
                ToonValue::Map(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Map",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ToonValue {
        #[inline]
        fn clone(&self) -> ToonValue {
            match self {
                ToonValue::Null => ToonValue::Null,
                ToonValue::Bool(__self_0) => {
                    ToonValue::Bool(::core::clone::Clone::clone(__self_0))
                }
                ToonValue::Int(__self_0) => {
                    ToonValue::Int(::core::clone::Clone::clone(__self_0))
                }
                ToonValue::Float(__self_0) => {
                    ToonValue::Float(::core::clone::Clone::clone(__self_0))
                }
                ToonValue::String(__self_0) => {
                    ToonValue::String(::core::clone::Clone::clone(__self_0))
                }
                ToonValue::Array(__self_0) => {
                    ToonValue::Array(::core::clone::Clone::clone(__self_0))
                }
                ToonValue::Map(__self_0) => {
                    ToonValue::Map(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ToonValue {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ToonValue {
        #[inline]
        fn eq(&self, other: &ToonValue) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (ToonValue::Bool(__self_0), ToonValue::Bool(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ToonValue::Int(__self_0), ToonValue::Int(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ToonValue::Float(__self_0), ToonValue::Float(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ToonValue::String(__self_0), ToonValue::String(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ToonValue::Array(__self_0), ToonValue::Array(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ToonValue::Map(__self_0), ToonValue::Map(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => true,
                }
        }
    }
    pub fn parse(input: &str) -> anyhow::Result<ToonValue> {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return Ok(ToonValue::Null);
        }
        if lines.len() == 1 {
            let line = lines[0].trim();
            if !line.contains(':') && !line.is_empty() {
                return Ok(parse_value(line));
            }
        }
        parse_lines(&lines, 0, 0).map(|(val, _)| val)
    }
    fn parse_lines(
        lines: &[&str],
        start_idx: usize,
        base_indent: usize,
    ) -> anyhow::Result<(ToonValue, usize)> {
        let mut map = Vec::new();
        let mut i = start_idx;
        while i < lines.len() {
            let line = lines[i];
            if line.trim().is_empty() {
                i += 1;
                continue;
            }
            let indent = line.len() - line.trim_start().len();
            if indent < base_indent {
                break;
            }
            let trimmed = line.trim();
            if let Some((key_part, val_part)) = trimmed.split_once(':') {
                let key = key_part.trim().to_string();
                let val_str = val_part.trim();
                if val_str.is_empty() {
                    if i + 1 < lines.len() {
                        let next_line = lines[i + 1];
                        if !next_line.trim().is_empty() {
                            let next_indent = next_line.len()
                                - next_line.trim_start().len();
                            if next_indent > indent {
                                let (nested_val, consumed) = parse_lines(
                                    lines,
                                    i + 1,
                                    next_indent,
                                )?;
                                map.push((key, nested_val));
                                i = consumed;
                                continue;
                            }
                        }
                    }
                    map.push((key, ToonValue::Map(Vec::new())));
                } else {
                    map.push((key, parse_value(val_str)));
                }
            } else {}
            i += 1;
        }
        Ok((ToonValue::Map(map), i))
    }
    fn parse_value(s: &str) -> ToonValue {
        if s == "true" {
            return ToonValue::Bool(true);
        }
        if s == "false" {
            return ToonValue::Bool(false);
        }
        if s == "null" {
            return ToonValue::Null;
        }
        if let Ok(i) = s.parse::<i64>() {
            return ToonValue::Int(i);
        }
        if let Ok(f) = s.parse::<f64>() {
            return ToonValue::Float(f);
        }
        ToonValue::String(s.to_string())
    }
    pub fn encode(val: &ToonValue) -> String {
        let mut out = String::new();
        encode_recursive(val, 0, &mut out);
        out.trim_end().to_string()
    }
    fn encode_recursive(val: &ToonValue, indent: usize, out: &mut String) {
        let prefix = " ".repeat(indent);
        match val {
            ToonValue::Map(entries) => {
                for (key, value) in entries {
                    out.push_str(&prefix);
                    out.push_str(key);
                    out.push_str(":");
                    match value {
                        ToonValue::Map(_) => {
                            out.push('\n');
                            encode_recursive(value, indent + 2, out);
                        }
                        _ => {
                            out.push(' ');
                            encode_recursive(value, 0, out);
                        }
                    }
                }
            }
            ToonValue::Array(items) => {
                let s = items
                    .iter()
                    .map(|v| value_to_string(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&s);
                out.push('\n');
            }
            _ => {
                out.push_str(&value_to_string(val));
                out.push('\n');
            }
        }
    }
    fn value_to_string(val: &ToonValue) -> String {
        match val {
            ToonValue::Null => "null".to_string(),
            ToonValue::Bool(b) => b.to_string(),
            ToonValue::Int(i) => i.to_string(),
            ToonValue::Float(f) => f.to_string(),
            ToonValue::String(s) => s.clone(),
            ToonValue::Array(_) => "[Array]".to_string(),
            ToonValue::Map(_) => "[Object]".to_string(),
        }
    }
}
use toon::ToonValue;
pub fn rust_toon_decode(input: String) -> PhpResult<Zval> {
    match toon::parse(&input) {
        Ok(val) => toon_value_to_zval(val),
        Err(e) => Err(PhpException::default(e.to_string())),
    }
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
struct _internal_rust_toon_decode;
impl ::ext_php_rs::internal::function::PhpFunction for _internal_rust_toon_decode {
    const FUNCTION_ENTRY: fn() -> ::ext_php_rs::builders::FunctionBuilder<'static> = {
        fn entry() -> ::ext_php_rs::builders::FunctionBuilder<'static> {
            ::ext_php_rs::builders::FunctionBuilder::new(
                    "toon_decode",
                    {
                        extern "C" fn handler(
                            ex: &mut ::ext_php_rs::zend::ExecuteData,
                            retval: &mut ::ext_php_rs::types::Zval,
                        ) {
                            use ::ext_php_rs::convert::IntoZval;
                            let mut input = ::ext_php_rs::args::Arg::new(
                                "input",
                                <String as ::ext_php_rs::convert::FromZvalMut>::TYPE,
                            );
                            let result = {
                                let parse = ex
                                    .parser()
                                    .arg(&mut input)
                                    .not_required()
                                    .parse();
                                if parse.is_err() {
                                    return;
                                }
                                rust_toon_decode({
                                    match input.val() {
                                        Some(val) => val,
                                        None => {
                                            ::ext_php_rs::exception::PhpException::default(
                                                    "Invalid value given for argument `input`.".into(),
                                                )
                                                .throw()
                                                .expect("Failed to throw PHP exception.");
                                            return;
                                        }
                                    }
                                })
                            };
                            if let Err(e) = result.set_zval(retval, false) {
                                let e: ::ext_php_rs::exception::PhpException = e.into();
                                e.throw().expect("Failed to throw PHP exception.");
                            }
                        }
                        handler
                    },
                )
                .arg(
                    ::ext_php_rs::args::Arg::new(
                        "input",
                        <String as ::ext_php_rs::convert::FromZvalMut>::TYPE,
                    ),
                )
                .not_required()
                .returns(
                    <PhpResult<Zval> as ::ext_php_rs::convert::IntoZval>::TYPE,
                    false,
                    <PhpResult<Zval> as ::ext_php_rs::convert::IntoZval>::NULLABLE,
                )
        }
        entry
    };
}
pub fn rust_toon_encode(input: &Zval) -> PhpResult<String> {
    let val = zval_to_toon_value(input)?;
    Ok(toon::encode(&val))
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
struct _internal_rust_toon_encode;
impl ::ext_php_rs::internal::function::PhpFunction for _internal_rust_toon_encode {
    const FUNCTION_ENTRY: fn() -> ::ext_php_rs::builders::FunctionBuilder<'static> = {
        fn entry() -> ::ext_php_rs::builders::FunctionBuilder<'static> {
            ::ext_php_rs::builders::FunctionBuilder::new(
                    "toon_encode",
                    {
                        extern "C" fn handler(
                            ex: &mut ::ext_php_rs::zend::ExecuteData,
                            retval: &mut ::ext_php_rs::types::Zval,
                        ) {
                            use ::ext_php_rs::convert::IntoZval;
                            let mut input = ::ext_php_rs::args::Arg::new(
                                "input",
                                <&Zval as ::ext_php_rs::convert::FromZvalMut>::TYPE,
                            );
                            let result = {
                                let parse = ex
                                    .parser()
                                    .arg(&mut input)
                                    .not_required()
                                    .parse();
                                if parse.is_err() {
                                    return;
                                }
                                rust_toon_encode({
                                    match input.val() {
                                        Some(val) => val,
                                        None => {
                                            ::ext_php_rs::exception::PhpException::default(
                                                    "Invalid value given for argument `input`.".into(),
                                                )
                                                .throw()
                                                .expect("Failed to throw PHP exception.");
                                            return;
                                        }
                                    }
                                })
                            };
                            if let Err(e) = result.set_zval(retval, false) {
                                let e: ::ext_php_rs::exception::PhpException = e.into();
                                e.throw().expect("Failed to throw PHP exception.");
                            }
                        }
                        handler
                    },
                )
                .arg(
                    ::ext_php_rs::args::Arg::new(
                        "input",
                        <&Zval as ::ext_php_rs::convert::FromZvalMut>::TYPE,
                    ),
                )
                .not_required()
                .returns(
                    <PhpResult<String> as ::ext_php_rs::convert::IntoZval>::TYPE,
                    false,
                    <PhpResult<String> as ::ext_php_rs::convert::IntoZval>::NULLABLE,
                )
        }
        entry
    };
}
#[doc(hidden)]
#[no_mangle]
extern "C" fn get_module() -> *mut ::ext_php_rs::zend::ModuleEntry {
    static __EXT_PHP_RS_MODULE_STARTUP: ::ext_php_rs::internal::ModuleStartupMutex = ::ext_php_rs::internal::MODULE_STARTUP_INIT;
    extern "C" fn ext_php_rs_startup(ty: i32, mod_num: i32) -> i32 {
        let a = 0i32;
        let b = __EXT_PHP_RS_MODULE_STARTUP
            .lock()
            .take()
            .inspect(|_| ::ext_php_rs::internal::ext_php_rs_startup())
            .expect("Module startup function has already been called.")
            .startup(ty, mod_num)
            .map(|_| 0)
            .unwrap_or(1);
        a | b
    }
    #[inline]
    fn internal(module: ModuleBuilder) -> ModuleBuilder {
        module
    }
    let builder = internal(
            ::ext_php_rs::builders::ModuleBuilder::new("php-rs-toon", "0.1.0"),
        )
        .startup_function(ext_php_rs_startup);
    match builder.try_into() {
        Ok((entry, startup)) => {
            __EXT_PHP_RS_MODULE_STARTUP.lock().replace(startup);
            entry.into_raw()
        }
        Err(e) => {
            ::core::panicking::panic_fmt(
                format_args!("Failed to build PHP module: {0:?}", e),
            );
        }
    }
}
#[no_mangle]
pub extern "C" fn ext_php_rs_describe_module() -> ::ext_php_rs::describe::Description {
    use ::ext_php_rs::describe::*;
    #[inline]
    fn internal(module: ModuleBuilder) -> ModuleBuilder {
        module
    }
    let builder = internal(
        ::ext_php_rs::builders::ModuleBuilder::new("php-rs-toon", "0.1.0"),
    );
    Description::new(builder.into())
}
fn toon_value_to_zval(val: ToonValue) -> PhpResult<Zval> {
    let mut zval = Zval::new();
    match val {
        ToonValue::Null => zval.set_null(),
        ToonValue::Bool(b) => zval.set_bool(b),
        ToonValue::Int(i) => zval.set_long(i),
        ToonValue::Float(f) => zval.set_double(f),
        ToonValue::String(s) => zval.set_string(&s, false)?,
        ToonValue::Array(arr) => {
            let mut vec = Vec::new();
            for item in arr {
                vec.push(toon_value_to_zval(item)?);
            }
            zval.set_array(vec).map_err(|e| PhpException::default(e.to_string()))?;
        }
        ToonValue::Map(map) => {
            let mut hash_map = HashMap::new();
            for (k, v) in map {
                hash_map.insert(k, toon_value_to_zval(v)?);
            }
            zval.set_array(hash_map).map_err(|e| PhpException::default(e.to_string()))?;
        }
    }
    Ok(zval)
}
fn zval_to_toon_value(zval: &Zval) -> PhpResult<ToonValue> {
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
        let mut is_list = true;
        let mut expected_idx = 0;
        let mut items = Vec::new();
        let mut entries = Vec::new();
        for (k, v) in ht.iter() {
            let val = zval_to_toon_value(v)?;
            if is_list {
                if let ArrayKey::Long(idx) = k {
                    if idx == expected_idx as i64 {
                        expected_idx += 1;
                        items.push(val.clone());
                    } else {
                        is_list = false;
                    }
                } else {
                    is_list = false;
                }
            }
            let key_str = match k {
                ArrayKey::Long(i) => i.to_string(),
                ArrayKey::String(s) => s.to_string(),
                ArrayKey::Str(s) => s.to_string(),
            };
            entries.push((key_str, val));
        }
        if is_list && !items.is_empty() {
            return Ok(ToonValue::Array(items));
        } else if is_list && items.is_empty() {
            return Ok(ToonValue::Array(Vec::new()));
        }
        return Ok(ToonValue::Map(entries));
    }
    Ok(ToonValue::String(zval.string().unwrap_or_default()))
}
