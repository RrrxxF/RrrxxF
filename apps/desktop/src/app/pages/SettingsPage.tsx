// Settings report the native app data location and current offline behaviour.
import { useEffect, useState } from "react";
import { projectApi, type AppInfo } from "../../services/projects";

export function SettingsPage() {
  const [info, setInfo] = useState<AppInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  useEffect(() => {
    projectApi
      .info()
      .then(setInfo)
      .catch((cause: unknown) => setError(String(cause)));
  }, []);
  return (
    <>
      <div className="page-intro">
        <span className="eyebrow">PREFERENCES / 设置</span>
        <h1>设置</h1>
        <p>管理这台电脑上的 DesignWorkbench。</p>
      </div>
      <section className="settings-card">
        <h2>本地数据</h2>
        <p>项目存储在这台电脑的 SQLite 数据库中。应用不会自动上传你的内容。</p>
        <div className="setting-line">
          <span>数据目录</span>
          <code>
            {info?.dataDirectory ??
              (error ? "仅桌面安装版可查看" : "正在读取…")}
          </code>
        </div>
        <div className="setting-line">
          <span>当前版本</span>
          <span>{info?.version ?? "0.3.0"}</span>
        </div>
        <div className="setting-line">
          <span>连接方式</span>
          <span>离线优先</span>
        </div>
      </section>
      <section className="settings-card">
        <h2>即将接入</h2>
        <p>
          搜索、悬浮面板和备份设置将在对应阶段加入。这里不会显示无法使用的开关。
        </p>
      </section>
    </>
  );
}
