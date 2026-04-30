# idont-notebook

> 一个基于 REPL 的轻量级命令行笔记本工具，支持多仓库管理，使用 Rust 编写。

## 功能特性

- **多仓库管理** — 通过 `initlib` 注册多个独立的笔记目录，`mknote` 时按需选择
- **REPL 交互模式** — 基于 `rustyline` 提供友好的命令行交互体验（支持历史记录、自动补全）
- **TOML 元数据** — 每个仓库使用 `notes.toml` 管理笔记索引与元信息
- **跨会话持久化** — 仓库注册信息保存在用户主目录，重启后不丢失
- **Markdown / 文本** — 原生支持 `.md` 和 `.txt` 格式

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

## 使用指南

### 1. 注册仓库

在使用前，需要先通过 `initlib` 初始化至少一个笔记仓库：

```text
> initlib D:/notes/work
# 仓库 "work" 已注册 (#0)，路径: D:/notes/work

> initlib D:/notes/personal my-note
# 仓库 "my-note" 已注册 (#1)，路径: D:/notes/personal
```

每个 `initlib <path>` 会在指定目录下创建 `.notes/notes.toml` 元数据文件，并将该仓库注册到全局配置。可选第二个参数为仓库自定义名称，默认取路径的最后一部分。

可以多次 `initlib` 注册不同的目录作为独立仓库。

### 2. 创建笔记

```text
> mknote daily.md
```

当存在多个仓库时，会列出所有已注册仓库供选择：

```text
请选择要创建笔记的仓库:
  [0] work (D:/notes/work/.notes)
  [1] my-note (D:/notes/personal/.notes)
> 请输入编号: 0
# mknote: 笔记已创建 -> D:/notes/work/daily.md
```

仅一个仓库时直接在该仓库中创建，无需选择。

### 3. 管理笔记

```text
# 列出所有笔记（含创建时间）
> listnote

# 查看笔记内容
> catnote daily.md

# 用系统默认编辑器打开编辑
> editnote daily.md

# 重命名笔记
> renote daily.md journal.md

# 删除笔记
> rmnote old-note.md

# 搜索笔记内容
> searchnote 关键词
```

### 4. 其他命令

| 命令 | 说明 |
|------|------|
| `help` | 显示帮助信息 |
| `listlog` | 显示本次会话的命令历史 |
| `exit` / `quit` | 退出 REPL |

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
│   │   ├── storage.rs      # Storage 核心逻辑（多仓库管理、CRUD）
│   │   └── error.rs        # 错误类型定义
│   └── core/               # 业务命令实现
│       ├── initlib.rs      # 仓库初始化/注册
│       ├── mknote.rs       # 创建笔记
│       ├── listnote.rs     # 列出笔记
│       ├── catnote.rs      # 查看笔记
│       ├── editnote.rs     # 编辑笔记
│       ├── rmnote.rs       # 删除笔记
│       ├── renote.rs       # 重命名笔记
│       ├── searchnote.rs   # 搜索笔记
│       ├── listlog.rs      # 会话历史
│       └── help.rs         # 帮助信息
```

## 存储机制

```
用户主目录/
└── idont-notebook-config.toml    # 全局配置（已注册仓库列表）

<仓库路径>/
├── .notes/
│   └── notes.toml                 # 仓库元数据（版本 + 笔记索引）
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
