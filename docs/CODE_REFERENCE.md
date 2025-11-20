# php-rs-toon 代碼快速參考

## 📍 檔案位置速查表

### Rust 源代碼

```
src/
├── lib.rs (139 行)
│   ├── php_function: toon_decode (行 8-15)
│   ├── php_function: toon_encode (行 17-22)
│   ├── php_module: get_module (行 24-29)
│   ├── fn toon_value_to_zval (行 33-57)
│   └── fn zval_to_toon_value (行 59-139)
│       └── 類型檢測邏輯 (行 78-134)
│
└── toon.rs (567 行)
    ├── enum ToonValue (行 3-12)
    ├── parser section
    │   ├── pub fn parse (行 16-31)
    │   ├── fn parse_lines (行 33-88)
    │   └── fn parse_value (行 90-130)
    ├── encoder section
    │   ├── pub fn encode (行 134-138)
    │   ├── fn encode_recursive (行 140-171)
    │   └── fn value_to_string (行 173-199)
    └── #[cfg(test)] tests (行 201-567)
        ├── SECTION 1: 基礎解析 (行 209-332)
        ├── SECTION 2: 編碼 (行 338-409)
        ├── SECTION 3: 往返一致性 (行 415-452)
        └── SECTION 4: 邊界情況 (行 458-566)
```

---

## 🔍 快速查找

### 按功能查找代碼位置

| 功能 | 檔案 | 行數 | 函數名 |
|------|------|------|--------|
| PHP 公開函數 | lib.rs | 8-22 | `rust_toon_decode`, `rust_toon_encode` |
| Zval → ToonValue | lib.rs | 59-139 | `zval_to_toon_value()` |
| ToonValue → Zval | lib.rs | 33-57 | `toon_value_to_zval()` |
| 類型檢測邏輯 | lib.rs | 78-134 | `zval_to_toon_value()` 中的邏輯 |
| TOON 字符串解析 | toon.rs | 16-31 | `parse()` |
| 多行解析 | toon.rs | 33-88 | `parse_lines()` |
| 單值解析 | toon.rs | 90-130 | `parse_value()` |
| 編碼主函數 | toon.rs | 134-138 | `encode()` |
| 遞迴編碼 | toon.rs | 140-171 | `encode_recursive()` |
| 值轉字符串 | toon.rs | 173-199 | `value_to_string()` |
| 字符串轉義 | toon.rs | 185-188 | `value_to_string()` 中的邏輯 |
| 單元測試 | toon.rs | 201-567 | `#[cfg(test)] mod tests` |

---

## 📊 函數調用流程圖

### 編碼流程

```
toon_encode() [lib.rs:17]
  │
  ├─ zval_to_toon_value() [lib.rs:59]
  │   ├─ 檢查 Zval 類型
  │   ├─ 遞迴轉換嵌套值
  │   └─ 決定 Array 還是 Map
  │
  └─ encode() [toon.rs:134]
      └─ encode_recursive() [toon.rs:140]
          └─ value_to_string() [toon.rs:173]
              └─ 返回 TOON 字符串
```

### 解碼流程

```
toon_decode() [lib.rs:8]
  │
  ├─ parse() [toon.rs:16]
  │   ├─ 按行分割
  │   └─ parse_lines() [toon.rs:33]
  │       └─ parse_value() [toon.rs:90]
  │           └─ 返回 ToonValue
  │
  └─ toon_value_to_zval() [lib.rs:33]
      ├─ 遞迴轉換 ToonValue
      └─ 返回 Zval (PHP 值)
```

---

## 🧪 測試用例快速定位

### 按類別查找測試

| 測試類別 | 檔案 | 行數範圍 | 函數 | 用例數 |
|---------|------|---------|------|--------|
| 基礎解析 | toon.rs | 209-332 | test_decode_simple 等 | 5 |
| 編碼測試 | toon.rs | 338-409 | test_encode_simple 等 | 9 |
| 往返一致性 | toon.rs | 415-452 | test_roundtrip_* | 5 |
| 邊界情況 | toon.rs | 458-566 | test_empty_string_key 等 | 10 |

### PHP 集成測試

