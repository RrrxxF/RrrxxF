# DesignWorkbench — 个人 UI 设计资产工作台

**产品需求文档（PRD）+ 技术设计（TDD）+ Codex 分阶段执行手册**  
版本：1.0（开发基线）｜编写日期：2026-10-10｜目标平台：Windows 10/11 x64，后续 macOS  
交付性质：**可运行、可安装、可离线使用、可验收的桌面程序**。本文件不是“只做展示页面”的提示词。

> **给 Codex 的最高优先级指令**：先完整阅读第 0 章《开发前准备与自动配置》，先检查权限、环境、仓库和 Windows CI 构建，再进入业务开发。用户不是程序员，用户本机不预装任何开发语言。能在 Codex 云端/仓库 CI 完成的安装、编译、测试、打包全部在远端执行；确实需要用户操作的地方只给明确、简短、可验证的操作步骤。绝不索要密码、Figma 令牌或 GitHub 私钥。每一阶段产出运行结果、测试结果、风险列表和下一阶段计划，禁止把 mock 数据页面冒充已完成的实际功能。

---

# 0. 开发前准备与自动配置（必须先执行）

## 0.1 用户要准备什么（用户操作清单）

| 项目 | 是否必须 | 用户需要做的事 | Codex 应完成的事 |
|---|---|---|---|
| ChatGPT/Codex 使用权限 | 必须 | 打开可编辑代码仓库的 Codex 工作环境 | 判断是否能运行命令、联网安装依赖、提交代码、触发 CI |
| GitHub 账号 | 推荐且为了自动打包近乎必需 | 登录 GitHub；授权 Codex/集成访问**指定仓库** | 创建或初始化私有仓库、推送代码、配置 GitHub Actions（依授权情况） |
| GitHub 仓库 | 必须有可写仓库 | 若无法授权 Codex 创建仓库，按提示手动建一个**私有**仓库 | 生成仓库文件、工作流、保护 `.env` 等敏感文件 |
| Windows 10/11 64 位电脑 | 必须 | 最后下载安装构建好的 `.exe`；按需允许 WebView2 安装 | 产出 Windows 安装包，提供下载位置及校验信息 |
| Figma 账号及软件/网页版 | 使用 Figma 时必须 | 能在 Figma 打开设计文件；如后期用配套插件需手动导入插件 | V1 先实现 SVG/PNG 剪贴板流程；后续开发 Figma 插件 |
| Node.js / Rust / C++ Build Tools | 用户本机**不需要** | 无需操作 | 只在云端/CI Windows Runner 自动安装/使用 |
| 云服务器、云数据库 | V1 不需要 | 无需购买或部署 | 使用本地 SQLite 和本地文件，离线优先 |
| 系统剩余空间 | 建议 ≥1 GB 作为首次体验预算 | 检查电脑磁盘空间；素材越多占用越大 | 设置缓存限额、可查占用、可清理缓存 |

**明确区分**：
1. Codex 无法代替用户接受网站的账号授权、双重验证、隐私条款或操作系统的 UAC 提示；用户需要在相关页面本人确认。
2. Codex 无法凭空获得 GitHub/Figma 权限；没有权限时不允许假称“已配置”。
3. GitHub Actions 运行的开发工具安装在构建环境，不要求用户 Windows 电脑装 Node/Rust/Python/VS Code。
4. 最终程序必须下载并安装在用户电脑；此方案不是“云端开发完无需安装的软件”。
5. 无代码签名证书的首版 Windows 安装程序可能出现 SmartScreen/未知发布者警告，必须如实提示；不得通过关闭安全功能来规避。
6. WebView2 是 Windows 上 Tauri 的页面运行环境。大多数现代 Windows 10/11 已有；若没有，由安装包按选定策略引导安装，可能联网，离线安装包可能显著增大。

## 0.2 Codex 必须先执行的检查与配置顺序

**S0-01 环境审计**
- 判断当前是 Codex 云端容器、Codespace、Windows runner 还是本机；打印 OS、架构、仓库目录、Git 状态、可用命令、可用网络，以及 GitHub 远程地址（不得打印任何秘密）。
- 检查仓库是否已有代码，禁止覆盖已有用户项目；如非空先报告迁移策略。
- 检查 GitHub remote、分支与 push 权限；无权创建或推送时明确展示用户需要完成的最少授权动作。
- 检查 Node LTS、包管理器（优先 pnpm）、Rust stable、cargo、Tauri CLI、系统构建依赖。缺少的只在**开发/CI 环境**补齐。

**S0-02 初始化项目**
- 初始化私有 Git 仓库或使用现有授权仓库；默认仓库名称 `design-workbench`，软件暂名 **DesignWorkbench**，后续可更名。
- 配置 React + TypeScript + Vite + Tauri 2 + Tailwind CSS + shadcn/ui（所选 UI 套件与实际版本须兼容）+ Zustand。
- Rust 侧配置命令处理、SQLite、文件操作、缩略图服务和业务逻辑；建议 sqlx/rusqlite 二选一，统一由 Rust 数据层访问 SQLite，避免前端直接执行任意 SQL。
- 配置 ESLint、Prettier、TypeScript strict、Rust fmt/clippy、单元测试框架 Vitest、E2E 框架 Playwright（可用环境中运行）。
- 添加 `.gitignore`、`.editorconfig`、环境变量示例（仅占位，不写真实凭据）、LICENSE/README/CONTRIBUTING、变更日志、架构决策记录 `docs/ADR/`。

**S0-03 自动构建配置**
- 在 `.github/workflows/` 配置：`ci.yml`（lint/typecheck/test/compile）与 `release-windows.yml`（Windows runner 编译 Tauri 2 NSIS `.exe` 安装包；可选 MSI）。
- 构建产物保存在 GitHub Actions Artifacts；可选手动批准后创建私有测试版本 Release。不得默认公开素材或将私有仓库转为公开。
- Release 与 CI 分离：每次普通提交触发 CI；仅手动 workflow_dispatch 或版本 tag 发布安装包，防止误发布。
- 记录所用 Node/Rust/依赖版本，提交 lockfile，固定 GitHub Actions 关键 action 的安全版本策略。
- 工作流添加构建产物 SHA-256；提供可下载的文件名、构建编号、版本号。
- 检查首次成功构建：必须有真实安装包产物，不能只通过前端 `vite build` 就声称完成桌面软件。

**S0-04 初始化验证**
- 在云端开发环境执行可用的静态检查、单元测试及构建；若 Linux 容器不能产生 Windows 安装包，必须由 Windows GitHub Runner 产出，记录两者测试差异。
- 保证空仓库可自动安装、运行、构建，文档记录实际执行命令与成功/失败结果。
- 建立 `docs/SETUP_FOR_NON_DEVELOPER.md`：用截图占位或简单文字告诉非程序员如何登录 GitHub、在哪里点 Actions、在哪里下载 EXE、如何报告 bug。

**S0 验收门槛**：用户无需在本机安装编程语言；仓库有真实代码、Lockfile、CI、Windows 构建工作流；Windows runner 能产生安装包或明确报告构建阻碍、解决方式；未完成 S0 不开始大型业务功能。

## 0.3 给 Codex 的“开工提示词”（直接复制）

```text
你是本桌面软件的资深架构师和全栈工程师。仓库将提供 DesignWorkbench_完整开发需求与Codex执行手册_v1.0.md。
首先完整读取该文档，严格按第0章执行 S0-01~S0-04。
我本人是UI设计师，Windows电脑没有Node.js、Rust或Python，禁止要求我在本机配置开发语言；请在可用云端环境和GitHub Actions Windows Runner完成配置、构建和验证。
先检查你对仓库、终端、网络、GitHub的真实权限。能自动配置的直接做；涉及账号授权、密钥、GitHub仓库权限或Figma插件安装等必须由我亲自操作的，仅提供准确的分步说明，不要假称已经授权。
先交付：1.环境审计表；2.仓库目录；3.真正可运行的桌面空壳；4.通过的CI；5.Windows安装包自动构建流程；6.首次构建报告；7.给非程序员的安装说明。
阶段完成后先汇报验收结果，再按文档下一阶段继续，不得用静态Mock页替代真实数据能力。
```

---

# 1. 产品目标与边界

## 1.1 产品定义

**DesignWorkbench**：面向个人 UI/UX 设计师的 Windows 桌面设计资产工作台，将**项目、SVG 图标、Figma 导出的静态组件素材、Design Tokens、设计参考图片、快捷收集与搜索**统一管理，支持离线运行及尽可能顺畅地与 Figma 交换资产。

