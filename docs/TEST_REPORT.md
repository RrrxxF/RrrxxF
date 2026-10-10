# S0 验收报告

> S1 的最新结果见文末「S1 验收报告」。

日期：2026-10-10。环境：Codex 云端 Debian 13 x86_64；目标：Windows 10/11 x64。

## 环境审计

| 项目 | 结果 |
| --- | --- |
| Git | 选定 `RrrxxF/RrrxxF`，本地 `work` 分支起初无提交，远端起初无分支；用户确认仓库为 Private |
| Node / pnpm | Node 24.19.0，pnpm 11.25.0 |
| Rust | 云端安装 stable 1.99.0 与 Windows GNU 目标；本机无需安装 |
| 依赖网络 | npm、crates.io、Rust 官方下载可用；GitHub API 在云端返回 403 |
| 本机图形环境 | 无 Windows GUI；Linux 缺少 Tauri 的 GTK/WebKit 系统依赖 |

## 本次实测

| 检查 | 结果 | 证据 |
| --- | --- | --- |
| `pnpm install --frozen-lockfile` | 通过 | 使用仓库锁文件安装 |
| `pnpm lint` / `pnpm typecheck` | 通过 | ESLint 与 TypeScript strict |
| `pnpm test` | 通过 | Vitest：1 文件、1 测试 |
| `pnpm build` | 通过 | Vite 产出静态前端文件；不代表 Windows 安装包 |
| `cargo check --locked --target x86_64-pc-windows-gnu` | 通过 | Rust/Tauri Windows 目标交叉检查；不代表完成链接或打包 |
| `cargo clippy --locked --target x86_64-pc-windows-gnu --all-targets -- -D warnings` | 通过 | Rust 静态检查 |
| `cargo fmt --all -- --check` | 通过 | Rust 格式检查 |
| 浏览器本地冒烟检查 | 通过 | 页面标题、原生服务提示可见；[预览截图](s0-browser-preview.png) |
| GitHub Actions CI | 首次两次运行通过；新提交的运行仍待结果 | 用户提供的 Actions 截图显示 CI #1、#2 绿色成功，CI #6 运行中、#7 排队中 |
| NSIS 安装包与 SHA-256 | 首次 Windows installer #1 成功，压缩包已独立核对 | 用户上传的 Actions ZIP 含 `DesignWorkbench_0.1.0_x64-setup.exe`（1,400,312 字节）与 `SHA256SUMS.txt`；实际 EXE 哈希与清单一致，文件被识别为 Windows NSIS 安装器 |
| Windows 安装与首次启动 | 通过 S0 范围 | 用户提供 Windows 实机窗口截图，显示 `DesignWorkbench 0.1.0`；该版本号由 Rust IPC 返回，表明原生桌面服务已响应 |
| WebView2 缺失路径 | 未测试 | 用户设备已能启动；尚未在缺少 WebView2 的设备上测试安装引导 |

`s0-browser-preview.png` 是浏览器预览截图，不是 Tauri 原生窗口截图。没有把前端构建当作 `.exe`。

## S0 与验收项

S0 工程、依赖锁文件、CI、NSIS 工作流与非程序员安装说明已建立。AC-020（Windows 安装包）已由首次 Windows installer 成功运行、工件上传和下载后的 EXE/哈希核对验证。用户已在 Windows 安装并启动 S0 空壳，原生 Rust IPC 已响应；AC-001 中 WebView2 缺失路径未测试，AC-002 要求的数据库初始化将在 S1 实现后验证。AC-003 至 AC-019、AC-021 至 AC-025 属于 S1–S8，未执行。

## 已知限制与下一步

当前云端无法运行 Windows 安装包，且 GitHub API 返回 403。Git 推送权限已验证：S0 工程提交 `7efd858` 与当前 `main` 已推送至私有仓库，标签 `v0.1.0-s0` 已推送。首个成功工件对应更早的同一 S0 工程代码提交 `0836301`；后续仅调整提交署名、配置脚本和报告。用户已在 Windows 打开该空壳，并提供窗口截图。S1 将实现六个入口、Rust 服务层、SQLite 版本化迁移和项目 CRUD，并验证关闭重启后的持久化。

