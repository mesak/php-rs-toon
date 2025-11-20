# php-rs-toon 專案完整索引

## 📑 文檔導覽

### 核心文檔

| 文檔名稱 | 路徑 | 內容 | 讀者 |
|---------|------|------|------|
| **README.md** | `README.md` | 項目概述、特性、快速開始 | 所有人 |
| **QUICKSTART.md** | `QUICKSTART.md` | 5分鐘快速指南 | 新手 |
| **USAGE.md** | `USAGE.md` | 詳細使用說明和示例 | 開發者 |
| **API_REFERENCE.md** | `API_REFERENCE.md` (新) | 完整 API 文檔、類型對應、錯誤處理 | 開發者 |
| **CODE_REFERENCE.md** | `CODE_REFERENCE.md` (新) | 源代碼位置、函數流程、修改指南 | 貢獻者 |
| **FILE_STRUCTURE.md** | `FILE_STRUCTURE.md` (新) | 專案結構、檔案內容概覽、統計 | 所有人 |
| **PROJECT_INDEX.md** | `PROJECT_INDEX.md` (新) | 此文檔，完整索引和導航 | 所有人 |
| **TEST_COVERAGE.md** | `TEST_COVERAGE.md` | 測試覆蓋報告 | QA/開發者 |
| **BENCHMARKS.md** | `BENCHMARKS.md` | 性能基準測試 | 優化者 |
| **CLAUDE.md** | `CLAUDE.md` | AI 助手配置 | AI 開發者 |

---

## 🗂️ 檔案樹形結構

```
php-rs-toon/
│
├── 📄 文檔 (10 files)
│   ├── README.md                    # 英文文檔
│   ├── README.zh_TW.md              # 繁體中文文檔
│   ├── QUICKSTART.md                # 快速開始
│   ├── USAGE.md                     # 使用說明
│   ├── API_REFERENCE.md    ✨新      # API 完整參考
│   ├── CODE_REFERENCE.md   ✨新      # 代碼快速參考
│   ├── FILE_STRUCTURE.md   ✨新      # 檔案結構詳解
│   ├── PROJECT_INDEX.md    ✨新      # 此索引
│   ├── TEST_COVERAGE.md             # 測試覆蓋
│   ├── BENCHMARKS.md                # 性能基準
│   └── CLAUDE.md                    # AI 配置
│
├── 🔧 構建配置 (2 files)
│   ├── Cargo.toml                   # Rust 包清單
│   └── Cargo.lock                   # 依賴鎖定
│
├── 🐳 容器化 (1 file)
│   └── Dockerfile                   # 構建環境
│
├── 📝 源代碼 (2 files)
│   └── src/
│       ├── lib.rs       (139 行)    # PHP FFI 橋接
│       └── toon.rs      (567 行)    # 解析器/編碼器
│
├── ✅ 測試 (1 file)
│   └── test.php         (444 行)    # PHP 集成測試
│
├── 📚 示例 (4 files)
│   └── examples/
│       ├── basic-encode.php
│       ├── nested-structures.php
│       ├── llm-optimization.php
│       └── README.md
│
├── 📊 基準測試 (1 directory)
│   └── benchmark/
│       └── benchmarks/
│           └── ToonBench.php
│
└── 🔨 編譯輸出 (1 directory)
    └── target/
        ├── debug/                   # 調試構建
        ├── release/                 # 發布構建
        │   ├── libphp_rs_toon.so   # Linux
        │   └── libphp_rs_toon.dylib # macOS
        └── build/                   # 構建中間文件
```

---

## 🎯 快速導航

### 按任務選擇文檔

#### 🚀 「我想快速開始」
1. **首先閱讀**: `QUICKSTART.md`
2. **然後**: `README.md` (特性部分)
3. **最後**: `examples/` 文件夾

#### 💻 「我想了解如何使用」
1. **首先閱讀**: `USAGE.md`
2. **查看**: `API_REFERENCE.md` (PHP API 部分)
3. **參考**: `examples/` 中的示例

#### 🔧 「我想貢獻代碼」
1. **首先閱讀**: `CODE_REFERENCE.md`
2. **然後**: `FILE_STRUCTURE.md`
3. **參考**: `API_REFERENCE.md` (Rust API 部分)
4. **最後**: 源代碼 `src/lib.rs` 和 `src/toon.rs`

#### 🧪 「我想運行測試」
1. **閱讀**: `TEST_COVERAGE.md`
2. **查看**: `test.php` 結構
3. **執行**: 參考 `QUICKSTART.md` 的測試命令

#### 📈 「我想優化性能」
1. **閱讀**: `BENCHMARKS.md`
2. **查看**: `CODE_REFERENCE.md` (性能優化點)
3. **分析**: `src/toon.rs` 中的性能敏感代碼