典型场景：
- 设计时需要返回箭头 → 快捷键呼出搜索 → 预览与改色 → 一键复制 SVG → 粘贴到 Figma。
- 在 Figma 做出一个漂亮的图标/组件 → `Copy as SVG` 或 PNG → 切到软件粘贴 → 命名、分类、保存，之后反复复用。
- 项目有橙/紫/蓝三套主题 → 快速切换 Token，预览组件配色 → 导出规范 JSON/CSS。
- 从网站/截图/Figma 收集 Banner、导航、卡片案例 → 图片图库分类、打标签、搜索、按颜色检索（后续）。
- 小悬浮窗置顶 Figma 边缘，不用每次打开庞大主界面。

## 1.2 明确不做（V1）

- 不做第二个 Figma：不支持在本软件内编辑 Figma Auto Layout、原生 Variant、约束、原型互动、组件实例关系。
- 不承诺静态 SVG 可以“还原成原生 Figma Component/Variant”；它是矢量资产快照。
- 不实现 Photoshop 级图片处理、任意复杂路径布尔运算、完整的矢量重绘引擎。
- 不强制用户登录、不购买云服务器、不实现多人协同与云同步；云端 NestJS + PostgreSQL 仅为未来可选扩展。
- 不默认上传任何私人图片、设计稿、SVG 或项目数据；不隐式同步到任何公共服务。
- 不将 Figma API 受权限约束的能力写成无条件保证。

## 1.3 首版交付定义（Release 1.0）

首版必须有：项目管理、全局/项目资产库、SVG 图标导入与属性编辑、Figma 复制/粘贴交互、Figma 静态组件收集、图片库、Token 多主题管理、搜索、标签和文件夹、悬浮面板、备份恢复、Windows 安装包。

**跨应用原生拖拽**是关键体验但须在早期做兼容性 PoC：先验证目标 Figma 的网页和桌面版本是否接受 Tauri 传出的 SVG 文件/文件 URI/系统数据对象；如无法稳定支持，正式界面必须提供效果可靠的「复制 SVG」「导出 SVG」「打开位置」替代。不得用网页内部拖拽成功当成跨进程拖拽成功。

## 1.4 衡量标准

- 1000 个图标、2000 张普通图片可正常建索引并可搜索；不要求所有原图常驻内存。
- 单个图标首次打开、改色、复制流程直观，核心交互不超过约 3 次操作（不包括首次导入）。
- 关闭网络后仍可搜索、编辑、分类、预览、复制、导出本地资产。
- 即使应用异常关闭，也不能造成已保存原始素材损坏；重要状态原子落盘。
- 对 1000 个索引项，在常见中档 Windows 电脑上交互流畅，采用虚拟列表；具体性能基线须通过 CI/手工报告记录环境，不虚报绝对毫秒保证。

---

# 2. 用户角色、信息架构与界面要求

## 2.1 用户角色

V1 仅支持本地单用户，不需要账号；未来可添加多设备同步身份体系。用户可能只会图形操作，不会终端和编程；所有常用操作必须通过 UI 完成。

## 2.2 一级导航（固定）

1. **工作台**：最近使用、收藏、项目入口、快捷导入、磁盘占用。
2. **图标库**：图标分类、搜索、参数编辑、SVG 复制/导出。
3. **组件库**：静态 Figma 组件素材的保存、预览、分类和复用。
4. **Design Tokens**：Token 套件、主题/模式、变量、引用、导入导出。
5. **设计图库**：图片保存、批量导入、网格/瀑布流、预览、分类。
6. **收集箱**：拖入/粘贴导入、去重提醒、待命名分类。

全局固定能力：顶部项目切换器、搜索、悬浮面板入口、设置、备份提示。项目导航可以选择“全部资源”或单个项目；默认记住上次选择。

## 2.3 关键页面规格

**主窗口**：默认 1240×800（可调整，最小建议 980×650）；三栏为可选布局：左侧导航+分类树，中间素材网格/列表，右侧属性检查器；较窄窗口隐藏或折叠属性栏。UI 使用中性色底板，界面本身不与用户业务主色绑定。

**图标库**：卡片展示图标、名称、尺寸、收藏；点击打开编辑面板，预览区支持棋盘透明底、白/黑底切换、不同尺寸；属性区尺寸、颜色、填充、描边、端点/连接、旋转（基础变换）；顶部复制、保存为副本、导出。批量选中仅允许双方都支持的操作。

**图片库**：自适应网格布局，原图比例预览；支持拖拽调整缩略图密度；详情查看原图、文件尺寸、格式、颜色、来源、备注、标签；缩放适应窗口/100%，放大时按需加载，不在列表中解码所有原图。

**Token 页**：左侧套件与主题树，中间分组/变量表，右侧可视化实时预览；支持搜索、颜色选择器、别名引用、错误诊断（悬空引用/循环/类型错误）、保存历史、导入/导出。显示“本地已保存”状态。

**组件库**：像图标库一样以图片/SVG 静态快照管理；无需实现原始 Auto Layout、Variant；可选择 SVG/PNG 快照、关联原 Figma 文件链接；支持拖出/复制、命名、搜索、标签。

**收集箱**：拖拽文件到空白区、快捷键粘贴；支持批量选择分类、重命名、归档到项目；检测已存在文件时提供“忽略 / 添加现有资产到当前分类 / 保留新副本”操作。

**悬浮面板**：可调大小和置顶，最小 300×400，搜索框+紧凑网格+复制；不自动挡住系统关键窗口；首次告知全局快捷键以及可能发生的快捷键冲突。

**设置**：工作目录、缓存清理、备份和恢复、键盘快捷键、语言（先简体中文）、开机启动开关（默认关）、更新渠道（手动优先）、隐私、诊断日志导出。

## 2.4 通用状态

每一页都需有：加载、空状态、无搜索结果、导入进行中、成功、失败可重试、权限不足、磁盘不足、来源丢失、无网络（需要联网时）、禁止误删的二次确认；删除支持回收站/撤销而非立即抹除原文件。

## 2.5 视觉/易用性验收

- Windows 系统缩放 100%、125%、150% 下布局正常；中文文字无乱码、文本无截断遮挡。
- WCAG 基本可访问性：键盘导航、焦点明显、按钮有名称、对比度合理；不依赖颜色独自表达状态。
- 所有输入（颜色值、数值、文件名、Token 名）有即时合法性提示，错误不能静默丢弃数据。
- 不允许用户仅靠熟悉开发工具才会使用导入、搜索、备份。

---

# 3. 核心业务功能规格

## 3.1 P-01 项目与资源作用域（P0）

功能：新建/重命名/归档/删除项目；全局资产可被多个项目引用，不复制物理文件；项目的图标、组件、图片、Token 套件独立展示。项目删除默认只解除关联，真正删除共享素材需单独确认。支持“默认项目”和“全部项目”。

验收：同一个 SVG 可同时关联 A/B 项目；从 A 移除不影响 B；重启后数据仍在；改名不改变 asset ID。

## 3.2 I-01 SVG 图标库（P0）

支持：SVG 文件批量导入、剪贴板 `image/svg+xml`/文本 SVG 粘贴、拖入文件、收藏、分类、标签、搜索、编辑历史；支持原图预览、绘制区尺寸、视图盒 viewBox、导出 PNG/SVG（保留原始版本）。

编辑规则：
- **颜色**：基于当前图标结构编辑 stroke/fill；多色图标可按图层/颜色选择范围替换，禁止默认一键全部涂单色造成信息损坏。
- **描边粗细**：仅对真实使用 stroke 的图元直接修改 `stroke-width`；已轮廓化的 fill 路径不能准确逆推出原始笔画粗细，界面须提示“该图标不支持精确描边调节”。
- **描边样式**：linecap（round/butt/square）、linejoin（round/miter/bevel），可选 stroke-dasharray；支持恢复初始值。
- **填充**：无填充/当前颜色/指定颜色；应处理 `fill-rule`（evenodd/nonzero）并保留其语义。
- **尺寸**：修改导出尺寸时保留原始 viewBox；明确区分缩放预览与改变矢量路径坐标。
- **持久化**：原始 SVG 不可覆盖；修改生成独立编辑参数和渲染变体，允许“另存为新素材”。
- **安全**：禁止脚本、远程资源加载、事件处理器、外部实体、危险 foreignObject 等；SVG 在存储前解析/清洗并安全渲染缩略图；未经清洗的 SVG 不允许直接注入 HTML。

验收：单色 stroke 图标可改线宽、颜色、端点且复制到 Figma；填充轮廓图标改变线宽时有明确限制；原始文件 SHA-256 不变；恶意 SVG 不执行脚本/外部请求。

## 3.3 C-01 静态 Figma 组件/素材库（P0）