## Windows 工件核对值

用户从 Actions 下载并上传原始 ZIP。ZIP 的 SHA-256 为 `95b643e3ea53281113ee43b9ce454252ae69b57e79d17f6a9ce5ddc79288c5d1`，与 Actions 页面显示的 Digest 前缀一致。解压得到的 `DesignWorkbench_0.1.0_x64-setup.exe` SHA-256 为 `02a09746fc4e7131e2069c3784435afa994bd6f03bfbaea7c149d4341975fcd6`，与同包 `SHA256SUMS.txt` 完全一致。格式识别为 Windows PE32 Nullsoft Installer 启动程序；NSIS 启动程序为 32 位不代表应用载荷架构。用户提供的 Windows 窗口截图确认 S0 空壳已启动；未单独检查内置应用载荷的 PE 架构。

## 后续验收节奏

每次提交由云端检查和 GitHub Actions CI 自动验证；前端界面可在云端用浏览器测试并保存截图，桌面能力用 Rust 测试和 Windows CI 检查。安装包仅在手动触发或推送版本标签时构建。用户无需每次提交都下载试装，只在需要真实 Windows/Figma 交互判断的里程碑和正式发布前参与实机验收。云端截图不能替代原生桌面验证，报告会分别标注。

---

# S1 验收报告

日期：2026-10-10。环境：Codex 云端 Debian 13 x86_64；Windows GNU 目标交叉检查。版本：0.2.0。

## 完成范围

- 六个一级页面使用独立路由和诚实的空状态；顶部项目切换器、可折叠侧栏与设置页基础已接入。
- 项目新建、列表、改名、选择、归档/恢复和软删除通过 typed Tauri IPC 调用 Rust；本地 SQLite 保存项目与最近选择。浏览器预览不提供假项目数据。
- 数据库位于 Tauri 用户应用数据目录，使用 WAL、事务化版本迁移和可见项目索引。迁移拒绝未来版本，避免旧程序误写新库。
- 原生数据规则位于 `crates/core`，Tauri 命令只负责 IPC。修改了 Windows CI，让其覆盖整个 Cargo workspace，并修正安装包工作流的 workspace 输出路径。

## 实测

| 检查 | 结果 |
| --- | --- |
| `pnpm lint`、`pnpm typecheck`、`pnpm test`、`pnpm build` | 通过；Vitest 1 测试，前端生产构建成功 |
| `cargo test --locked -p design-workbench-core` | 通过；3 个真实 SQLite 集成测试，覆盖 CRUD、持久化、归档与选择、输入错误、迁移重复执行和未来版本保护 |
| `cargo check --locked --target x86_64-pc-windows-gnu -p design-workbench` | 通过；不等于 Windows 安装包已完成链接与运行 |
| `cargo clippy --locked --target x86_64-pc-windows-gnu --workspace --all-targets -- -D warnings` | 通过 |
| `cargo fmt --all -- --check` | 通过 |
| Chromium 1240×800 浏览器检查 | 六个入口可见，`#/icons` 路由与空状态可见；[S1 界面预览](s1-browser-preview.png) |

## AC 状态与边界

- **S1 阶段验收**：代码与自动测试覆盖真实 IPC→SQLite 项目管理、重启后持久化、六个路由和空状态。Windows 安装版首次启动与项目实际点击操作仍须 Windows 构建后验证。
- **AC-002 首次启动**：数据目录初始化与数据库迁移已由 Rust 测试验证；Windows 原生 UI 首启未在本云端执行。
- **AC-019 数据升级**：首次迁移、重复执行、未来版本拒绝已验证；从后续真实旧版本升级与故障回退留待版本出现后验证。
- **AC-020 Windows 构建**：S0 已验证；S1 的 Windows CI/NSIS 将由本阶段提交与里程碑标签触发，结果另行核对。
- **AC-021 中文/UI 缩放**：中文项目名和界面已在测试与浏览器 1240×800 预览验证；Windows 125%/150% 缩放未验证。

