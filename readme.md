# idont-notebook

> 一个基于 REPL 的轻量级命令行笔记本工具，支持多仓库管理，使用 Rust 编写。

## 功能特性

- **多仓库管理** — 通过 `initlib` 注册多个独立笔记目录，`selectlib` 切换上下文
- **任意文件类型** — 不限于 `.md`/`.txt`，支持任意文件格式
- **磁盘扫描** — 直接扫描 `.notes/data/` 目录，手动放入的文件自动可见
- **文件隐藏** — `untrack` 隐藏不需要显示的文件，`track` 恢复显示
- **REPL 交互模式** — 基于 `rustyline` 提供友好的命令行交互体验（历史记录、自动补全、动态 prompt）
- **系统默认打开** — `editnote` 使用系统默认程序打开文件（支持 WSL）
- **TOML 元数据** — 每个仓库使用 `notes.toml` 管理隐藏列表
- **跨会话持久化** — 仓库注册信息保存在用户主目录，重启后不丢失
- **安全删除** — `rmnote` 需二次确认，防止误删

---

## 命令一览

| 命令 | 说明 |
|------|------|
| `initlib <path> [name]` | 注册笔记仓库（已存在的 `.notes` 可直接注册） |
| `listlib` | 列出所有已注册仓库（标记当前选中项） |
| `selectlib <编号或名称>` | 选中一个仓库作为操作上下文 |
| `currentlib` | 显示当前选中的仓库 |
| `mknote <filename>` | 在当前选中仓库创建笔记（任意类型） |
| `listnote [-a]` | 列出笔记（`-a` 显示全部含已隐藏文件） |
| `editnote <filename>` | 用系统默认程序打开文件 |
| `catnote <filename> [-n 行数] [-t 行数]` | 打印笔记内容（前 N 行 / 后 N 行），二进制文件提示用 editnote |
| `rmnote <filename>` | 删除笔记（需确认） |
| `track <filename>` | 恢复显示被隐藏的文件 |
| `untrack <filename>` | 隐藏文件（不删除，仅不显示） |
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

---

## 使用指南

### 注册仓库

```text
idontnote> initlib D:/notes/work
# 仓库 "work" 已注册 (#0)，路径: D:/notes/work/.notes

idontnote> initlib D:/notes/personal my-note
# 仓库 "my-note" 已注册 (#1)，路径: D:/notes/personal/.notes
```

如果目录下已有 `.notes`，会直接注册而不会报错。

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

idont(work)> mknote image.png
# mknote: 笔记已创建 -> D:/notes/work/.notes/data/image.png

idont(work)> listnote
# listnote: 共 2 篇笔记
#   - image.png
#   - daily.md

idont(work)> editnote daily.md
# (用系统默认程序打开)

idont(work)> catnote daily.md
# (打印笔记全部内容)

idont(work)> catnote daily.md -n 5
# (打印前 5 行)
```

### 手动管理文件

直接往 `.notes/data/` 目录放入文件，`listnote` 会自动显示：

```text
idont(work)> listnote
# listnote: 共 3 篇笔记
#   - image.png
#   - notes.txt
#   - daily.md

idont(work)> untrack image.png
# untrack: 已隐藏 image.png（文件仍保留在磁盘上）

idont(work)> listnote
# listnote: 共 2 篇笔记
#   - notes.txt
#   - daily.md

idont(work)> listnote -a
# listnote: .notes/data/ 共 3 个文件
#        image.png
#   -    notes.txt
#   -    daily.md

idont(work)> track image.png
# track: 已恢复显示 image.png
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
│   │   ├── models.rs       # 数据结构 (NotebookMeta, TrackingInfo, NotebookEntry, GlobalConfig) + 常量
│   │   ├── config.rs       # GlobalConfig 持久化 (load/save)
│   │   ├── storage.rs      # Storage 结构体 (仓库管理、磁盘扫描、隐藏列表)
│   │   └── error.rs        # 错误类型定义 (thiserror)
│   └── core/               # 业务命令实现
│       ├── initlib.rs      # 仓库初始化/注册
│       ├── listlib.rs      # 列出仓库
│       ├── selectlib.rs    # 选中仓库上下文
│       ├── currentlib.rs   # 显示当前仓库
│       ├── mknote.rs       # 创建笔记
│       ├── listnote.rs     # 列出笔记（磁盘扫描）
│       ├── editnote.rs     # 系统默认程序打开
│       ├── catnote.rs      # 打印笔记内容（支持二进制检测）
│       ├── rmnote.rs       # 删除笔记（含确认提示）
│       ├── track.rs        # 恢复显示被隐藏的文件
│       ├── untrack.rs      # 隐藏文件
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
│   ├── notes.toml                 # 仓库元数据 + 隐藏列表
│   └── data/                      # 笔记实际存储目录（任意类型文件）
│       ├── daily.md
│       ├── image.png
│       └── backup.zip
```

`notes.toml` 结构：

```toml
[notebook]
version = 1
created_at = "2026-05-08T10:00:00+00:00"

[tracking]
hidden = ["backup.zip"]
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