组件在此指“为复用而收藏的静态视觉素材”，不是 Figma 原生组件编辑系统。支持从 Figma `Copy as SVG`、`Copy as PNG` 粘贴进软件；普通 Figma Ctrl+C 的专有剪贴板格式不保证能被软件解码，须给出具体“Copy as SVG/PNG”操作提示。

字段：自定义名称、素材类型（组件快照/SVG/PNG）、类别、项目、标签、原始宽高、导入时间、来源 URL、Figma file key（可选）、node id（可选）、备注。允许选择导入到图标库或组件库；不强制保留变体/自动布局/实例绑定。

验收：Figma 中复制一个按钮为 SVG → 软件粘贴保存 → 重启仍可搜索 → 复制 SVG 粘回 Figma；源文件打不开时本地素材仍能使用。

## 3.4 M-01 设计图片库（P0）

首版支持 PNG/JPEG/WebP/GIF（GIF 首版可静态缩略图、原文件保存；是否支持动图预览由实际依赖能力决定并标明）、SVG（需要分类为“图标/插画/设计图”，默认安全预览）；屏幕截图、海报、Banner、参考设计、汽车模型素材均为常规图片资产，不硬编码领域分类。

支持：文件拖入、系统文件选择器、从剪贴板粘贴图片、批量导入、编辑名称与备注、项目和多个文件夹/标签关联、收藏、原图预览、尺寸/格式查看、复制图片到剪贴板、导出原文件、显示在资源管理器。先保留原文件并生成缩略图；图片 EXIF 方向正确显示，导出原图不改变元数据。

必须支持文件去重（内容哈希）和非破坏性删除（回收站）；列表延迟加载/虚拟化，大图只在详情按需解码。后期可增加取色、截图标注、相似图片和 OCR；V1 不依赖 AI 网络识别。

验收：一次导入 200 张图片不会让界面失去响应；图片断网可查看；分类变更不会改变原始文件；重复导入可提示；大图失败可重试，不导致整个库崩溃。

## 3.5 T-01 多主题 Design Token（P0）

采用 DTCG 2025.10 规范的核心概念管理 Token，必须保留 `$type`、`$value`、`$description`、别名引用等信息。支持每个项目多个 Token Set、多个主题（例如 Light/紫色/橙色），支持基础值与语义值两层：`color.palette.orange.500` → `color.action.primary`。

V1 最少支持 `color`、`dimension`、`fontWeight`、`number`、`duration`、`cubicBezier`、`string` 中使用明确规范或应用扩展的类型；特别说明 **string 并非 DTCG 2025.10 通用规范类型**，如果用户需要普通文本元数据必须存于软件扩展字段，**不要将其导出成假的标准 Token 类型**。界面首选颜色、字号/间距/圆角（dimension）、透明度（number）、字重（fontWeight），再逐步支持复合 typography/shadow/border。

标准兼容性：
- DTCG color 规范采用规范对象值（colorSpace、components，可选 hex），不只是把 `#F76B35` 当成规范原样格式。
- Dimension 格式用对象 `{ "value": 8, "unit": "px" }`。
- 别名 `{color.palette.orange.500}` 可解析，循环引用、悬空引用、类型不匹配必须阻断导出并定位错误。
- 项目主题管理属于**应用内部配置**，导出时生成每套主题独立的、可验证的 DTCG JSON；如采用正式 Resolver 格式，要按规范单独生成，不能自创 `$modes` 后冒充 DTCG 规范。
- 对未知 `$extensions` 保留数据；允许导入带部分未知类型的文件但标记“未完全支持”，绝不静默丢弃未知字段后覆盖源文件。

功能：创建套件/主题、增删改变量、分组树、搜索、颜色拾取、别名引用选择器、复制值、主题切换预览、JSON 导入/导出、CSS custom properties 导出、变量版本记录与恢复。

验收：同一 `color.button.primary` 在橙/紫主题分辨显示并正确导出；修改基础 Token 自动影响引用的预览；循环引用报错；导入文件后尽可能无损导出，或明确显示不支持项。

## 3.6 Q-01 收集箱和快捷操作（P0）

- 应用内拖拽多文件、全局 Ctrl+V 收集剪贴板中 SVG/PNG/JPG/WebP/文件路径（依平台支持）；粘贴非图片文本时不自动生成素材。
- 导入任务有进度/取消/失败重试；批量重命名支持固定前缀+顺序号；导入完成提供“一键归类”。
- 首版全局快捷键可配置，候选 `Ctrl+Shift+Space`，遇系统冲突能检测并更换；不要强占用户已有快捷键。
- 搜索范围可切换全部/当前项目/图标/组件/图片；有最近搜索与最近使用（可清除历史）。
- 中文/英文/标签搜索；中文拼音可做索引扩展。不能单凭 SQLite FTS5 默认 unicode 分词断言所有中文子串查询都正确；使用有测试覆盖的包含匹配/自定义词元策略。

## 3.7 F-01 Figma 往返（P0/P1）

**P0：低依赖、立即可用**
- Figma → 本软件：Copy as SVG / Copy as PNG → 软件粘贴收集。
- 本软件 → Figma：复制 SVG 到系统剪贴板，Figma 画布中 Paste；Windows 上实现与测试 SVG MIME/HTML 文本等合理格式；如目标软件不支持某种剪贴板格式，回退为导出 SVG 文件并拖入/菜单导入。
- 本软件 → Figma：支持“导出 SVG / 文件另存 / 打开文件位置”；为原生文件拖拽做兼容验证，不把仅在应用内部的 HTML5 DnD 视为完成。

**P1：可选 Figma 配套插件**
- 用户在 Figma 运行插件，选择若干节点，插件通过 `exportAsync` 导出 SVG 字符串/PNG 字节并读取可用的元信息（名称、节点 ID、尺寸、文件上下文）。
- 插件 UI 与桌面端通信优先采用**明示发起的导出文件包或复制数据**，在 PoC 验证通过后可增加本机 loopback 传输。Figma 插件网络访问受 manifest 配置约束；不能假设从 Figma iframe 就能任意访问本机端口。
- 如启用 loopback：本地服务只绑定 `127.0.0.1`，使用短期配对码/随机令牌、CORS origin 限制、请求大小限制和超时；不得开放到局域网；退出后关闭；不把认证令牌写入日志。
- 不做自动抓取所有用户 Figma 文件、不处理未授权团队设计数据。
- Figma Variables REST API 受 Enterprise 及席位/权限限制；V1 **不依赖**它。后续变量导入可在授权文件内通过插件 API 做有权限的读取，按可用 API 能力实现，不承诺无条件双向同步。

**外部拖拽的验收判定**：分别在 Windows 版 Figma Desktop、Chrome/Edge 的 Figma Web（在可用环境中）试验，记录“拖出 SVG 文件是否被 Figma 接受并保持为矢量”。若一种环境失败，界面显示可靠替代操作；不得隐瞒兼容性差异。

## 3.8 B-01 备份、恢复、迁移（P0）

- 备份范围：SQLite 数据库 + 原始文件 + 缩略图可重建策略 + 项目配置 + Token JSON 及 manifest；生成带版本号的 ZIP 包（不包含认证密钥）。
- 使用 SQLite 在线备份 API 或安全一致性快照，不得直接在数据库写入中粗暴复制 db 文件；确保外键和文件引用一致。
- 压缩包恢复前验证校验和、路径遍历、最大解压大小、数据库版本；支持“恢复到新库”优先；破坏性覆盖前必须手动确认并先自动备份当前数据。
- 支持用户指定数据目录；变更目录时复制/校验后切换，原路径失败不应造成素材失踪。
- 默认备份建议 7 天周期但须由用户开关控制；本机空间不足时提示并跳过，不破坏现有库。

## 3.9 后续迭代（明确不影响 V1 验收）

P1：截图快捷收集、取色并存 Token、批量图像缩放/格式转换、Figma 来源追踪与插件、Token 主题预览模板、更多原生拖拽兼容、相似文件提示。  
P2：OCR、视觉相似图搜图、组件截图比对、浏览器扩展、云端同步与账号、团队共享、AI 标签建议、macOS 正式签名与公证。

---
# 4. 详细技术架构

## 4.1 首版技术选型与理由

