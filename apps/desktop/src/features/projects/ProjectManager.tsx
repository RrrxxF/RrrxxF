// Project actions use Rust IPC; delete asks explicitly because it hides the project from lists.
import { useState } from "react";
import { useProjects } from "./ProjectsContext";
import { ProjectDialog } from "./ProjectDialog";
import type { Project } from "../../services/projects";

export function ProjectManager() {
  const { projects, loading, error, nativeAvailable, select, archive, remove } =
    useProjects();
  const [editing, setEditing] = useState<Project | null>(null);
  const [creating, setCreating] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);

  async function run(action: () => Promise<void>) {
    setActionError(null);
    try {
      await action();
    } catch (cause) {
      setActionError(String(cause));
    }
  }

  const active = projects.filter((project) => !project.archivedAt);
  const archived = projects.filter((project) => project.archivedAt);
  return (
    <section className="project-section" aria-labelledby="projects-heading">
      <div className="section-head">
        <div>
          <span className="eyebrow">PROJECTS</span>
          <h2 id="projects-heading">你的项目</h2>
        </div>
        <button
          className="button primary"
          disabled={!nativeAvailable}
          onClick={() => setCreating(true)}
        >
          ＋ 新建项目
        </button>
      </div>
      {!nativeAvailable && (
        <p className="notice" role="status">
          浏览器预览：项目管理需要桌面安装版，浏览器中不会创建假数据。
        </p>
      )}
      {loading && (
        <p className="notice" role="status">
          正在加载本地项目…
        </p>
      )}
      {(error || actionError) && (
        <p role="alert" className="error">
          {actionError || error}
        </p>
      )}
      {!loading && active.length === 0 && (
        <div className="project-empty">
          <span>✦</span>
          <h3>从第一个项目开始</h3>
          <p>为一套设计、一位客户或一个灵感主题建立空间。</p>
        </div>
      )}
      {active.length > 0 && (
        <div className="project-grid">
          {active.map((project) => (
            <ProjectCard
              key={project.id}
              project={project}
              onSelect={() => void run(() => select(project.id))}
              onEdit={() => setEditing(project)}
              onArchive={() => void run(() => archive(project.id, true))}
              onDelete={() => {
                if (
                  window.confirm(
                    `将“${project.name}”移出项目列表？数据会保留在本地数据库中。`,
                  )
                )
                  void run(() => remove(project.id));
              }}
            />
          ))}
        </div>
      )}
      {archived.length > 0 && (
        <details className="archived-list">
          <summary>已归档项目 · {archived.length}</summary>
          {archived.map((project) => (
            <div className="archived-row" key={project.id}>
              <span>{project.name}</span>
              <button
                className="text-button"
                onClick={() => void run(() => archive(project.id, false))}
              >
                恢复
              </button>
            </div>
          ))}
        </details>
      )}
      {creating && <ProjectDialog onClose={() => setCreating(false)} />}
      {editing && (
        <ProjectDialog
          id={editing.id}
          initialName={editing.name}
          onClose={() => setEditing(null)}
        />
      )}
    </section>
  );
}

function ProjectCard({
  project,
  onSelect,
  onEdit,
  onArchive,
  onDelete,
}: {
  project: Project;
  onSelect: () => void;
  onEdit: () => void;
  onArchive: () => void;
  onDelete: () => void;
}) {
  return (
    <article className="project-card">
      <div className="project-card-top">
        <span className="project-symbol">
          {project.name.slice(0, 1).toUpperCase()}
        </span>
        <span className="project-meta">本地项目</span>
      </div>
      <h3>{project.name}</h3>
      <p>创建于 {new Date(project.createdAt).toLocaleDateString("zh-CN")}</p>
      <div className="project-actions">
        <button className="text-button" onClick={onSelect}>
          进入项目 ↗
        </button>
        <button
          className="icon-button"
          title="重命名"
          aria-label={`重命名 ${project.name}`}
          onClick={onEdit}
        >
          ✎
        </button>
        <button
          className="icon-button"
          title="归档"
          aria-label={`归档 ${project.name}`}
          onClick={onArchive}
        >
          ▱
        </button>
        <button
          className="icon-button"
          title="删除"
          aria-label={`删除 ${project.name}`}
          onClick={onDelete}
        >
          ×
        </button>
      </div>
    </article>
  );
}
