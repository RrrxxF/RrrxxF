# DesignWorkbench 安装说明

当前 S0 安装包尚待首次 Windows CI 验证。不要把前端预览当成桌面安装包。

1. 登录 GitHub，打开授权的私有仓库 `RrrxxF/RrrxxF`。
2. 点击 **Actions** → **Windows installer** → 选择一次成功运行。
3. 在页面底部 **Artifacts** 下载 `DesignWorkbench-...-windows-nsis`，解压后查看 `.exe` 和 `SHA256SUMS.txt`。
4. 核对文件名与 SHA-256 后，双击 `.exe` 安装。普通使用不需要 Node.js、Rust 或 Python。
5. 若安装器提示缺少 WebView2，请按微软安装引导处理。未签名的首版可能出现 Windows“未知发布者”提示；应先核对来源和校验值，不要关闭系统安全功能。
6. 若失败，请记录安装包文件名、GitHub Actions 运行编号、Windows 版本及错误截图。不要上传私人素材。

S0 仅提供桌面空壳；项目和素材功能从 S1 起开发。