| 子系统 | 推荐技术 | 使用边界 |
|---|---|---|
| Windows 桌面容器 | Tauri 2 | 小型桌面外壳、窗口、菜单、操作系统集成 |
| 前端 | React + TypeScript（strict）+ Vite | 全部交互 UI，不直接操作系统危险接口 |
| UI 基础 | Tailwind CSS + shadcn/ui + Radix 原语 | 设计风格统一、键盘无障碍 |
| 前端状态 | Zustand | 当前项目、过滤器、面板状态；不作为永久数据真源 |
| 本地业务后端 | Rust（Tauri commands） | 唯一可信写入路径：资产、导入、数据库、备份 |
| 数据存储 | SQLite（WAL、外键） | 结构化元数据、关联关系、Token、历史 |
| 媒体存储 | 用户应用数据目录 | 原始素材、变体、缩略图、备份 |
| SVG 处理 | Rust/TypeScript 经审计的 SVG 解析与渲染库 | 安全清洗、显示和导出；不能执行 SVG 中的脚本 |
| 图片处理 | Rust 图像库 + 懒加载 | 生成缩略图、读 EXIF、输出 PNG 等 |
| Token 序列化 | DTCG 2025.10 JSON | 导入/导出规范、别名、颜色对象、主题展开 |
| 测试 | Vitest + Rust tests + Playwright | 单元/集成/桌面主要流程验证 |
| 分发 | GitHub Actions Windows runner + Tauri bundle NSIS | 用户下载安装，无需编程环境 |
| 可选未来云端 | NestJS + PostgreSQL + S3 兼容对象存储 | 仅扩展，不阻塞 V1 |

原则：**Local-first / Offline-first**；Rust 负责事务、权限和 I/O；React 只做用户交互；所有文件先校验再入库；即便没有网络，本地主要功能可用。

## 4.2 模块边界

```text
Desktop Shell (Tauri 2)
├── React UI
│   ├── Dashboard / Project Switcher / Search / Floating Panel
│   ├── Icon Library / Static Component Library / Image Gallery
│   ├── Design Token Editor / Inbox / Settings
│   └── Typed IPC client + Zustand (ephemeral UI state)
├── Rust application services
│   ├── Project / Folder / Tag / Search service
│   ├── Import / Clipboard / SVG sanitize / Image thumbnails
│   ├── Asset metadata / Blob store / Variants / Export
│   ├── Token parser / Resolver / Exporter / Revision
│   ├── Backup / Restore / Migration / Diagnostics
│   └── Tauri Commands / OS adapters / Permission boundary
├── SQLite (metadata, indexes, relations, revisions)
└── AppData media folder (content-addressed original blobs, previews)

Figma
├── Phase 1: Native Figma Copy as SVG/PNG ↔ Desktop Clipboard/Files
└── Phase 2: Companion Figma Plugin (explicit user-selected export)
```

## 4.3 仓库建议目录

```text
design-workbench/
├── .github/workflows/
│   ├── ci.yml
│   └── release-windows.yml
├── apps/desktop/
│   ├── src/
│   │   ├── app/                 # 路由、布局、依赖注入
│   │   ├── pages/               # dashboard/icons/components/images/tokens/inbox/settings
│   │   ├── components/          # 公共 UI、缩略图、编辑控件
│   │   ├── features/            # project、search、import、svg-editor、tokens
│   │   ├── services/ipc/        # Tauri typed commands, no raw DB SQL
│   │   ├── stores/              # UI state only
│   │   ├── hooks/
│   │   ├── styles/
│   │   └── tests/
│   ├── src-tauri/
│   │   ├── src/
│   │   │   ├── main.rs / lib.rs
│   │   │   ├── commands/
│   │   │   ├── domain/
│   │   │   ├── services/
│   │   │   ├── persistence/
│   │   │   ├── media/
│   │   │   ├── security/
│   │   │   └── tests/
│   │   ├── migrations/
│   │   ├── capabilities/
│   │   └── tauri.conf.json
│   └── package.json
├── apps/figma-plugin/          # P1; V1 可放接口桩与独立规划，不强制构建
├── packages/contracts/         # 可 JSON schema/codegen 的请求响应定义
├── packages/design-tokens/     # Token 规范定义与测试样例
├── fixtures/                   # 合法/恶意 SVG, 图片, tokens, 导入文件
├── docs/
│   ├── SETUP_FOR_NON_DEVELOPER.md
│   ├── USER_GUIDE.md
│   ├── API.md
│   ├── SECURITY.md
│   ├── RELEASE.md
│   ├── TEST_REPORT.md
│   └── ADR/
├── README.md
├── pnpm-workspace.yaml
└── pnpm-lock.yaml
```

目录可因实际工具版本轻微调整，但不得把全部业务逻辑塞进单个 `App.tsx` 或 Rust 的 `main.rs`。

## 4.4 事务与本地文件一致性

所有导入使用“暂存→安全解析→哈希去重→写入 blob→生成缩略图→SQLite 事务写元数据”的流程。若中途失败，要能清理孤儿临时文件；数据库成功但缩略图失败时允许重建且标记待处理。更新 Token 使用事务及 revision。备份创建一致性快照。不使用文件名作为资产唯一键。

## 4.5 Windows 用户数据目录

使用 Tauri 已定义的用户应用数据目录解析接口，**不要将用户素材写在程序安装目录**；建议结构：

```text
<AppData>/DesignWorkbench/
├── data/designworkbench.sqlite3
├── media/blobs/<sha256-prefix>/<sha256>
├── media/derived/<asset-id>/<variant-id>.<ext>
├── cache/thumbs/<hash>_<size>.webp
├── import-staging/
├── backups/
├── logs/
└── config/settings.json
```

用户可在 UI 中查看实际数据路径；允许迁移到新目录。内部路径必须规范化，拒绝 `../` 等路径遍历，避免对其他目录误删。

---

# 5. 数据结构与持久化设计

## 5.1 实体关系与核心模型

核心关系：
- `projects` 与 `assets` 是多对多（`project_assets`）。
- `assets` 描述逻辑资产；`blobs` 描述原始文件内容（按 SHA-256 去重）。同一个 blob 可以被多个 asset 引用。
- `folders` 可属于项目或全局；资源与文件夹多对多（`asset_folders`）。
- `tags` 与 `asset_tags` 多对多；`favorites` 可记逻辑资产状态。
- `token_sets` 属于项目或全局；`token_themes` 与 `tokens`/`token_values` 决定多主题配置；`token_revisions` 保存更改历史。
- `figma_sources` 与本地资源关联，仅提供来源信息与可用的唯一标识；不能视为原生组件结构。

## 5.2 SQLite 最小表结构（示意，可在迁移中加强约束）

```sql
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;

CREATE TABLE projects (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  color TEXT,
  archived_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE blobs (
  sha256 TEXT PRIMARY KEY,
  rel_path TEXT NOT NULL UNIQUE,
  mime TEXT NOT NULL,
  byte_size INTEGER NOT NULL CHECK (byte_size >= 0),
  created_at TEXT NOT NULL
);

CREATE TABLE assets (
  id TEXT PRIMARY KEY,
  blob_sha256 TEXT NOT NULL REFERENCES blobs(sha256),
  kind TEXT NOT NULL CHECK (kind IN ('icon','component','image')),
  name TEXT NOT NULL,
  original_filename TEXT NOT NULL,
  width REAL,
  height REAL,
  description TEXT NOT NULL DEFAULT '',
  source_type TEXT NOT NULL DEFAULT 'file',
  checksum_version INTEGER NOT NULL DEFAULT 1,
  deleted_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE project_assets (
  project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
  PRIMARY KEY(project_id, asset_id)
);

CREATE TABLE folders (
  id TEXT PRIMARY KEY,
  project_id TEXT REFERENCES projects(id) ON DELETE CASCADE,
  parent_id TEXT REFERENCES folders(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  created_at TEXT NOT NULL,
  CHECK (id <> parent_id)
);

CREATE TABLE asset_folders (
  asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
  folder_id TEXT NOT NULL REFERENCES folders(id) ON DELETE CASCADE,
  PRIMARY KEY(asset_id, folder_id)
);

CREATE TABLE tags (
  id TEXT PRIMARY KEY,
  label TEXT NOT NULL,
  normalized_label TEXT NOT NULL UNIQUE
);

CREATE TABLE asset_tags (
  asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
  tag_id TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY(asset_id, tag_id)
);

CREATE TABLE asset_variants (
  id TEXT PRIMARY KEY,
  asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
  edit_spec_json TEXT NOT NULL,     -- validated JSON, not arbitrary HTML/JS
  derived_rel_path TEXT,
  created_at TEXT NOT NULL
);

CREATE TABLE figma_sources (
  asset_id TEXT PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
  figma_url TEXT,
  file_key TEXT,
  node_id TEXT,
  component_key TEXT,
  imported_at TEXT NOT NULL
);

CREATE TABLE token_sets (
  id TEXT PRIMARY KEY,
  project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE token_themes (
  id TEXT PRIMARY KEY,
  token_set_id TEXT NOT NULL REFERENCES token_sets(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  sort_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE tokens (
  id TEXT PRIMARY KEY,
  token_set_id TEXT NOT NULL REFERENCES token_sets(id) ON DELETE CASCADE,
  path TEXT NOT NULL,
  dtcg_type TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  extensions_json TEXT NOT NULL DEFAULT '{}',
  UNIQUE (token_set_id, path)
);

CREATE TABLE token_values (
  token_id TEXT NOT NULL REFERENCES tokens(id) ON DELETE CASCADE,
  theme_id TEXT NOT NULL REFERENCES token_themes(id) ON DELETE CASCADE,
  value_json TEXT NOT NULL,
  PRIMARY KEY (token_id, theme_id)
);

CREATE TABLE token_revisions (
  id TEXT PRIMARY KEY,
  token_set_id TEXT NOT NULL REFERENCES token_sets(id) ON DELETE CASCADE,
  actor TEXT NOT NULL DEFAULT 'local',
  reason TEXT,
  snapshot_json TEXT NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE import_jobs (
  id TEXT PRIMARY KEY,
  status TEXT NOT NULL CHECK(status IN ('pending','running','completed','failed','cancelled')),
  summary_json TEXT NOT NULL DEFAULT '{}',
  started_at TEXT NOT NULL,
  finished_at TEXT
);

CREATE TABLE app_settings (
  key TEXT PRIMARY KEY,
  value_json TEXT NOT NULL
);

CREATE INDEX idx_assets_kind_updated ON assets(kind, updated_at DESC);
CREATE INDEX idx_assets_name ON assets(name);
CREATE INDEX idx_project_assets_asset ON project_assets(asset_id);
CREATE INDEX idx_tokens_set_path ON tokens(token_set_id, path);
```

