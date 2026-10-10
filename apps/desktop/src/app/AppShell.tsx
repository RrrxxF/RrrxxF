// Shared chrome exposes six routes and a persistent project switcher on every page.
import { useState } from "react";
import { NavLink, Outlet, useNavigate } from "react-router-dom";
import { sections } from "./navigation";
import { useProjects } from "../features/projects/ProjectsContext";
import { ProjectDialog } from "../features/projects/ProjectDialog";

export function AppShell() {
  const navigate = useNavigate();
  const { projects, selectedId, select, clearSelection, nativeAvailable } =
    useProjects();
  const [collapsed, setCollapsed] = useState(false);
  const [creating, setCreating] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const selected = projects.find((project) => project.id === selectedId);

  async function changeProject(id: string) {
    try {
      if (id === "all") await clearSelection();
      else await select(id);
      setMessage(null);
      navigate("/");
    } catch (cause) {
      setMessage(String(cause));
    }
  }

  return (
    <div className={`app-shell${collapsed ? " collapsed" : ""}`}>
      <aside className="sidebar" aria-label="主导航">
        <div className="brand">
          <span className="brand-mark">
            D<span>.</span>
          </span>
          <div className="brand-copy">
            <strong>DesignWorkbench</strong>
            <small>个人设计资产工作台</small>
          </div>
        </div>
        <button
          className="collapse-button"
          onClick={() => setCollapsed(!collapsed)}
          aria-label={collapsed ? "展开侧栏" : "收起侧栏"}
          title={collapsed ? "展开侧栏" : "收起侧栏"}
        >
          {collapsed ? "»" : "«"}
        </button>
        <div className="nav-caption">工作空间</div>
        <nav>
          {sections.map((item) => (
            <NavLink
              key={item.path}
              to={item.path}
              end={item.path === "/"}
              title={item.label}
              className={({ isActive }) =>
                `nav-link${isActive ? " active" : ""}`
              }
            >
              <span className="nav-icon" aria-hidden="true">
                {item.icon}
              </span>
              <span className="nav-label">{item.label}</span>
            </NavLink>
          ))}
        </nav>
        <div className="sidebar-bottom">
          <NavLink
            to="/settings"
            title="设置"
            className={({ isActive }) => `nav-link${isActive ? " active" : ""}`}
          >
            <span className="nav-icon">⚙</span>
            <span className="nav-label">设置</span>
          </NavLink>
          <div className="local-note">
            <span className="online-dot" /> <span>本地优先 · 无需登录</span>
          </div>
        </div>
      </aside>
      <div className="main-column">
        <header className="app-header">
          <div className="header-context">
            <span className="header-caption">当前空间</span>
            <label className="sr-only" htmlFor="project-switcher">
              切换项目
            </label>
            <select
              id="project-switcher"
              value={selected?.id ?? "all"}
              disabled={!nativeAvailable}
              onChange={(event) => void changeProject(event.target.value)}
            >
              <option value="all">全部资源</option>
              {projects
                .filter((project) => !project.archivedAt)
                .map((project) => (
                  <option value={project.id} key={project.id}>
                    {project.name}
                  </option>
                ))}
            </select>
          </div>
          <div className="header-actions">
            <span className="header-status">
              <span className="online-dot" /> 私有本地空间
            </span>
            <button
              className="header-new"
              disabled={!nativeAvailable}
              onClick={() => setCreating(true)}
            >
              ＋ 新建项目
            </button>
          </div>
        </header>
        {message && (
          <p role="alert" className="error top-error">
            {message}
          </p>
        )}
        <main className="content">
          <Outlet />
        </main>
      </div>
      {creating && <ProjectDialog onClose={() => setCreating(false)} />}
    </div>
  );
}
