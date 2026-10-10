// Each library has its own route and an honest empty state until its asset phase arrives.
import { useNavigate } from "react-router-dom";
import { sections } from "../navigation";
import { useProjects } from "../../features/projects/ProjectsContext";

export function LibraryPage({
  section,
}: {
  section: (typeof sections)[number];
}) {
  const navigate = useNavigate();
  const { selectedId, projects } = useProjects();
  const selected = projects.find((item) => item.id === selectedId);
  return (
    <>
      <div className="page-intro">
        <span className="eyebrow">
          LIBRARY / {selected?.name ?? "全部资源"}
        </span>
        <h1>{section.label}</h1>
        <p>你的本地设计资产，按自己的方式整理。</p>
      </div>
      <section className="library-panel">
        <div className="library-toolbar">
          <span>全部内容</span>
          <span className="count-pill">0 项</span>
        </div>
        <div className="library-empty">
          <span className="empty-symbol" aria-hidden="true">
            {section.icon}
          </span>
          <h2>{section.title}</h2>
          <p>{section.description}</p>
          <button className="button secondary" onClick={() => navigate("/")}>
            返回工作台
          </button>
        </div>
      </section>
    </>
  );
}
