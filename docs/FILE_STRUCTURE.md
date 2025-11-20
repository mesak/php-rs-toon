# php-rs-toon 專案檔案結構與內容整理

## 📁 專案路徑
```
/home/mesak/plugins/php/php-rs-toon/
```

---

## 📋 檔案清單與內容概覽

### 🔧 核心源代碼

#### 1. `src/lib.rs` (139 行)
**路徑**: `/home/mesak/plugins/php/php-rs-toon/src/lib.rs`

**功能**: PHP FFI 橋接層，連接 PHP 和 Rust
- 導出兩個 PHP 函數：
  - `toon_encode($data)` - 將 PHP 陣列編碼為 TOON 字符串
  - `toon_decode($string)` - 將 TOON 字符串解析為 PHP 陣列

- 核心轉換函數：
  - `toon_value_to_zval()` (行 33-57) - ToonValue → PHP Zval
  - `zval_to_toon_value()` (行 59-139) - PHP Zval → ToonValue

- 類型檢測邏輯：
  - 連續整數鍵 (0, 1, 2...) → TOON 數組 (Arrays)
  - 其他鍵 → TOON 映射 (Maps)

**主要依賴**: `ext-php-rs = "0.15.1"`

---

#### 2. `src/toon.rs` (567 行)
**路徑**: `/home/mesak/plugins/php/php-rs-toon/src/toon.rs`

**核心數據結構** (行 3-12):
```rust
pub enum ToonValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<ToonValue>),              // 有序列表
    Map(Vec<(String, ToonValue)>),      // 有序映射 (保留插入順序)
}
```

**解析器** (行 16-130):
- `parse()` - 主入口，處理 TOON 字符串
- `parse_lines()` - 遞迴解析多行結構 (行 33-88)
- `parse_value()` - 解析單個值 (行 90-130)
  - 支持: null, 布爾值, 整數, 浮點數, 帶引號字符串, 內聯列表

**編碼器** (行 134-199):
- `encode()` - 將 ToonValue 轉換為 TOON 格式
- `encode_recursive()` - 遞迴編碼，管理縮進 (行 140-171)
- `value_to_string()` - 單個值轉換為字符串 (行 173-199)

**測試** (行 201-567):
- SECTION 1: 基礎解析測試 (13 個測試)
- SECTION 2: 編碼測試 (9 個測試)
- SECTION 3: 往返一致性測試 (5 個測試)
- SECTION 4: 邊界情況和特殊字符 (10 個測試)

---

### 📦 配置文件

#### 3. `Cargo.toml` (12 行)
**路徑**: `/home/mesak/plugins/php/php-rs-toon/Cargo.toml`

```toml
[package]
name = "php-rs-toon"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
ext-php-rs = "0.15.1"
anyhow = "1.0"
```

**關鍵配置**:
- `crate-type = ["cdylib"]` - 編譯為共享庫 (.so/.dylib)
- 依賴: ext-php-rs (PHP FFI), anyhow (錯誤處理)

---

### ✅ 測試文件

#### 4. `test.php` (444 行)
**路徑**: `/home/mesak/plugins/php/php-rs-toon/test.php`

**測試運行器類**: `ToonTestRunner` (行 3-94)
- 方法:
  - `test_round_trip()` - 編碼→解碼往返測試
  - `test_encode_decode()` - 解碼測試
  - `test_encode()` - 編碼測試
  - `print_summary()` - 測試摘要

**8 個測試段落**:

| 段落 | 名稱 | 行數 | 測試數 |
|------|------|------|--------|
| 1 | 原始類型與基本類型 | 102-147 | 5 |
| 2 | 特殊字符與轉義 | 150-185 | 4 |
| 3 | 陣列與列表 | 188-214 | 4 |
| 4 | 映射與關聯陣列 | 217-265 | 4 |
| 5 | 混合結構 | 268-317 | 3 |
| 6 | 往返一致性 | 320-366 | 3 |
| 7 | 解碼測試 (TOON 字符串) | 369-410 | 4 |
| 8 | 編碼測試 (生成 TOON) | 413-438 | 3 |

**總計**: 30+ 個測試用例

---

### 📖 文檔

#### 5. `README.md` (157 行)
**路徑**: `/home/mesak/plugins/php/php-rs-toon/README.md`

**內容**:
- 項目描述和特性
- 需求和依賴
- 快速開始指南
- 建構和安裝步驟
- 使用示例
- Docker 驗證
- 項目結構
- 貢獻指南
- 許可

---

#### 6. `README.zh_TW.md` (...)
**路徑**: `/home/mesak/plugins/php/php-rs-toon/README.zh_TW.md`