当前图标、组件、Token、图片、收集箱只提供 S1 路由和空状态，素材数据操作按 S2–S7 实现。软删除保留数据库记录，但本阶段界面不提供撤销软删除的入口。

---

# S2 验收报告

日期：2026-10-10。环境：Codex 云端 Debian 13 x86_64；Windows GNU 目标静态检查。版本：0.3.0。

## 实现与实测

| 范围 | 代码与证据 |
| --- | --- |
| 持久化 | `0002_assets.sql` 增加 blobs、assets、项目关联、文件夹、标签、收藏；升级前以 SQLite `VACUUM INTO` 保存一致性快照，迁移在事务中执行 |
| 文件导入 | Rust 分块读取并算 SHA-256；暂存、按真实内容检查 SVG/PNG/JPEG/WebP/GIF/BMP、生成 WebP 缩略图；原文件存入 AppData 内容寻址目录，SQLite 提交失败时清理新建 blob |
| 前端 | 图标库、组件库、设计图库可选择文件或拖入；批量计数、取消、搜索、文件夹/标签/收藏筛选、50 条分页、详情页改名及关联项目 |
| 前端检查 | `pnpm lint`、`pnpm typecheck`、`pnpm test`（2 测试）、`pnpm build` 均通过；浏览器测试确认六个路由和本地数据不可用时的真实提示 |
| Rust 集成测试 | `cargo test --locked -p design-workbench-core` 通过：5 项素材测试、3 项项目测试；200 个 4×4 PNG 文件批量导入测试在本云端约 10–12 秒完成（包含文件生成、缩略图、SQLite 操作，不能推算用户设备性能） |
| 失败场景 | 伪装扩展名的损坏文件、取消与模拟磁盘空间不足均拒绝入库且不遗留暂存文件；中文名/中文标签搜索通过；从 S1 库升级保留旧项目并生成快照 |
| 离线依赖检查 | `CARGO_NET_OFFLINE=true` 且 HTTP(S) 代理指向不可用本地端口时，5 项素材测试仍通过；Windows 桌面断网操作仍待实机验收 |
| 界面截图 | [S2 浏览器布局预览](s2-browser-preview.png)：可见导入、搜索、分类与空状态；浏览器无原生文件与 SQLite 权限，因此不是 Windows 桌面实机截图 |

## AC 状态与限制

- **AC-003 / AC-004**：同一 SVG 关联 A/B 项目后移除 A，B 与物理 blob 仍在；重复 PNG 内容只创建一个 blob，测试通过。
- **AC-009 / AC-010**：200 个 PNG 导入、50 条分页、项目/文件夹/标签/收藏的 SQLite 重启持久化测试通过；Windows 实际批量操作与缩放仍待实机验收。
- **AC-019**：从 S1 版本数据库迁移到 S2 并生成迁移前快照，测试通过。
- **AC-020**：S2 Windows 安装包将在 `v0.3.0-s2` 标签构建；云端 GitHub Actions API 凭据无效，不能自行读取私有仓库运行结果。

素材读取与写入均走本地 Rust 服务，不依赖网络；尚未在 Windows 断网状态下手工验收。SVG 原始文件在 S2 仅作为不执行的素材存储，界面不直接渲染原始 SVG；安全清洗与属性编辑属于 S3。取消批量导入后可重新选择文件继续，尚无持久化作业恢复入口；AC-009 因此仍是部分通过。当前软删除无 UI 恢复入口，物理 blob 保留。S1 GNU 交叉目标 `cargo check` 已通过，但 GNU `cargo build` 在 MinGW 链接 Tauri cdylib 时遇到 `export ordinal too large: 92900`；这不是 Windows Actions MSVC 构建结果。