| 段落 | test.php | 行數 | 測試數 | 涵蓋 |
|------|----------|------|--------|------|
| 1 | 原始類型 | 102-147 | 5 | null, bool, int, float, string |
| 2 | 特殊字符 | 150-185 | 4 | 轉義, 換行, Unicode, 符號 |
| 3 | 陣列 | 188-214 | 4 | 序列, 空, 嵌套, 大陣列 |
| 4 | 映射 | 217-265 | 4 | 簡單, 嵌套, 深層, 多鍵 |
| 5 | 混合結構 | 268-317 | 3 | 複雜對象, 列表中的映射 |
| 6 | 往返一致性 | 320-366 | 3 | 雙向轉換, 多次循環 |
| 7 | 解碼 | 369-410 | 4 | TOON 字符串解析 |
| 8 | 編碼 | 413-438 | 3 | 生成 TOON 字符串 |

---

## 💻 核心代碼片段

### 1. 類型檢測邏輯

**位置**: `src/lib.rs:78-134`

**目標**: 判斷 PHP 陣列是轉換為 TOON Array 還是 Map

```rust
// 行 90-118: 掃描階段
let mut is_list = true;
let mut expected_idx = 0;

for (k, v) in ht.iter() {
    if is_list {
        if let ArrayKey::Long(idx) = k {
            if idx == expected_idx as i64 {
                expected_idx += 1;
                items.push(val.clone());
            } else {
                is_list = false;
            }
        } else {
            is_list = false;  // 非整數鍵
        }
    }
}

// 行 120-134: 決策邏輯
if is_list && !items.is_empty() {
    // 檢查複雜元素
    let has_complex = items.iter()
        .any(|v| matches!(v, ToonValue::Map(_) | ToonValue::Array(_)));
    if !has_complex {
        return Ok(ToonValue::Array(items));
    }
}
return Ok(ToonValue::Map(entries));
```

---

### 2. TOON 值解析

**位置**: `src/toon.rs:90-130`

**目標**: 解析單個 TOON 值

```rust
pub fn parse_value(s: &str) -> ToonValue {
    let s = s.trim();

    // 特殊字符串
    if s == "true" { return ToonValue::Bool(true); }
    if s == "false" { return ToonValue::Bool(false); }
    if s == "null" { return ToonValue::Null; }

    // 數字
    if let Ok(i) = s.parse::<i64>() { return ToonValue::Int(i); }
    if let Ok(f) = s.parse::<f64>() { return ToonValue::Float(f); }

    // 帶引號字符串 (行 109-115)
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        let inner = &s[1..s.len()-1];
        let unescaped = inner
            .replace("\\\"", "\"")
            .replace("\\n", "\n")
            .replace("\\\\", "\\");
        return ToonValue::String(unescaped);
    }

    // 內聯列表 (行 120-127)
    if s.contains(',') {
        let parts: Vec<&str> = s.split(',')
            .map(|p| p.trim())
            .collect();
        let items: Vec<ToonValue> = parts
            .iter()
            .map(|&p| parse_value(p))
            .collect();
        return ToonValue::Array(items);
    }

    ToonValue::String(s.to_string())
}
```

---

### 3. 字符串轉義規則

**位置**: `src/toon.rs:173-199` (`value_to_string()`)

**目標**: 決定字符串是否需要引號和如何轉義

```rust
match val {
    ToonValue::String(s) => {
        // 空字符串 → 總是引號
        if s.is_empty() {
            return "\"\"".to_string();
        }

        // 檢查是否需要引號 (行 185)
        if s.contains('\n')
            || s.contains(':')
            || s.contains(',')
            || s.contains('"')
            || s.trim() != s {
            // 轉義 (行 187)
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n");
            format!("\"{}\"", escaped)
        } else {
            s.clone()
        }
    }
    // ...
}
```

---

### 4. 遞迴編碼邏輯

**位置**: `src/toon.rs:140-171`

**目標**: 處理嵌套結構的縮進

