# S0 验收报告

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
| GitHub Actions CI | 已推送，运行状态无法读取 | GitHub API 对云端返回 403；需在 Actions 页面核对 |
| NSIS 安装包与 SHA-256 | 已推送触发标签，产物状态无法读取 | 标签 `v0.1.0-s0`；需在 Windows installer 的 Artifacts 核对 |
| Windows 安装与 WebView2 | 尚未测试 | 需真实 Windows 设备安装 |

`s0-browser-preview.png` 是浏览器预览截图，不是 Tauri 原生窗口截图。没有把前端构建当作 `.exe`。

## S0 与验收项

S0 工程、依赖锁文件、CI、NSIS 工作流与非程序员安装说明已建立。AC-020（Windows 安装包）仍待 Windows runner 产生真实 `.exe` 和 SHA-256。AC-001（空电脑安装）与 AC-002（首次原生启动）仍待 Windows 实机。AC-003 至 AC-019、AC-021 至 AC-025 属于 S1–S8，未执行。

## 已知限制与下一步

当前云端无法运行 Windows 安装包，且 GitHub API 返回 403。Git 推送权限已验证：S0 提交 `7efd858` 已推送至私有仓库 `main`，标签 `v0.1.0-s0` 已推送。用户需要打开仓库 **Actions** 页面，核对 CI 与 Windows installer 的结果、NSIS `.exe` 和 `SHA256SUMS.txt`；若失败，提供运行链接或错误日志以继续排查。S1 将实现六个入口、Rust 服务层、SQLite 版本化迁移和项目 CRUD，并验证关闭重启后的持久化。
