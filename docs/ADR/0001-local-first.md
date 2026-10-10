# ADR 0001：本地优先桌面架构

采用 Tauri 2 + React + Rust。后续持久化通过 Rust 服务访问 SQLite 和应用数据目录，前端不直接执行 SQL。V1 不依赖云服务器，不默认上传用户资产。Windows NSIS 安装包由 GitHub Actions Windows 运行器构建。