**实现补充要求**：
- 增加 `asset_usage`/`favorites`/`backup_records`/`audit_events` 等实际需要的表，以及数据库迁移版本管理；表结构示例不是最终唯一可用 Schema。
- SQLite FTS 索引应结合汉字查询的实际测试；搜索可先使用折中方案（规范化字段+LIKE/标签映射）并测性能，再引入 FTS5 trigram 或专门的分词方案。
- `project_assets` 关联表未指定数据顺序，可增加使用/置顶/排序字段。
- 文件真实删前需检查 `blob` 引用数量；不能因为删除一个项目就删掉另一个项目使用的物理文件。
- Token 同一路径在不同 Theme 的值通过 `token_values` 保存；导出时复用 DTCG 结构生成每主题文件。
- 文件安全限制、坏格式和路径校验应在 Rust 层执行，不可靠前端校验。

## 5.3 数据库迁移与升级

- 每个版本有递增迁移脚本；迁移前检查 schema version，必要时自动保存数据库一致性快照。
- 禁止启动时静默丢弃旧表/旧字段；迁移失败回滚并允许导出诊断包。
- 后续添加云同步时，每个实体已有稳定 UUID；不直接使用文件绝对路径做跨设备 ID。

## 5.4 资产去重逻辑

导入的文件内容使用 SHA-256；默认相同文件只保存一个 blob。用户可创建多个逻辑资产记录引用同一 blob，以便不同名称/标签/类型；同名不同内容不认为相同。近似图匹配并非 V1 必需；不能把感知哈希误当“文件完全相同”的可靠依据。

---

# 6. 跨层服务接口与错误模型

## 6.1 命令接口（Rust ↔ React）

Tauri IPC 应定义类型安全的请求/响应结构，统一序列化。示例（只是名称建议，须由实际 TypeScript/Rust 类型生成或确保同步）：

```ts
type AssetKind = 'icon' | 'component' | 'image';
type AppErrorCode =
  | 'INVALID_INPUT' | 'UNSUPPORTED_FORMAT' | 'SVG_UNSAFE'
  | 'PERMISSION_DENIED' | 'FILE_TOO_LARGE' | 'DISK_FULL'
  | 'NOT_FOUND' | 'IMPORT_CANCELLED' | 'DB_FAILURE'
  | 'BACKUP_INVALID' | 'TOKEN_REFERENCE_CYCLE' | 'UNKNOWN';

type AppResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: { code: AppErrorCode; message: string; retryable: boolean } };

// Projects
listProjects(): Promise<AppResult<Project[]>>;
createProject(input: CreateProjectInput): Promise<AppResult<Project>>;
renameProject(id: string, name: string): Promise<AppResult<Project>>;

// Assets
importAssets(input: ImportAssetsInput): Promise<AppResult<ImportJob>>;
getImportJob(id: string): Promise<AppResult<ImportJob>>;
listAssets(query: AssetQuery): Promise<AppResult<Paginated<AssetSummary>>>;
getAsset(id: string): Promise<AppResult<AssetDetail>>;
updateAssetMeta(id: string, patch: AssetMetaPatch): Promise<AppResult<AssetDetail>>;
setAssetProjects(id: string, projectIds: string[]): Promise<AppResult<void>>;
createAssetVariant(id: string, edit: SvgEditSpec): Promise<AppResult<AssetVariant>>;
copyAssetToClipboard(id: string, mode: 'svg' | 'png'): Promise<AppResult<void>>;
exportAssets(ids: string[], options: ExportOptions): Promise<AppResult<ExportReceipt>>;
trashAssets(ids: string[]): Promise<AppResult<void>>;
restoreAssets(ids: string[]): Promise<AppResult<void>>;

// Tokens
listTokenSets(projectId?: string): Promise<AppResult<TokenSet[]>>;
createTokenSet(input: CreateTokenSetInput): Promise<AppResult<TokenSet>>;
upsertToken(input: UpsertTokenInput): Promise<AppResult<Token>>;
validateTokenSet(tokenSetId: string, themeId: string): Promise<AppResult<TokenValidation>>;
importTokens(path: string, options: TokenImportOptions): Promise<AppResult<TokenImportReport>>;
exportTokens(id: string, themeIds: string[], format: 'dtcg' | 'css'): Promise<AppResult<ExportReceipt>>;

// Storage & backup
getStorageStats(): Promise<AppResult<StorageStats>>;
createBackup(destination: string): Promise<AppResult<BackupReceipt>>;
inspectBackup(path: string): Promise<AppResult<BackupInspection>>;
restoreBackup(path: string, mode: 'new-library' | 'replace-current'): Promise<AppResult<void>>;
```

每个函数必须明确：输入验证、用户可见错误、是否破坏性、是否可取消；业务功能 UI 必须调用真实 IPC。后端不得接受前端任意 SQL 字符串或任意文件写入路径。

## 6.2 长任务进度

批量导入/缩略图/备份使用后台异步任务 + 事件进度：`jobId`、`completed`、`total`、`failed`、`status`；事件需有 jobId 防止串任务，关闭 UI 或取消时保证资源回收；重启可检查残留 staging 并恢复/清理。

## 6.3 Figma 来源接口

不强制用户输入 Figma token。来源 URL 经校验后可保存为 metadata 并通过操作系统默认浏览器打开；仅当后续明确增加官方授权流程时才调用 REST API，不能默认持有长期静态 API Key。

---

# 7. 实现难点与安全规范（强制）

## 7.1 SVG 安全

- 使用解析器白名单安全元素/属性，删除 `script`、`on*` 事件、外部 URL、动态网络图片、DTD/外部实体、危险嵌入内容；对复杂合法图标的滤镜/裁剪做有界支持与回归测试。
- 预览时禁止将原始不可信 SVG 字符串直接写入 DOM 的 `innerHTML`；优先转换为安全数据 URL/沙盒预览/栅格缩略图。
- SVG 外部 `<use href>`、`foreignObject`、嵌入字体与数据 URI 等需明确策略；默认拒绝远程依赖。
- 编辑器对不支持的属性应保留原始结构或清晰提示，而非静默改变外观。

## 7.2 文件与数据库安全

- MIME sniffing 与文件扩展名双重校验；防止伪装扩展名的恶意输入。
- 可配置导入限制（V1 默认单文件 100 MB，单批 500 项；超过只提示并允许用户分批操作）；对超大尺寸、压缩炸弹等增加解码资源上限。
- 限制 Tauri capabilities 的文件读写范围；把选择文件和写入路径能力限制到必要用户操作。禁止前端任意执行 shell 命令。
- 新增 `open external URL` 白名单/确认提示，避免 `file://` / 自定义 Scheme 任意跳转。
- 日志中不记录私人素材内容、访问令牌、完整剪贴板；诊断包需用户确认后才导出。
- 所有备份恢复做 ZIP Slip 防护、完整性验证、Schema 兼容性检测；需要密码备份时作为未来可选项，不以明文 Secret 文件捆入 ZIP。

## 7.3 剪贴板隐私