傳統中文文檔，內容與英文版本對應。

---

#### 7. `USAGE.md`
**路徑**: `/home/mesak/plugins/php/php-rs-toon/USAGE.md`

詳細使用說明：
- 基本編碼/解碼
- 高級用法
- API 參考
- 最佳實踐

---

#### 8. `QUICKSTART.md`
**路徑**: `/home/mesak/plugins/php/php-rs-toon/QUICKSTART.md`

快速入門指南：
- 5 分鐘安裝
- 第一個程序
- 常見問題

---

#### 9. `TEST_COVERAGE.md`
**路徑**: `/home/mesak/plugins/php/php-rs-toon/TEST_COVERAGE.md`

測試覆蓋報告：
- 測試用例分類
- 覆蓋率統計
- 測試計畫

---

#### 10. `BENCHMARKS.md`
**路徑**: `/home/mesak/plugins/php/php-rs-toon/BENCHMARKS.md`

性能基準測試報告：
- 編碼/解碼速度對比
- 記憶體使用
- 優化結果

---

#### 11. `CLAUDE.md`
**路徑**: `/home/mesak/plugins/php/php-rs-toon/CLAUDE.md`

AI 助手指南（本文件）：
- 項目概述
- 建構和測試命令
- 架構說明
- 開發注意事項

---

### 🐳 容器化

#### 12. `Dockerfile`
**路徑**: `/home/mesak/plugins/php/php-rs-toon/Dockerfile`

清潔構建環境：
- PHP 8.2
- Rust 工具鏈
- 編譯依賴
- 測試環境

---

### 🔧 生成文件

#### 13. `expanded.rs` (20308 行)
**路徑**: `/home/mesak/plugins/php/php-rs-toon/expanded.rs`

`#[php_module]` 宏展開生成的文件（自動生成，不需手動編輯）。

---

#### 14. `Cargo.lock`
**路徑**: `/home/mesak/plugins/php/php-rs-toon/Cargo.lock`

依賴鎖定文件，確保可重現構建。

---

### 📂 編譯輸出目錄

```
target/
├── debug/              # 調試構建
│   └── build/          # 構建腳本輸出
├── release/            # 發布構建
│   ├── libphp_rs_toon.so  # Linux 共享庫
│   └── libphp_rs_toon.dylib  # macOS 共享庫
```

---

## 🎯 核心概念

### 數據流

```
PHP 數組 (Zval)
    ↓
zval_to_toon_value() [lib.rs:59-139]
    ↓
ToonValue (Rust 枚舉) [toon.rs:3-12]
    ↓
encode() [toon.rs:134]
    ↓
TOON 格式字符串
```

```
TOON 格式字符串
    ↓
parse() [toon.rs:16-31]
    ↓
ToonValue (Rust 枚舉)
    ↓
toon_value_to_zval() [lib.rs:33-57]
    ↓
PHP 陣列 (Zval)
```

### 類型檢測邏輯

**PHP 陣列 → TOON 類型決策** (lib.rs:78-134)

```
PHP 陣列
├─ 連續整數鍵 (0,1,2...) 且無複雜元素
│  └─ → ToonValue::Array (內聯列表)
└─ 其他
   └─ → ToonValue::Map (多行映射)
```

---

## 🚀 常見命令

### 構建
```bash
cargo build              # 調試構建
cargo build --release   # 優化發布構建
```

### 測試
```bash
cargo test                                              # Rust 單元測試
php -d extension=target/release/libphp_rs_toon.so test.php  # PHP 集成測試
```

### 代碼質量
```bash
cargo fmt               # 代碼格式化
cargo clippy --release # 代碼檢查
```

### Docker
```bash
docker build -t php-rs-toon-debug .
docker run --rm -v $(pwd):/app php-rs-toon-debug bash -c \
  "cargo build --release && php -d extension=target/release/libphp_rs_toon.so test.php"
```

---

## 📊 統計

| 項目 | 數值 |
|------|------|
| 源代碼行數 | ~706 行 (lib.rs + toon.rs) |
| 文檔頁面 | 6 份 MD 文件 |
| 測試用例 | 30+ PHP 測試 + Rust 單元測試 |
| 依賴項 | 2 個主要 crate |
| PHP 函數 | 2 個公共函數 |
| ToonValue 類型 | 8 種變體 |

---

## 📝 版本信息

- **當前版本**: 1.0.0
- **PHP 版本**: 8.0+
- **Rust 版本**: 2021 edition
- **編譯類型**: cdylib (共享庫)
- **許可**: MIT

---

*此文檔生成於 2025-11-20*