```rust
fn encode_recursive(val: &ToonValue, indent: usize, out: &mut String) {
    let prefix = " ".repeat(indent);

    match val {
        ToonValue::Map(entries) => {
            for (key, value) in entries {
                out.push_str(&prefix);
                out.push_str(key);

                match value {
                    ToonValue::Map(_) => {
                        // 嵌套 Map: 換行 + 增加縮進
                        out.push_str(":\n");
                        encode_recursive(value, indent + 2, out);
                    }
                    _ => {
                        // 標量或陣列: 內聯
                        out.push_str(": ");
                        encode_recursive(value, 0, out);
                    }
                }
            }
        }
        // Arrays 用逗號分隔
        ToonValue::Array(items) => {
            let s = items
                .iter()
                .map(|v| value_to_string(v))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&s);
            out.push('\n');
        }
        // ...
    }
}
```

---

## 🎯 常見修改點

### 添加新的原始類型

1. **ToonValue 枚舉** (toon.rs:3)
   ```rust
   pub enum ToonValue {
       // ... 新類型
       NewType(NewRustType),
   }
   ```

2. **parse_value()** (toon.rs:90)
   ```rust
   if /* 檢測條件 */ {
       return ToonValue::NewType(value);
   }
   ```

3. **value_to_string()** (toon.rs:173)
   ```rust
   ToonValue::NewType(val) => /* 格式化邏輯 */
   ```

4. **encode_recursive()** (toon.rs:140)
   ```rust
   ToonValue::NewType(val) => {
       // 編碼邏輯
   }
   ```

5. **zval_to_toon_value()** (lib.rs:59)
   ```rust
   if zval.is_newtype() {
       return Ok(ToonValue::NewType(
           zval.newtype().unwrap_or_default()
       ));
   }
   ```

6. **toon_value_to_zval()** (lib.rs:33)
   ```rust
   ToonValue::NewType(val) => {
       zval.set_newtype(val)?;
   }
   ```

7. **測試** (toon.rs:201)
   - 在 `#[cfg(test)]` 模塊中添加測試用例

---

### 修改解析邏輯

**檔案**: `src/toon.rs`

1. **改變值解析優先級** → 修改 `parse_value()` 的檢查順序 (行 90-130)

2. **添加新的引號字符** → 修改 `parse_value()` (行 109-115)

3. **改變轉義規則** → 修改 `value_to_string()` (行 187-188)

4. **改變縮進策略** → 修改 `encode_recursive()` 的縮進參數 (行 152, 156)

---

### 修改類型檢測

**檔案**: `src/lib.rs:78-134`

1. **改變 Array 檢測標準** → 修改 `is_list` 邏輯 (行 90-109)

2. **改變複雜元素定義** → 修改 `has_complex` 檢查 (行 121-122)

3. **改變空陣列處理** → 修改行 127-131

---

## 📈 性能優化點

### 可以優化的地方

| 位置 | 現狀 | 優化建議 |
|------|------|---------|
| parse_lines | 遞迴 | 使用堆棧替換遞迴 |
| encode_recursive | 遞迴 + 字符串連接 | 使用 StringBuilder |
| parse_value | 多次 parse 嘗試 | 提前檢查類型標記 |
| zval_to_toon_value | Vec 複製 | 使用引用或 Cow |
| 字符串轉義 | 多次 replace 調用 | 一次掃描完成 |

---

## 🔐 安全檢查點

### 需要檢查的地方

| 位置 | 檢查項 | 位置 |
|------|--------|------|
| parse | 堆棧深度限制 | toon.rs:33 |
| encode | 輸出大小限制 | toon.rs:140 |
| zval_to_toon_value | 無限遞迴 | lib.rs:96 |
| toon_value_to_zval | 無限遞迴 | lib.rs:44 |
| 字符串 | 超大字符串 | lib.rs:76 |

---

## 🧪 測試執行速查表

### 運行特定測試

```bash
# Rust 單元測試
cargo test                              # 全部
cargo test test_decode                  # 解碼測試
cargo test test_encode                  # 編碼測試
cargo test test_roundtrip               # 往返測試

# PHP 集成測試
php -d extension=target/release/libphp_rs_toon.so test.php

# Docker 完整測試
docker build -t php-rs-toon .
docker run --rm -v $(pwd):/app php-rs-toon \
  bash -c "cargo build --release && \
           cargo test && \
           php -d extension=target/release/libphp_rs_toon.so test.php"
```

---

*代碼快速參考 v1.0 - 2025-11-20*