- 默认只在本软件前台用户主动按粘贴或点击导入时读取剪贴板；“自动监控剪贴板”如果未来加入必须默认关闭且清楚可撤销。
- 读取到文字/链接时不得自动发送到网络；显示隐私与权限说明。

## 7.4 原生拖拽兼容风险

- Tauri WebView HTML 拖放到其他进程并不等于 OS 原生 file drag；必须在 Windows 做 PoC。优先评估标准文件拖出、平台 API 或维护良好的 Tauri 插件；严禁引入无维护、无审计的高权限二进制代码而不做风险说明。
- Windows 成功后再考虑 macOS 拖出 API；若原生文件拖出不稳定，复制 SVG/导出文件是验收允许的可靠回退，但须标注“原生拖拽未适配”。

## 7.5 Figma 权限限制与真实边界

- Figma 复制/导出 SVG/PNG 有文件的共享和拷贝权限限制；若用户没有权限，不能绕过。
- Figma Plugin API 的 node.exportAsync 可以导出支持的节点；具体返回类型因设置而异，版本改动时检查官方 docs。
- Figma Variables REST API 不是任意个人账户都可使用；官方文档要求 Enterprise 组织席位与相应权限，禁止将它设为 V1 必需服务。
- 插件本身不能保证在任意文件、任意团队权限下获取所有组件元信息；抓取用户未选中节点必须有清楚的界面授权和权限检查。

## 7.6 发布安全

- 无数字签名不等于软件恶意，但可能触发 Windows SmartScreen。README 说明来源、SHA-256 和风险；不得要求用户关闭杀毒软件。
- 自动更新不作为 V1 必需。若添加，必须考虑签名、HTTPS、更新来源校验、失败回滚；不要把任意 GitHub URL 当信任更新源。

---

# 8. 测试与验收计划

## 8.1 测试层级

- **静态检查**：TypeScript `tsc --noEmit`、ESLint、Rust fmt/clippy；有锁定依赖和依赖漏洞扫描。
- **单元测试**：Token 规范解析/循环引用、文件哈希去重、SVG 清洗器、路径/命名校验、业务数据转换。
- **集成测试**：SQLite CRUD/迁移、导入→缩略图→索引→搜索→导出、备份→恢复→哈希对比。
- **UI/E2E**：项目切换、图标改色、图片批量归类、Token 主题切换、空状态与错误提示。无法在 CI 直接模拟跨进程剪贴板时标注手工验证，不得伪造自动测试结果。
- **真实 Windows 手工验收**：安装/首次启动/升级/卸载、Figma Desktop 和 Figma Web 粘贴/拖拽、125% 缩放、中文文件名、网络断开、WebView2 缺失路径。

## 8.2 关键测试清单（验收 ID）

| ID | 场景 | 合格标准 |
|---|---|---|
| AC-001 | 空电脑安装（无 Node/Rust） | 正常安装并启动；若 WebView2 缺失有可靠引导 |
| AC-002 | 首次启动 | 初始化数据库和数据目录，UI 无崩溃 |
| AC-003 | 项目 A/B 共用 SVG | A 移除不影响 B；物理文件不误删 |
| AC-004 | 同一文件重复导入 | 检测哈希、允许关联或跳过、磁盘无重复 blob |
| AC-005 | Figma Copy as SVG → 导入 | 可预览、命名、保存、搜索及再次复制 |
| AC-006 | 软件 SVG → Figma | 以 Figma 实际测试为准，SVG 矢量可编辑；若格式不支持，有导出回退 |
| AC-007 | 改色与改线宽 | 对 stroke 型生效；对 outline 型提示限制；原件不变 |
| AC-008 | 恶意 SVG | 不执行脚本、不发外网请求、不读本地文件 |
| AC-009 | 导入 200 张图片 | 有进度，能取消与恢复，列表可滚动 |
| AC-010 | 设计图库分类 | 项目/文件夹/标签/收藏持久化，重启仍在 |
| AC-011 | 多主题 Token | Light/Orange/Purple 变量值互不覆盖 |
| AC-012 | Token 引用链/循环 | 正常引用正确解析，循环提示路径且阻止错误导出 |
| AC-013 | DTCG JSON 导入/导出 | 规范基础类型经 validator 验证，未知扩展被保留或明确报告 |
| AC-014 | 全局快捷键 | 冲突可重新设定且不造成应用死锁 |
| AC-015 | 悬浮面板 | 置顶开关和窗口关闭恢复正常，无隐形阻挡 |
| AC-016 | 断网 | 离线浏览、编辑、复制、导出均可用 |
| AC-017 | 备份恢复 | 新库恢复后资产数与 SHA-256 对应，Token/项目一致 |
| AC-018 | 删除与恢复 | 回收站可恢复，物理清理只删无人引用 blob |
| AC-019 | 数据升级 | 从旧数据库迁移成功，异常可回退/报告 |
| AC-020 | Windows 构建 | GitHub Actions 有 NSIS 安装包和 SHA-256 |
| AC-021 | 中文内容/UI 缩放 | 中文无乱码；Windows 125%、150% 不遮挡 |
| AC-022 | 非程序员说明 | 只用图形界面能完成安装、导入、备份、恢复 |
| AC-023 | 数据隐私 | 默认不上传原始素材或剪贴板，不将 Secret 写入日志 |
| AC-024 | Figma 原生拖出兼容测试 | 有记录 Desktop/Web 结果，失败时导出/复制回退真正可用 |
| AC-025 | 大文件与无权限 | 用户可理解的错误，不崩溃、不损坏数据库 |

每项须填写：测试方式（自动/手工）、实际结果（通过/失败/受限）、运行环境、截图或日志文件位置。不得仅写“预期通过”。

## 8.3 测试数据集

在 `fixtures/` 保留：
- 10 个常见 stroke SVG、10 个填充型 outline SVG、3 个多色 SVG、5 个恶意 SVG、2 个复杂渐变 SVG。
- PNG/JPEG/WebP/GIF，各含普通尺寸/透明/EXIF/中文名/大分辨率样例。
- 至少 3 套 Token JSON（各含 aliases 与一个错误 case），以及 DTCG 标准官方示例的兼容性测试。
- 不将真实私有 Figma 组件或版权有风险的用户图片写入公开仓库测试夹。

---

# 9. 分阶段开发计划（按顺序，一次只做一个阶段）

**执行原则**：每阶段都必须“代码 + 实际可用功能 + 测试 + 变更记录 + 界面截图 + Windows 构建/可用性情况”。任何功能依赖的 PoC 提前实施；任何平台限制必须直接暴露，不允许无限期堆 TODO。

| 阶段 | 目标 | 主要交付 | 通过后进入 |
|---|---|---|---|
| S0 | 配置、仓库、Windows 构建 | 基础桌面程序、CI、EXE、非程序员安装说明 | S1 |
| S1 | 桌面 UI 与数据库基础 | 6 导航骨架、窗口、SQLite migrations、项目 CRUD | S2 |
| S2 | 通用资产引擎 | 安全导入、blob 去重、文件夹/标签、缩略图、搜索 | S3 |
| S3 | SVG 图标库 | SVG 编辑器、变体/原件保护、剪贴板复制和导出 | S4 |
| S4 | 组件库与 Figma 基础往返 | Copy as SVG/PNG 进出、来源信息、跨软件兼容 PoC | S5 |
| S5 | 设计图片库 | 图片列表、详情、批量操作、压缩预览、收藏 | S6 |
| S6 | Design Token 系统 | 多套件主题、别名、规范导入/导出、预览 | S7 |
| S7 | 效率工具 | 悬浮窗、全局快捷键、收集箱、最近使用 | S8 |
| S8 | 稳定性、备份与发布 | 备份恢复、安全审计、CI Release、文档、验收 | V1.0 |
| S9（可选） | Figma 配套插件 | 选中节点批量输出 SVG/PNG/元信息 | V1.1 |

## S0 交付检查点

运行环境就绪、GitHub Actions 正常、Windows EXE 成功下载或提供真实阻断报告；让用户在自己电脑试装空壳，优先暴露 Windows Runtime/网络/权限问题。

## S1 验收

真实 Rust IPC + SQLite：新建项目/改名/切换/归档；重启仍保存；六个一级页面有真实路由与空状态。禁止仅用浏览器 localStorage 冒充最终数据层。

## S2 验收

单文件与批量导入可用，blob 去重、生效的项目关联与标签、多条件搜索；故意导入损坏/错误格式能优雅失败；无外部联网依赖。

## S3 验收

图标编辑预览实时更新，复制/导出 SVG 正确；对 outline 不支持的线宽操作有明确提示；原件 SHA 校验；安全 SVG 测试通过。

