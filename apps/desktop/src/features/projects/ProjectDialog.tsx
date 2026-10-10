// One dialog handles create and rename with the same client-side validation as Rust.
import { useState } from "react";
import type { FormEvent } from "react";
import { useProjects } from "./ProjectsContext";

export function ProjectDialog({
  initialName,
  id,
  onClose,
}: {
  initialName?: string;
  id?: string;
  onClose: () => void;
}) {
  const { create, rename } = useProjects();
  const [name, setName] = useState(initialName ?? "");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const valid =
    name.trim().length > 0 &&
    [...name.trim()].length <= 80 &&
    ![...name].some(
      (character) =>
        character.charCodeAt(0) < 32 || character.charCodeAt(0) === 127,
    );

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (!valid) return;
    setBusy(true);
    try {
      if (id) await rename(id, name);
      else await create(name);
      onClose();
    } catch (cause) {
      setError(String(cause));
      setBusy(false);
    }
  }

  return (
    <div className="modal-backdrop" onMouseDown={onClose}>
      <section
        className="modal"
        role="dialog"
        aria-modal="true"
        aria-labelledby="project-dialog-title"
        onMouseDown={(event) => event.stopPropagation()}
      >
        <div className="modal-head">
          <h2 id="project-dialog-title">{id ? "重命名项目" : "新建项目"}</h2>
          <button className="icon-button" onClick={onClose} aria-label="关闭">
            ×
          </button>
        </div>
        <form onSubmit={(event) => void submit(event)}>
          <label htmlFor="project-name">项目名称</label>
          <input
            id="project-name"
            autoFocus
            maxLength={80}
            value={name}
            onChange={(event) => setName(event.target.value)}
            placeholder="例如：品牌视觉升级"
          />
          <p className="field-hint">
            1–80 个字符。项目和素材仅保存在你的电脑。
          </p>
          {error && (
            <p role="alert" className="error">
              {error}
            </p>
          )}
          <div className="modal-actions">
            <button
              type="button"
              className="button secondary"
              onClick={onClose}
            >
              取消
            </button>
            <button className="button primary" disabled={!valid || busy}>
              {busy ? "保存中…" : "保存项目"}
            </button>
          </div>
        </form>
      </section>
    </div>
  );
}
