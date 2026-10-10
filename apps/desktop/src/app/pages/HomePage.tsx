// The workbench summarises actual projects and leaves later asset metrics unclaimed.
import { ProjectManager } from "../../features/projects/ProjectManager";
import { useProjects } from "../../features/projects/ProjectsContext";

export function HomePage() {
  const { projects, selectedId } = useProjects();
  const selected = projects.find((item) => item.id === selectedId);
  return (
    <>
      <div className="page-intro">
        <span className="eyebrow">WORKSPACE / 工作台</span>
        <h1>
          {selected ? `${selected.name}，继续创作。` : "让每个灵感，找到归处。"}
        </h1>
        <p>把设计资源安放在自己的电脑里，按项目有序整理。</p>
      </div>
      <div className="summary-grid">
        <div className="summary-card">
          <span>项目总数</span>
          <strong>
            {projects
              .filter((item) => !item.archivedAt)
              .length.toString()
              .padStart(2, "0")}
          </strong>
          <small>在这个本地空间里</small>
        </div>
        <div className="summary-card muted">
          <span>图标与组件</span>
          <strong>—</strong>
          <small>将在后续阶段接入</small>
        </div>
        <div className="summary-card muted">
          <span>设计图库</span>
          <strong>—</strong>
          <small>将在后续阶段接入</small>
        </div>
      </div>
      <ProjectManager />
    </>
  );
}