## S4 验收

真实 Figma Copy as SVG/PNG 粘贴保存；软件复制 SVG 在 Figma 可粘贴（Windows Desktop/Web 兼容报告）；原生拖拽可用则上线，不可用则提供可操作的回退并说明。

## S5 验收

批量图片导入、真实缩略图、按原图比例预览、移动分类、标签搜索、复制/导出图片；避免占满内存。

## S6 验收

图形 UI 可修改 Token 与主题；别名解析、错误处理；DTCG JSON 和 CSS 导出格式经单元/规范测试；所有变化入库可恢复。

## S7 验收

悬浮窗口置顶、快速搜索、快捷键、收集箱导入与最近使用真实打通；有冲突检测及开关。

## S8 验收

完整 AC-001~025 状态报告；备份恢复可靠；真实安装包签入私有测试 Release 或 Actions Artifacts；README、用户使用手册与已知限制说明完备。

---
# 10. 给 Codex 的逐阶段执行提示词

> 使用方法：先把本 `.md` 文件放在 Codex 可访问的仓库 `docs/` 下，启动时发送第 0 章提示词。确认 S0 通过，再依次给 S1、S2……提示词。提示词中的“实际完成”必须有代码路径和测试输出证据。

## Prompt S1 — 桌面架构、数据库和六大入口

```text
继续 DesignWorkbench 项目，仅执行 S1，不提前做其它阶段。阅读项目文档第1、2、4、5章。
在现有 Tauri 2 + React + TypeScript 项目中实现六个一级导航入口、顶部项目切换器、空状态、可折叠侧栏和设置页基础。所有常用 UI 使用可重用组件且支持简体中文。
在 Rust 数据层建立 SQLite 初始化与版本化 migration，完成项目 create/list/rename/archive/delete（软删除或归档优先），前端通过 typed IPC 操作，不得用模拟数组和localStorage作为真实持久化。
测试：项目 CRUD、关掉重开后仍在、错误输入验证、迁移可重复执行；运行 lint/typecheck/test，尽可能生成 Windows 构建产物。
汇报：改动文件、真实运行截图、测试命令与结果、已知问题、AC编号通过情况。仅 S1 全部验收通过才开始 S2。
```

## Prompt S2 — 统一资产引擎与分类搜索

```text
继续，仅执行 S2。建立 assets/blobs/projects/folders/tags 相关持久化和服务层；使用 SHA-256 按内容去重，原文件安全保存至用户 AppData 下，不允许逻辑删除误删其他项目共用文件。实现选择文件、拖入应用、单/批量导入、进度/取消、MIME检测、文件异常处理、缩略图生成、资产网格基础、项目/文件夹/标签/收藏/搜索。
应有完整从 UI 操作到 Rust/SQLite 的真实数据链路。测试重复文件、中文文件名、200文件批量、断网、非法格式和磁盘空间不足的模拟错误。产出 migration 和文档更新，展示实际数据持久化。不要开始 SVG 图标编辑器。
```

## Prompt S3 — 图标资产和 SVG 可编辑属性

```text
继续，仅执行 S3。读取文档第3.2、7.1章。实现安全 SVG 解析/清洗和缩略图，区分 stroke 型与 fill/outline 型。UI提供图标预览、编辑颜色、stroke-width、fill、linecap、linejoin、尺寸（保留 viewBox）、重置；原始 SVG 永不被覆盖，修改存 variant spec 或衍生文件。支持复制 SVG 到剪贴板、导出 SVG/PNG、另存为新素材。
逐一测试恶意SVG、渐变/多色、outline路径、中文命名与Figma粘贴。对无法精确改变粗细的outline图标显示解释，不得假装支持。提交测试报告、截图与可用构建。
```

## Prompt S4 — Figma 静态组件资产与复制粘贴往返

```text
继续，仅执行 S4。实现组件库（静态SVG/PNG快照）及 Figma 来源信息字段，支持Figma Copy as SVG/PNG之后在软件Ctrl+V导入、命名、分类、搜索、收藏；支持从软件复制SVG/PNG回Figma粘贴。明确普通Figma Ctrl+C专有格式不保证支持。
在Windows真实验证Figma Desktop与Chrome/Edge Figma Web（环境确实可用时）跨软件复制粘贴，并尽早做原生跨进程SVG文件拖拽PoC。记录受支持的系统剪贴板格式和实际结果；如不支持原生拖拽则提供导出SVG文件和复制SVG的可靠替代。不要实现原生组件Variant、Auto Layout或未授权Variables REST同步。
提交真实示例、测试视频/截图或失败限制报告，不能把app内部HTML拖动冒充成功。
```

## Prompt S5 — 设计图库

```text
继续，仅执行 S5。为 PNG/JPEG/WebP/GIF/安全SVG 实现设计图片图库；支持拖入、粘贴、批量导入、网格/详情预览、项目分类、文件夹、标签、备注、收藏、复制图片、另存导出和资源管理器定位。缩略图异步生成并缓存，大图按需加载、EXIF方向正确，原图永不被自动降质或覆盖。损坏文件、超大图片须优雅报错；对批量导入200张的流畅性与内存做实际测试。
必须从现有资产引擎复用项目/标签/哈希去重，不得重复开发另一套素材数据库。交付功能验收记录。
```

## Prompt S6 — 多套 Design Token 和主题

```text
继续，仅执行 S6。按DTCG 2025.10官方规范实现Token数据模型和JSON导入/导出；UI支持多项目、多套件、多主题、分组、基础值、语义别名、颜色/dimension/number/fontWeight等基础类型，实时主题预览和CSS变量导出。循环引用、悬空别名、类型不匹配、非法颜色要阻止错误导出并显示清晰错误。
注意：标准DTCG颜色、尺寸应使用规范对象表示；不要虚构$string类型或$modes字段并当成标准输出。内部应用的theme映射可以自定义，但每套主题导出的JSON必须可验证。导入未知扩展尽量保留并报告。增加revision历史和恢复。跑单元测试、校验样例与UI操作验收。
```

## Prompt S7 — 悬浮窗、收集箱与全局搜索

```text
继续，仅执行 S7。实现与主窗口共享数据库的置顶悬浮素材面板、可调整大小、全局搜索与可配置快捷键、收集箱批量归档、最近使用记录。全局快捷键遇到系统冲突应允许换键，禁用时正确释放。剪贴板只在用户主动粘贴时读取，默认不开启后台监听。搜索必须覆盖中文名称、英文名称、标签，测试真实中文命中。所有面板从真实SQLite读取，不能自行保存脱节缓存。
提供Windows 100%、125%、150%缩放下的截图和键盘操作测试。
```

## Prompt S8 — 稳定性、用户文档和 Windows 发布

```text
继续，仅执行 S8。完整实现备份恢复、回收站、库迁移、磁盘统计/缓存清理、日志导出、错误恢复。对SQLite使用一致性备份，ZIP恢复要防路径穿越/压缩炸弹，导出时不泄露机密。
执行单测、集成测、静态检查、Windows真实环境手工验收，逐项填写AC-001~AC-025。GitHub Actions产出Windows NSIS安装EXE、SHA-256，文件名包含版本号。生成非程序员使用说明：在哪里下载、怎么安装、怎么收集Figma SVG/PNG、怎么备份恢复。
列出仍受限的平台能力（原生拖拽、签名警告等）而不是宣称全部成功；修复P0/P1级bug之后提交V1.0候选版。
```

## Prompt S9 — 可选 Figma 插件

```text
仅在V1.0稳定后执行S9。开发独立Figma插件：用户在Figma里选中一个或多个节点，插件读取可用名称、nodeId、width/height等元数据，并按当前Plugin API调用exportAsync导出SVG/PNG快照，再以用户主动操作的方式传给桌面端（V1.1先做可下载导入包/剪贴板方案，loopback桥接须额外PoC与权限审核）。
不保存或伪造Auto Layout/Variants；不调用必须Enterprise才可用的Variables REST作为基础能力；不抓取未选择的私有内容。编写Figma插件安装和操作说明、兼容限制、安全测试。
```

---

# 11. Codex 执行管理规则（强制写入项目 AGENTS.md）

在仓库根目录创建 `AGENTS.md`，内容至少包括：

