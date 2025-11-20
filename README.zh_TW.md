# PHP TOON 擴展 (Rust)

**語言**: [English](README.md) | [繁體中文](#)

---

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-2021-orange?style=flat-square)](https://www.rust-lang.org/)
[![PHP](https://img.shields.io/badge/PHP-8.0%2B-777BB4?style=flat-square)](https://www.php.net/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)
[![Language: English | 繁體中文](https://img.shields.io/badge/Language-English%20%7C%20%E7%B9%81%E9%AB%94%E4%B8%AD%E6%96%87-blue?style=flat-square)](#languages)

一個超高效能的 PHP 擴展，用於編碼和解碼 [TOON (Token-Oriented Object Notation)](https://github.com/HelgeSverre/toon-php) 格式。採用 Rust 打造，擁有最佳的效能和安全性。

[English](README.md) • [繁體中文](#繁體中文)

</div>

---

## 繁體中文

### ✨ 功能特色

- **⚡ 雷電般的速度** – 使用 Rust 精心打造，提供無與倫比的性能和安全性
- **🔄 完整雙向支持** – `toon_encode()` 和 `toon_decode()` 實現無縫轉換
- **🎯 智慧型類型偵測** – 自動區分序列陣列和關聯式陣列
- **📍 順序保留** – 保持關聯式陣列的插入順序 (PHP 7.1+ 原生陣列行為)
- **🔐 型態安全** – 記憶體安全，關鍵路徑中零不安全程式碼

### 📋 系統需求

- **Rust** – 最新穩定版
- **PHP** – 8.0 或更高版本
- **php-config** – 包含在 `php-dev` 或 `php-devel` 套件中
- **Clang** – 用於 `bindgen`

### 🚀 快速開始

#### 建置

```bash
# 複製並進入目錄
git clone <repository_url>
cd php-rs-toon

# 建置最佳化版本
cargo build --release
```

輸出: `target/release/libphp_rs_toon.so` (Linux) 或 `target/release/libphp_rs_toon.dylib` (macOS)

#### 安裝

```bash
# 找到 PHP 擴展目錄
php-config --extension-dir

# 複製已建置的擴展 (Linux 範例)
cp target/release/libphp_rs_toon.so $(php-config --extension-dir)/

# 在 php.ini 中啟用
echo "extension=libphp_rs_toon.so" >> /etc/php/8.2/cli/php.ini
```

### 💡 使用範例

#### 基本編碼

```php
<?php

$data = [
    "user" => [
        "id" => 123,
        "email" => "ada@example.com",
        "metadata" => [
            "active" => true,
            "score" => 9.5
        ]
    ]
];

$toonString = toon_encode($data);
echo $toonString;
```

**輸出:**
```
user:
  id: 123
  email: ada@example.com
  metadata:
    active: true
    score: 9.5
```

#### 基本解碼

```php
<?php

$toonString = <<<'TOON'
user:
  id: 123
  email: ada@example.com
TOON;

$array = toon_decode($toonString);
var_dump($array);
```

### 🐳 Docker 驗證

在隔離環境中測試擴展：

```bash
# 建置容器
docker build -t php-rs-toon-test .

# 執行測試
docker run --rm -v $(pwd):/app php-rs-toon-test \
  bash -c "cargo build --release && php -d extension=target/release/libphp_rs_toon.so test.php"
```

### 📚 專案結構

```
php-rs-toon/
├── Cargo.toml              # Rust 套件清單
├── Cargo.lock              # 可重現建置
├── Dockerfile              # 乾淨的建置環境
├── README.md               # 英文文檔
├── README.zh_TW.md         # 繁體中文文檔 (此檔案)
├── test.php                # 整合測試套件
├── expanded.rs             # 生成的巨集展開
└── src/
    ├── lib.rs              # PHP FFI 綁定
    └── toon.rs             # 解析器和編碼器
```

### 🏗️ 架構概覽

#### 核心元件

**src/lib.rs** – PHP FFI 橋接
- 匯出兩個函數: `toon_encode(Zval)` 和 `toon_decode(String)`
- 處理 PHP 陣列 (Zval) 和內部 ToonValue 表示之間的型別轉換
- 偵測邏輯: 序列整數鍵 → TOON 陣列；否則 → TOON 映射

**src/toon.rs** – TOON 解析器和編碼器
- 核心 `ToonValue` 列舉: Null, Bool, Int, Float, String, Array, Map
- `parse()` 函數: 類 YAML 的縮排語法，處理帶逃脫的引用字符串
- `encode()` 函數: 將 ToonValue 轉換為具有適當縮排的 TOON 格式

#### 資料格式

TOON 是一個類似 YAML 的縮排表示法：
- 純量: `key: value` 或內聯 (用於基本型別)
- 映射 (關聯式陣列): 鍵值對加縮排
- 陣列 (序列): 逗號分隔內聯格式或作為清單項
- 支援: null, 布林值, 整數, 浮點數, 帶逃脫的引用字符串

### 🔧 開發指南

#### 修改 TOON 解析邏輯

1. 編輯 `src/toon.rs` 中的 `parse()` 函數
2. 在 `toon.rs` 的 `#[cfg(test)]` 模組中新增對應的測試
3. 執行 `cargo test` 驗證

#### 新增 PHP 測試案例

1. 編輯 `test.php` 並新增測試案例
2. 執行: `php -d extension=target/release/libphp_rs_toon.so test.php`

#### 測試模式

- **Rust 單元測試**: 在 `toon.rs` 中使用 `#[test]`
- **整合測試**: 將 PHP 測試案例新增到 `test.php`
- 始終測試往返: 編碼 → 解碼 (或反之) 以確保一致性

### 📝 常見開發任務

#### 為生產環境建置

```bash
# 建置最佳化版本
cargo build --release

# 複製 .so/.dylib 到 PHP 擴展目錄
cp target/release/libphp_rs_toon.so $(php-config --extension-dir)/

# 在 php.ini 中新增
extension=libphp_rs_toon.so

# 驗證安裝
php -m | grep php_rs_toon
```

#### 代碼品質檢查

```bash
cargo fmt                    # 程式碼格式化
cargo clippy --release       # 檢查潛在問題
cargo test                   # 執行所有測試
```

### 🤝 貢獻指南

歡迎貢獻！請確保：

- 代碼遵循 Rust 慣例 (`cargo fmt`, `cargo clippy`)
- 測試通過 (`cargo test`)
- PHP 整合測試正常運作

### 📄 許可証

MIT – 詳見 [LICENSE](LICENSE)

---

## 語言

- **[English](README.md)** – 英文文檔
- **繁體中文** – 此檔案
