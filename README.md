# DesignWorkbench

Windows 桌面设计资产工作台。当前为 S0 基础工程，业务功能将在 S1–S8 实现。完整需求见 [开发手册](docs/DesignWorkbench_完整开发需求与Codex执行手册_v1.0.md)。

技术栈：Tauri 2、React、TypeScript、Rust。用户素材默认保存在本机；S0 不读取或上传素材。

## 开发

云端环境安装依赖：`pnpm install --frozen-lockfile`。运行 `pnpm lint`、`pnpm typecheck`、`pnpm test`、`pnpm build`。有 Windows GUI 的开发机可运行 `pnpm tauri dev`。CI 在 Windows 运行 Rust 检查；安装包由手动触发的 Windows installer 工作流产生。

普通用户无需安装开发工具。安装说明见 [非程序员安装指南](docs/SETUP_FOR_NON_DEVELOPER.md)。

日常提交只触发 CI；`Windows installer` 仅在手动触发或推送版本标签时构建。用户不必每次下载试装，真实 Windows/Figma 交互和发布前再做里程碑验收。
