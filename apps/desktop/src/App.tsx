// S0 desktop shell: displays native IPC readiness without pretending browser preview is the app.
import { useEffect, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import "./App.css";

export default function App() {
  const [runtime, setRuntime] = useState("正在检查桌面服务…");

  useEffect(() => {
    if (!isTauri()) {
      setRuntime("浏览器预览：桌面服务需在安装版中运行");
      return;
    }
    invoke<string>("app_info")
      .then(setRuntime)
      .catch((error: unknown) => setRuntime(`桌面服务启动失败：${String(error)}`));
  }, []);

  return (
    <main className="shell">
      <header className="topbar">
        <span className="mark" aria-hidden="true">D</span>
        <div>
          <h1>DesignWorkbench</h1>
          <p>你的本地设计资产工作台</p>
        </div>
        <span className="stage">S0 · 桌面基础</span>
      </header>
      <section className="welcome">
        <span className="eyebrow">准备就绪</span>
        <h2>把灵感留在自己电脑里。</h2>
        <p>桌面框架已建立。项目、图标、组件、图片和设计变量将在后续阶段逐步接入真实本地数据。</p>
        <div className="runtime" role="status">{runtime}</div>
      </section>
      <section className="next" aria-label="下一阶段">
        <h3>下一步 · S1</h3>
        <p>建立本地 SQLite 数据库与项目管理，并接入六个工作区入口。</p>
      </section>
      <footer>离线优先 · 素材默认只保存在本机</footer>
    </main>
  );
}