#### 🏗️ 「我想了解架構」
1. **首先**: `FILE_STRUCTURE.md`
2. **然後**: `API_REFERENCE.md` (數據結構部分)
3. **最後**: `CODE_REFERENCE.md` (流程圖)

---

## 🔍 按類型查找內容

### PHP 公開 API

| API | 簽名 | 文檔 | 位置 |
|-----|------|------|------|
| `toon_encode()` | `encode(mixed): string` | `API_REFERENCE.md` | lib.rs:17 |
| `toon_decode()` | `decode(string): mixed` | `API_REFERENCE.md` | lib.rs:8 |

### 類型系統

| 類型 | 定義 | 文檔 | 位置 |
|------|------|------|------|
| `ToonValue` | Rust 枚舉 | `API_REFERENCE.md` | toon.rs:3 |
| TOON Format | 語法規範 | `API_REFERENCE.md` | (文檔) |
| 類型對應 | PHP ↔ Rust | `API_REFERENCE.md` | (表格) |

### 函數

| 函數 | 用途 | 文檔 | 位置 |
|------|------|------|------|
| `parse()` | 解析 TOON 字符串 | `API_REFERENCE.md` | toon.rs:16 |
| `encode()` | 編碼 ToonValue | `API_REFERENCE.md` | toon.rs:134 |
| `parse_value()` | 解析單個值 | `CODE_REFERENCE.md` | toon.rs:90 |
| `encode_recursive()` | 遞迴編碼 | `CODE_REFERENCE.md` | toon.rs:140 |
| `zval_to_toon_value()` | PHP → Rust 轉換 | `CODE_REFERENCE.md` | lib.rs:59 |
| `toon_value_to_zval()` | Rust → PHP 轉換 | `CODE_REFERENCE.md` | lib.rs:33 |

### 數據結構

| 結構 | 用途 | 文檔 | 位置 |
|------|------|------|------|
| `Map(Vec<(String, ToonValue)>)` | 有序鍵值對 | `API_REFERENCE.md` | toon.rs:11 |
| `Array(Vec<ToonValue>)` | 有序列表 | `API_REFERENCE.md` | toon.rs:10 |

---

## 🧪 測試導覽

### Rust 單元測試

| 測試類別 | 檔案 | 行數 | 用例 | 文檔 |
|---------|------|------|------|------|
| 基礎解析 | toon.rs | 209-332 | 5 | `CODE_REFERENCE.md` |
| 編碼 | toon.rs | 338-409 | 9 | `CODE_REFERENCE.md` |
| 往返一致性 | toon.rs | 415-452 | 5 | `CODE_REFERENCE.md` |
| 邊界情況 | toon.rs | 458-566 | 10 | `CODE_REFERENCE.md` |

### PHP 集成測試

| 段落 | 行數 | 涵蓋 | 用例 | 文檔 |
|------|------|------|------|------|
| 原始類型 | 102-147 | 基本型別 | 5 | `CODE_REFERENCE.md` |
| 特殊字符 | 150-185 | 轉義, Unicode | 4 | `CODE_REFERENCE.md` |
| 陣列 | 188-214 | 列表結構 | 4 | `CODE_REFERENCE.md` |
| 映射 | 217-265 | 關聯陣列 | 4 | `CODE_REFERENCE.md` |
| 混合結構 | 268-317 | 複雜巢狀 | 3 | `CODE_REFERENCE.md` |
| 往返一致性 | 320-366 | 雙向轉換 | 3 | `CODE_REFERENCE.md` |
| 解碼 | 369-410 | TOON 字符串 | 4 | `CODE_REFERENCE.md` |
| 編碼 | 413-438 | 生成 TOON | 3 | `CODE_REFERENCE.md` |

---

## 📊 數據速查表

### 文件統計

| 類別 | 項目 | 數量 | 行數 |
|------|------|------|------|
| 文檔 | Markdown 文件 | 10 | ~500+ |
| 源代碼 | Rust 代碼 | 2 | 706 |
| 配置 | TOML/Dockerfile | 2 | ~30 |
| 測試 | PHP 測試 | 1 | 444 |
| 示例 | 示例代碼 | 4 | ~200 |

### 代碼統計

| 文件 | 行數 | 函數 | 結構 | 測試 |
|------|------|------|------|------|
| lib.rs | 139 | 5 | 1 enum | 集成於 PHP |
| toon.rs | 567 | 6 + tests | 1 enum | 48+ 測試 |

### 類型統計

| 類型 | 數量 | 示例 |
|------|------|------|
| ToonValue 變體 | 8 | Null, Bool, Int, Float, String, Array, Map |
| PHP 函數 | 2 | toon_encode, toon_decode |
| 內部函數 | 6 | parse, encode, zval_to_toon_value, ... |

---

## 🎓 學習路徑

### 初級 (30 分鐘)
1. 閱讀 `QUICKSTART.md`
2. 查看 `README.md` 的特性部分
3. 運行一個簡單的例子