1. 项目内最高权威文档：本 PRD/TDD，所有功能必须满足验收项与“不做”边界。
2. 任意阶段前执行 `git status`、检查现有代码；不得删除用户资源、抹去他人改动，危险数据库变更先迁移与备份。
3. 实现阶段独立提交，遵守清晰的 commit；所有依赖锁版本，保持无秘密文件入库。
4. 优先写真实业务链路；UI 样例数据可作为演示 fixture，但不能拿 fixture 替代功能。
5. 任何 API 权限、平台行为、拖拽能力不确定时，先做最小可复现测试，并在报告中列明通过环境和失败环境。
6. 一个阶段完成后必须：编译/测试、更新文档、产出截图、提交验收矩阵，再开始后续开发。
7. 遇到无法获得的 GitHub/Figma/Windows 权限，只提供针对性用户操作指引，继续完成不依赖该权限的任务；不索要密码、私钥或明文 Token。
8. 不擅自开通收费 API、服务器或在线第三方服务；默认本地离线、不开自动上传。
9. 代码层禁止绕过 Rust 服务直接任意访问本机文件/SQLite；遵守最小能力授权。
10. 优先保障数据不丢失，禁止为了“顺利演示”移除校验、安全机制和错误提示。

每阶段结束的固定报告模板：

```text
# Sx 完成报告
- 阶段与提交：
- 实现功能（标注真实/受限/未完成）：
- 新增/修改文件：
- 测试命令、运行环境与结果：
- Windows 安装包构建结果/下载位置：
- 截图/录像证据：
- AC 验收通过/失败/待测：
- 风险和已知问题：
- 需要我亲自操作的授权/安装步骤（如无写“无”）：
- 下一阶段建议：
```

---

# 12. 安装包、资源占用与交付标准

## 12.1 安装包类型和目标

V1 Windows 以 NSIS `setup.exe` 为主，MSI 仅在 Windows runner 满足 WiX/VBScript 要求时生成。尽可能支持 Windows 10/11 x64。Windows WebView2 不一定需要独立下载，但安装链路必须在缺失 Runtime 的设备上有明确策略。

**工程预估（非实测保证）**：Tauri 2 轻量安装包可能是数十 MB，安装后主程序和初始缓存可能约 50–150 MB；若安装包内嵌完整 WebView2 离线 Runtime，安装包会显著增加。实际空间主要由用户收藏图片决定：例如 1000 张每张 1.5 MB，原文件就约 1.5 GB，另加缩略图、变体、索引和备份。必须在软件设置中显示数据库/原始素材/缩略图/备份各自用量。

## 12.2 交付物（缺一不可）

- 源代码私有 GitHub 仓库，含 README、AGENTS.md、工作流和 lockfile。
- 可下载的 Windows 安装包（真实 `.exe`，不是 Web 预览链接）。
- 版本号、Git commit、SHA-256 文件与构建任务链接。
- `docs/USER_GUIDE.md` 用户使用教程；`docs/SETUP_FOR_NON_DEVELOPER.md` 用户零开发环境安装指南。
- `docs/TEST_REPORT.md` 包含验收矩阵和已知受限功能，说明跨应用拖拽具体可用的目标环境。
- `docs/SECURITY.md` 说明本地数据路径、隐私、文件安全和恢复方法。
- 可选 `apps/figma-plugin` 独立源码与插件导入说明（S9）。

## 12.3 用户最后如何使用（面向不懂编程的用户）

1. 用户登录 GitHub → 打开本软件的私有仓库 → `Actions` → 找到成功的 Windows 打包任务 → 下载 `DesignWorkbench-...-setup.exe` 工件（如工件打包为 ZIP，先解压）。
2. 核对构建版本、来源、可选校验和 → 双击 `.exe` 安装；必要时按安装引导处理 WebView2，而不是安装 Rust/Node.js。
3. 首次启动后，选择保存目录或使用默认路径 → 创建第一个项目 → 导入 SVG/图片。
4. 在 Figma 选中图标 → `Copy/Paste as > Copy as SVG` → 切回 DesignWorkbench → 在图标库/收集箱按 Ctrl+V。
5. 从本软件复制 SVG → 切回 Figma 画布粘贴；若目标环境不识别则导出 `.svg` 并导入。
6. 打开 Token 页创建项目主题；定期在设置里进行本地备份。

---

# 13. 决策记录与风险清单

| 编号 | 风险 | 影响 | 防范/回退 |
|---|---|---|---|
| R-01 | Codex 无 GitHub 写入权限 | 无法自动构建/发布 | 用户授权指定私有仓库；先完成本地 repo 初始化 |
| R-02 | 云端 Linux 不能直接构建 Windows installer | 发布受阻 | 使用 `windows-latest` GitHub Runner；不假称 Linux 构建是 Windows 成品 |
| R-03 | Figma 剪贴板格式/浏览器行为不一致 | 复制粘贴失败 | Copy as SVG/PNG、导出/导入文件回退，真实环境测试 |
| R-04 | Tauri 外部系统拖拽兼容性不确定 | 不能直接拖到 Figma | S4 原生 DnD PoC，失败提供可靠复制/导出选项 |
| R-05 | SVG 轮廓化不可逆 | 粗细无法精确调整 | 明示限制，只对 stroke 图形精准编辑 |
| R-06 | 未经清洗的 SVG 可带危险内容 | 脚本/外部请求风险 | 安全解析、拒绝危险资源、测试 fixture |
| R-07 | 大量原图膨胀磁盘 | 空间不足、运行缓慢 | 哈希去重、缩略图缓存限制、存储占用统计和清理 |
| R-08 | Token 规范变化/复杂类型 | 导入导出损失 | 针对 DTCG 2025.10 验证、未知扩展保留、清晰诊断 |
| R-09 | 数据目录移动/迁移失败 | 丢资产 | 原子转移、复制后校验、回滚与备份 |
| R-10 | Windows 未签名安装包警告 | 用户安装信任障碍 | 透明发布来源/sha256，未来选择代码签名，不关闭系统安全 |
| R-11 | Figma Variables REST 计划限制 | 不能无条件读取变量 | V1 不依赖；后续插件+权限确认 |
| R-12 | Figma 插件到本地服务连接受限 | 插件互通失败 | 导出文件/粘贴方式作为主路径，桥接必须 PoC |

---

# 14. 官方资料与技术验证来源

> 下面的链接供 Codex 在实际开发时核对**最新**文档与依赖版本；链接不是对所有目标操作系统/账号权限都已通过实机测试的保证。

- Tauri v2 安装前置条件：https://v2.tauri.app/start/prerequisites/
- Tauri v2 Windows installer：https://v2.tauri.app/distribute/windows-installer/
- Tauri v2 GitHub Actions：https://v2.tauri.app/distribute/pipelines/github/
- Tauri GitHub Action：https://github.com/tauri-apps/tauri-action
- Tauri SQL 插件：https://v2.tauri.app/plugin/sql/
- Figma 设计工具间复制资源：https://help.figma.com/hc/en-us/articles/360040030374-Copy-assets-between-design-tools
- Figma 复制与粘贴：https://help.figma.com/hc/en-us/articles/4409078832791-Copy-and-paste-objects
- Figma Plugin exportAsync：https://developers.figma.com/docs/plugins/api/properties/nodes-exportasync/
- Figma Plugin getLocalVariablesAsync：https://developers.figma.com/docs/plugins/api/properties/figma-variables-getlocalvariablesasync/
- Figma Plugin Manifest 网络访问：https://developers.figma.com/docs/plugins/manifest/
- Figma Variables REST 权限：https://developers.figma.com/docs/rest-api/variables/
- DTCG 2025.10 格式规范：https://www.w3.org/community/reports/design-tokens/CG-FINAL-format-20251028/
- DTCG 2025.10 Resolver：https://www.w3.org/community/reports/design-tokens/CG-FINAL-resolver-20251028/

---

# 15. 发布前最终确认表

- [ ] 用户电脑无需 Node.js、Rust、Python、Visual Studio 即可安装和日常使用。
- [ ] 初版不需要订阅云服务器或支付 API；未授权不上传设计文件。
- [ ] 真实 Windows 安装包成功构建与安装，版本和校验信息清晰。
- [ ] 项目、图标、静态组件、图片、Token、收集箱、搜索、备份全部通过真实数据闭环。
- [ ] Figma Copy as SVG/PNG 可导入；软件 SVG 可复制/导出给 Figma。
- [ ] 跨软件原生拖拽有独立的 Desktop/Web 测试结果，不以模拟替代。
- [ ] 轮廓图标无法精准改粗细时，UI 明确提示。
- [ ] 图片可分类、打标签、预览、批量导入、保留原图。
- [ ] Token 多主题与别名生效，规范导出通过测试。
- [ ] SQLite 备份恢复、垃圾回收、迁移与恶意文件测试通过。
- [ ] README、非程序员安装说明、用户使用手册和已知限制齐全。

**完成定义**：以上 V1 必需项经真实测试通过，或经文档明确准许的回退方案验证可用，并有能让用户在 Windows 上实际安装的软件包，方可宣称完成 V1.0。
