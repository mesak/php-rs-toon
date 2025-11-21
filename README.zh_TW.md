# PHP TOON 擴展 (Rust)

**語言**: [English](README.md) | [繁體中文](#)

---

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-2021-orange?style=flat-square)](https://www.rust-lang.org/)
[![PHP](https://img.shields.io/badge/PHP-8.0%2B-777BB4?style=flat-square)](https://www.php.net/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

超高效能 PHP 擴展，用於編碼和解碼 [TOON (Token-Oriented Object Notation)](https://github.com/HelgeSverre/toon-php) 格式。採用 Rust 打造，提供最佳性能和安全性。

</div>

---

## ✨ 功能特色

- **⚡ 極速性能** – Rust 驅動，無與倫比的速度
- **🔄 雙向支援** – `toon_encode()` 和 `toon_decode()`
- **🎯 智慧型別偵測** – 自動區分陣列與關聯式陣列
- **📍 順序保留** – 保持插入順序
- **🔐 型別安全** – 記憶體安全，零不安全程式碼

---

## 📦 安裝

### 系統需求

```bash
# 安裝 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安裝 PHP 開發標頭檔
sudo apt install php8.2-dev clang  # Ubuntu/Debian
# 或
brew install php clang              # macOS
```

### 建置與安裝

```bash
# 複製專案
git clone https://github.com/mesak/php-rs-toon.git
cd php-rs-toon

# 建置正式版本
cargo build --release

# 安裝擴展
sudo cp target/release/libphp_rs_toon.so $(php-config --extension-dir)/

# 啟用擴展
echo "extension=libphp_rs_toon.so" | sudo tee -a $(php-config --ini-path)/20-toon.ini

# 驗證安裝
php -m | grep php_rs_toon
```

---

## 🔧 開發

### 開發建置

```bash
# 除錯版本（編譯較快）
cargo build

# 正式版本（最佳化）
cargo build --release

# 格式化程式碼
cargo fmt

# 檢查程式碼品質
cargo clippy --release
```

### Docker 建置

```bash
# 建置測試環境
docker build -f Dockerfile.test -t php-rs-toon:test .

# 建置正式版本
docker build -f Dockerfile.prod -t php-rs-toon:prod .
```

---

## 🧪 測試

### 執行 Rust 單元測試

```bash
cargo test
```

### 執行 PHP 整合測試

```bash
# 使用已安裝的擴展
php test.php

# 使用建置的擴展（不需安裝）
php -d extension=target/release/libphp_rs_toon.so test.php

# Docker 測試
docker build -f Dockerfile.test -t php-rs-toon:test .
docker run --rm php-rs-toon:test
```

---

## ⚡ 性能測試

### 快速基準測試

```bash
# 單一性能測試
php -d extension=target/release/libphp_rs_toon.so perf-test.php

# 與純 PHP 實作比較
php -d extension=target/release/libphp_rs_toon.so perf-compare.php
```

### 完整基準測試套件

```bash
cd benchmark
composer install
./run-benchmarks.sh

# Docker 基準測試
docker build -f Dockerfile.benchmark -t php-rs-toon:bench .
docker run --rm php-rs-toon:bench
```

**性能結果**：
- **快 10-30 倍** 相較於純 PHP 實作
- **最佳化記憶體使用** 採用預分配策略
- **遞迴深度保護** (最大深度: 100)

---

## 💡 使用範例

### 基本編碼

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

$toon = toon_encode($data);
echo $toon;
```

**輸出：**
```
user:
  id: 123
  email: ada@example.com
  metadata:
    active: true
    score: 9.5
```

### 基本解碼

```php
<?php

$toon = <<<'TOON'
user:
  id: 123
  name: Alice
  tags: 1, 2, 3
TOON;

$data = toon_decode($toon);
print_r($data);
```

**輸出：**
```
Array
(
    [user] => Array
        (
            [id] => 123
            [name] => Alice
            [tags] => Array
                (
                    [0] => 1
                    [1] => 2
                    [2] => 3
                )
        )
)
```

### 巢狀結構

```php
<?php

$data = [
    "company" => [
        "name" => "TechCorp",
        "departments" => [
            ["name" => "Engineering", "employees" => 50],
            ["name" => "Sales", "employees" => 30],
        ],
        "metadata" => [
            "founded" => 2020,
            "public" => false
        ]
    ]
];

$toon = toon_encode($data);
$decoded = toon_decode($toon);

assert($data === $decoded); // 往返一致性
```

### 錯誤處理

```php
<?php

try {
    $result = toon_decode("invalid: : syntax");
} catch (Exception $e) {
    echo "解析錯誤: " . $e->getMessage();
}
```

### 更多範例

參見 [`examples/`](examples/) 目錄：
- [`basic-encode.php`](examples/basic-encode.php) - 簡單編碼
- [`nested-structures.php`](examples/nested-structures.php) - 複雜巢狀資料
- [`llm-optimization.php`](examples/llm-optimization.php) - LLM 友善格式

---

## 📚 API 參考

### `toon_encode(mixed $data): string`

將 PHP 資料編碼為 TOON 格式字串。

**參數：**
- `$data` - PHP 值（陣列、字串、整數、浮點數、布林值、null）

**回傳：** TOON 格式字串

**例外：** 超過遞迴深度限制 (>100) 時拋出例外

---

### `toon_decode(string $toon): mixed`

將 TOON 字串解碼為 PHP 資料。

**參數：**
- `$toon` - TOON 格式字串

**回傳：** PHP 值（陣列、字串、整數、浮點數、布林值、null）

**例外：** 解析錯誤時拋出例外

---

## 🏗️ 專案結構

```
php-rs-toon/
├── src/
│   ├── lib.rs              # PHP FFI 橋接
│   └── toon.rs             # TOON 解析器與編碼器
├── examples/               # 使用範例
├── benchmark/              # 性能基準測試
├── test.php                # 整合測試
├── perf-test.php           # 快速性能測試
├── perf-compare.php        # Rust vs PHP 比較
├── Cargo.toml              # Rust 依賴
└── Dockerfile.*            # Docker 配置
```

---

## 🤝 貢獻

歡迎貢獻！請：

1. Fork 專案
2. 建立功能分支
3. 進行變更並加入測試
4. 執行 `cargo fmt && cargo clippy && cargo test`
5. 提交 Pull Request

---

## 📄 授權

MIT 授權 - 詳見 [LICENSE](LICENSE)

---

## 🔗 資源

- [TOON 格式規範](https://github.com/HelgeSverre/toon-php)
- [Rust ext-php-rs 文檔](https://docs.rs/ext-php-rs/)
- [PHP 擴展開發](https://www.php.net/manual/zh/internals2.php)

---

**語言**: [English](README.md) | [繁體中文](#)