### 中級 (2 小時)
1. 閱讀完整的 `USAGE.md`
2. 研究 `API_REFERENCE.md` 的 PHP API 部分
3. 查看 `examples/` 中的各種示例
4. 運行 `test.php` 並理解測試結構

### 高級 (1 天)
1. 深入 `CODE_REFERENCE.md`
2. 閱讀 `FILE_STRUCTURE.md` 的詳細內容
3. 研究 `API_REFERENCE.md` 的 Rust API 部分
4. 詳細分析 `src/lib.rs` 和 `src/toon.rs`
5. 執行 `cargo test` 並理解每個測試
6. 修改代碼並編譯驗證

### 專家級 (2 天+)
1. 完全掌握以上所有文檔
2. 研究 `BENCHMARKS.md` 的性能特性
3. 提出優化建議 (參考 `CODE_REFERENCE.md` 的優化點)
4. 開發新功能或修復 bug
5. 貢獻代碼回到主倉庫

---

## 🔗 文檔交叉索引

### API_REFERENCE.md 交叉引用

| 章節 | 相關文檔 | 相關位置 |
|------|---------|---------|
| PHP 公開 API | CODE_REFERENCE.md | 快速查找表 |
| 數據結構 | FILE_STRUCTURE.md | 核心概念 |
| 類型對應 | CODE_REFERENCE.md | 常見修改點 |
| 錯誤處理 | USAGE.md | 錯誤示例 |

### CODE_REFERENCE.md 交叉引用

| 章節 | 相關文檔 | 相關位置 |
|------|---------|---------|
| 檔案位置 | FILE_STRUCTURE.md | 完整結構 |
| 函數調用流程 | API_REFERENCE.md | Rust 內部 API |
| 測試定位 | TEST_COVERAGE.md | 完整報告 |
| 修改指南 | USAGE.md | 示例代碼 |

### FILE_STRUCTURE.md 交叉引用

| 章節 | 相關文檔 | 相關位置 |
|------|---------|---------|
| 檔案清單 | CODE_REFERENCE.md | 快速查找 |
| 常見命令 | QUICKSTART.md | 命令詳解 |
| 統計 | PROJECT_INDEX.md | 本文檔 |

---

## 🔄 文檔更新歷史

| 日期 | 新增文檔 | 修改內容 |
|------|---------|---------|
| 2025-11-20 | FILE_STRUCTURE.md | 初始建立 |
| 2025-11-20 | API_REFERENCE.md | 初始建立 |
| 2025-11-20 | CODE_REFERENCE.md | 初始建立 |
| 2025-11-20 | PROJECT_INDEX.md | 初始建立 |

---

## 💡 常見問題快速定位

| 問題 | 答案位置 |
|------|---------|
| 如何安裝? | QUICKSTART.md, README.md |
| 如何使用? | USAGE.md, API_REFERENCE.md |
| 有哪些示例? | examples/, USAGE.md |
| 如何運行測試? | TEST_COVERAGE.md, CODE_REFERENCE.md |
| 性能如何? | BENCHMARKS.md |
| 如何貢獻? | CODE_REFERENCE.md, 常見修改點 |
| API 參考? | API_REFERENCE.md |
| 源代碼在哪? | CODE_REFERENCE.md, FILE_STRUCTURE.md |
| 如何編譯? | QUICKSTART.md, README.md |
| 支持 PHP 版本? | QUICKSTART.md, README.md |

---

## 🎯 各角色適讀文檔

### 終端用戶
✅ `README.md` → `QUICKSTART.md` → `USAGE.md` → `examples/`

### Web 開發者
✅ `QUICKSTART.md` → `USAGE.md` → `API_REFERENCE.md` (PHP API)

### PHP 擴展開發者
✅ `FILE_STRUCTURE.md` → `CODE_REFERENCE.md` → `API_REFERENCE.md`

### 系統管理員
✅ `README.md` → `QUICKSTART.md` (Docker 部分)

### 性能優化者
✅ `BENCHMARKS.md` → `CODE_REFERENCE.md` (優化點) → 源代碼

### 項目貢獻者
✅ `CODE_REFERENCE.md` → `FILE_STRUCTURE.md` → `API_REFERENCE.md` → 源代碼

### CI/CD 工程師
✅ `Dockerfile` → `.github/workflows/` (如有) → 構建命令

---

## 📞 支持和反饋

- **報告 Bug**: 提交 GitHub Issue
- **功能請求**: GitHub Discussions
- **改進文檔**: 提交 PR 修改 `.md` 文件
- **性能問題**: 參考 `BENCHMARKS.md` 和 `CODE_REFERENCE.md`

---

## 📝 文檔版本

```
php-rs-toon 文檔套件
版本: 1.0
日期: 2025-11-20
包含: 10 個文檔 + 源代碼註釋
維護: Mesak <mesak@example.com>
許可: MIT
```

---

*完整索引 v1.0 - 最後更新: 2025-11-20*
