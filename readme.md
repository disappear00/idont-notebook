# idont-notebook

> 一个基于 REPL 的轻量级命令行笔记本工具，支持多仓库管理，使用 Rust 编写。

## 功能特性

- **多仓库管理** — 通过 `initlib` 注册多个独立笔记目录，`selectlib` 切换上下文
- **REPL 交互模式** — 基于 `rustyline` 提供友好的命令行交互体验（历史记录、自动补全、动态 prompt）
- **TOML 元数据** — 每个仓库使用 `notes.toml` 管理笔记索引与元信息
- **跨会话持久化** — 仓库注册信息保存在用户主目录，重启后不丢失
- **Markdown / 文本** — 原生支持 `.md` 格式
- **安全删除** — `rmnote` 需二次确认，防止误删

---

## 命令一览

| 命令 | 说明 |
|------|------|
| `initlib <path> [name]` | 注册笔记仓库，支持自定义名称 |
| `listlib` | 列出所有已注册仓库（标记当前选中项） |
| `selectlib <编号或名称>` | 选中一个仓库作为操作上下文 |
| `currentlib` | 显示当前选中的仓库 |
| `mknote <filename>` | 在当前选中仓库创建笔记 |
| `listnote` | 列出当前仓库的所有笔记 |
| `editnote <filename>` | 查看/编辑笔记（调用系统编辑器） |
| `catnote <filename> [-n 行数] [-t 行数]` (`ca`) | 打印笔记内容（前 N 行 / 后 N 行） |
| `rmnote <filename>` | 删除笔记（需确认，物理文件 + 元数据） |
| `listlog` | 显示本次会话的命令历史 |
| `help` | 显示帮助 |
| `exit` / `quit` | 退出 REPL |

---

## 安装

### 方式一：npm（推荐）

```bash
npm install -g idont-notebook
```

安装后直接运行：

```bash
idontnote
```

> 支持 Windows / macOS（Intel + Apple Silicon） / Linux（x86_64 + ARM64）

### 方式二：Cargo

```bash
cargo install idont-notebook
```

### 方式三：本地编译

```bash
cargo build --release
```

编译产物位于 `target/release/idontnote`。

### 运行

```bash
idontnote
# 或本地编译后：
# ./target/release/idontnote
```

---

## 使用指南

### 注册仓库

```text
idontnote> initlib D:/notes/work
# 仓库 "work" 已注册 (#0)，路径: D:/notes/work/.notes

idontnote> initlib D:/notes/personal my-note
# 仓库 "my-note" 已注册 (#1)，路径: D:/notes/personal/.notes
```

### 选择仓库

```text
idontnote> listlib
#   [0] work       (D:/notes/work/.notes)
# * [1] my-note    (D:/notes/personal/.notes)

idontnote> selectlib 0
# 已切换到仓库 [0] work
idont(work)> _
```

prompt 会动态显示当前仓库名称。也可以按名称选择：`selectlib my-note`。

### 创建 & 操作笔记

```text
idont(work)> mknote daily.md
# mknote: 笔记已创建 -> D:/notes/work/.notes/data/daily.md

idont(work)> listnote
# listnote: 共 1 篇笔记
#   - daily.md (创建于 2026-05-04T22:30:00+08:00)

idont(work)> editnote daily.md
# (调用系统编辑器打开，可查看和编辑)

idont(work)> catnote daily.md
# (打印笔记全部内容)

idont(work)> catnote daily.md -n 5
# (打印前 5 行)

idont(work)> catnote daily.md -t 3
# (打印后 3 行)

idont(work)> rmnote daily.md
# rmnote: 确定删除笔记 daily.md？[y/N] y
# rmnote: 已删除笔记 daily.md
```

### 切换仓库

```text
idont(work)> selectlib my-note
# 已切换到仓库 [1] my-note
idont(my-note)> listnote
# listnote: 共 0 篇笔记

idont(my-note)> currentlib
# 当前仓库: my-note (#1), 路径: D:/notes/personal/.notes
```

---

## 项目结构

```
idont-notebook/
├── Cargo.toml              # 项目配置与依赖
├── src/
│   ├── main.rs             # 程序入口
│   ├── command.rs          # 命令枚举定义 & 输入解析
│   ├── handler.rs          # 命令分发器
│   ├── repl.rs             # REPL 循环 (rustyline, 动态 prompt)
│   ├── storage/            # 存储层
│   │   ├── mod.rs          # pub re-export
│   │   ├── models.rs       # 数据结构 (NotebookMeta, NoteMeta, NotebookEntry, GlobalConfig) + 常量
│   │   ├── config.rs       # GlobalConfig 持久化 (load/save)
│   │   ├── storage.rs      # Storage 结构体 (仓库管理、笔记 CRUD、选中上下文)
│   │   └── error.rs        # 错误类型定义 (thiserror)
│   └── core/               # 业务命令实现
│       ├── initlib.rs      # 仓库初始化/注册
│       ├── listlib.rs      # 列出仓库
│       ├── selectlib.rs    # 选中仓库上下文
│       ├── currentlib.rs   # 显示当前仓库
│       ├── mknote.rs       # 创建笔记
│       ├── listnote.rs     # 列出笔记
│       ├── editnote.rs     # 查看/编辑笔记（系统编辑器）
│       ├── catnote.rs      # 打印笔记内容（head/tail）
│       ├── rmnote.rs       # 删除笔记（含确认提示）
│       ├── listlog.rs      # 会话历史
│       └── help.rs         # 帮助信息
```

## 存储机制

```
用户主目录/
└── .idont/
    └── idont-notebook-config.toml    # 全局配置（已注册仓库列表）

<仓库路径>/
├── .notes/
│   ├── notes.toml                 # 仓库元数据（版本 + NoteMeta 列表）
│   └── data/                      # 笔记实际存储目录
│       ├── daily.md
│       └── meeting.md
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
