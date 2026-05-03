# idont-notebook

> 一个基于 REPL 的轻量级命令行笔记本工具，支持多仓库管理，使用 Rust 编写。

## 功能特性

- **多仓库管理** — 通过 `initlib` 注册多个独立的笔记目录，`mknote` 时按需选择
- **REPL 交互模式** — 基于 `rustyline` 提供友好的命令行交互体验（支持历史记录、自动补全）
- **TOML 元数据** — 每个仓库使用 `notes.toml` 管理笔记索引与元信息
- **跨会话持久化** — 仓库注册信息保存在用户主目录，重启后不丢失
- **Markdown / 文本** — 原生支持 `.md` 和 `.txt` 格式

---

## 当前进度

### ✅ 已完成

| 命令 | 状态 | 说明 |
|------|------|------|
| `initlib <path> [name]` | ✅ 可用 | 注册笔记仓库，支持多仓库，可选自定义名称，自动创建 `.notes/notes.toml` 并持久化到全局配置 |
| `mknote <filename>` | ✅ 可用 | 列出所有已注册仓库供用户选择编号 → 在选中仓库创建空文件 → 更新 `notes.toml` 元数据（含时间戳） |
| `listnote` | ✅ 可用 | 从 `notes.toml` 读取并展示笔记列表（含文件名和创建时间） |
| `listlog` | ✅ 可用 | 展示本次会话的命令历史记录 |
| `help` | ✅ 可用 | 显示所有可用命令的帮助文本 |
| `exit` / `quit` | ✅ 可用 | 退出 REPL |
| **Storage 核心层** | ✅ 完成 | 多仓库管理（`NotebookEntry` + `Vec`）、全局配置持久化（`GlobalConfig` → `~/idont-notebook-config.toml`）、完整的错误类型体系 |

### 🔲 待实现

> 以下命令已注册入口、可被解析和分发，但核心逻辑仍为 **TODO 占位**，仅打印提示信息。

| 命令 | 状态 | 缺失功能 |
|------|------|----------|
| `catnote <filename>` | 🔲 占位 | 需实现：读取指定笔记文件内容并输出到终端 |
| `editnote <filename>` | 🔲 占位 | 需实现：调用系统默认编辑器打开笔记文件（如 `edit` / `code` / `vim`） |
| `rmnote <filename>` | 🔲 占位 | 需实现：删除笔记物理文件 + 从 `notes.toml` 中移除对应 `NoteMeta` 条目 |
| `renote <old> <new>` | 🔲 占位 | 需实现：重命名物理文件 + 更新 `notes.toml` 中的 `filename` 字段 |
| `searchnote <keyword>` | 🔲 占位 | 需实现：遍历所有笔记文件内容进行关键词匹配，返回匹配结果（含上下文行） |

### 🧪 未来计划

- [ ] 补齐上述 5 个占位命令的完整实现
- [ ] 多仓库选择通用化：`listnote`/`rmnote`/`catnote` 等操作也支持选择目标仓库
- [ ] 笔记模板：`mknote` 时可选模板自动填充头部元信息
- [ ] 标签/分类：在 `NoteMeta` 中增加 tags 字段
- [ ] 导出功能：将仓库打包为单一文件或导出为 PDF

---

## 快速开始

### 编译

```bash
cargo build --release
```

编译产物位于 `target/release/idontnote`。

### 运行

```bash
cargo run
# 或
./target/release/idontnote
```

进入 REPL 交互环境后即可开始使用。

---

## 使用指南（已完成部分）

### 注册仓库

```text
> initlib D:/notes/work
# 仓库 "work" 已注册 (#0)，路径: D:/notes/work

> initlib D:/notes/personal my-note
# 仓库 "my-note" 已注册 (#1)，路径: D:/notes/personal
```

每个 `initlib <path>` 会在指定目录下创建 `.notes/notes.toml` 元数据文件，并将该仓库注册到全局配置。可选第二个参数为仓库自定义名称，默认取路径的最后一部分。可以多次 `initlib` 注册不同目录作为独立仓库。

### 创建笔记

```text
> mknote daily.md
```

多个仓库时弹出选择：

```text
请选择要创建笔记的仓库:
  [0] work (D:/notes/work/.notes)
  [1] my-note (D:/notes/personal/.notes)
> 请输入编号: 0
# mknote: 笔记已创建 -> D:/notes/work/daily.md
```

仅一个仓库时直接创建，无需选择。

### 查看笔记列表

```text
> listnote
# listnote: 共 2 篇笔记
#   - daily.md (创建于 2026-04-30T22:47:00+08:00)
#   - meeting.md (创建于 2026-04-30T22:48:15+08:00)
```

### 其他可用命令

| 命令 | 说明 |
|------|------|
| `help` | 显示帮助信息 |
| `listlog` | 显示本次会话的命令历史 |
| `exit` / `quit` | 退出 REPL |

---

## 项目结构

```
idont-notebook/
├── Cargo.toml              # 项目配置与依赖
├── src/
│   ├── main.rs             # 程序入口
│   ├── command.rs          # 命令枚举定义 & 输入解析
│   ├── handler.rs          # 命令分发器
│   ├── repl.rs             # REPL 循环 (rustyline)
│   ├── storage/            # 存储层
│   │   ├── mod.rs          # 模块导出
│   │   ├── storage.rs      # Storage 核心（✅ 多仓库、持久化、CRUD 框架）
│   │   └── error.rs        # 错误类型定义
│   └── core/               # 业务命令实现
│       ├── initlib.rs      # ✅ 仓库初始化/注册
│       ├── mknote.rs       # ✅ 创建笔记（含仓库选择）
│       ├── listnote.rs     # ✅ 列出笔记（从 notes.toml）
│       ├── catnote.rs      # 🔲 查看笔记（TODO）
│       ├── editnote.rs     # 🔲 编辑笔记（TODO）
│       ├── rmnote.rs       # 🔲 删除笔记（TODO）
│       ├── renote.rs       # 🔲 重命名笔记（TODO）
│       ├── searchnote.rs   # 🔲 搜索笔记（TODO）
│       ├── listlog.rs      # ✅ 会话历史
│       └── help.rs         # ✅ 帮助信息
```

## 存储机制

```
用户主目录/
└── idont-notebook-config.toml    # 全局配置（已注册仓库列表，跨会话持久化）

<仓库路径>/
├── .notes/
│   └── notes.toml                 # 仓库元数据（版本 + 笔记索引 NoteMeta 列表）
├── note1.md                       # 笔记文件（用户内容）
├── note2.md
└── ...
```

## 技术栈

| 库 | 用途 |
|----|------|
| [rustyline](https://crates.io/crates/rustyline) | REPL 交互（历史记录、补全） |
| [serde](https://crates.io/crates/serde) + [toml](https://crates.io/crates/toml) | TOML 序列化/反序列化 |
| [thiserror](https://crates.io/crates/thiserror) | 错误类型派生 |
| [chrono](https://crates.io/crates/chrono) | 时间戳生成 |
| [dirs](https://crates.io/crates/dirs) | 用户主目录定位 |

## License

MIT
