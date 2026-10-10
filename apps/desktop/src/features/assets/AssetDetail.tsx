// Detail controls update one persisted asset at a time and expose explicit project/folder links.
import { useState } from "react";
import type { Asset, Folder } from "../../services/assets";
import { assetApi } from "../../services/assets";
import { useProjects } from "../projects/ProjectsContext";
import { AssetThumbnail } from "./AssetThumbnail";

export function AssetDetail({
  asset: initial,
  folders,
  onClose,
  onChanged,
}: {
  asset: Asset;
  folders: Folder[];
  onClose: () => void;
  onChanged: () => Promise<void>;
}) {
  const { projects } = useProjects();
  const [asset, setAsset] = useState(initial);
  const [name, setName] = useState(initial.name);
  const [tags, setTags] = useState(initial.tags.join(", "));
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function update(action: () => Promise<Asset>) {
    setBusy(true);
    setError(null);
    try {
      const next = await action();
      setAsset(next);
      await onChanged();
    } catch (cause) {
      setError(String(cause));
    } finally {
      setBusy(false);
    }
  }

  async function remove() {
    if (!window.confirm(`将“${asset.name}”移出素材列表？原文件会保留。`))
      return;
    setBusy(true);
    try {
      await assetApi.remove(asset.id);
      await onChanged();
      onClose();
    } catch (cause) {
      setError(String(cause));
      setBusy(false);
    }
  }

  return (
    <div className="modal-backdrop" onMouseDown={onClose}>
      <section
        className="modal asset-detail"
        role="dialog"
        aria-modal="true"
        aria-labelledby="asset-detail-title"
        onMouseDown={(event) => event.stopPropagation()}
      >
        <div className="modal-head">
          <h2 id="asset-detail-title">素材详情</h2>
          <button className="icon-button" onClick={onClose} aria-label="关闭">
            ×
          </button>
        </div>
        <AssetThumbnail asset={asset} />
        <div className="asset-fields">
          <label htmlFor="asset-name">名称</label>
          <div className="inline-field">
            <input
              id="asset-name"
              value={name}
              maxLength={120}
              onChange={(event) => setName(event.target.value)}
            />
            <button
              className="button secondary"
              disabled={busy || !name.trim()}
              onClick={() => void update(() => assetApi.rename(asset.id, name))}
            >
              保存
            </button>
          </div>
          <label htmlFor="asset-tags">标签（逗号分隔）</label>
          <div className="inline-field">
            <input
              id="asset-tags"
              value={tags}
              onChange={(event) => setTags(event.target.value)}
              placeholder="例如：按钮, 深色"
            />
            <button
              className="button secondary"
              disabled={busy}
              onClick={() =>
                void update(() =>
                  assetApi.setTags(
                    asset.id,
                    tags
                      .split(/[,，]/)
                      .map((label) => label.trim())
                      .filter(Boolean),
                  ),
                )
              }
            >
              保存
            </button>
          </div>
          <p>
            来源：{asset.originalFilename} · {asset.mime} ·{" "}
            {(asset.byteSize / 1024).toFixed(1)} KiB
          </p>
          <p>
            内容 SHA-256：<code>{asset.sha256}</code>
          </p>
          <div className="detail-links">
            <strong>关联项目</strong>
            {projects
              .filter((project) => !project.archivedAt)
              .map((project) => (
                <label key={project.id}>
                  <input
                    type="checkbox"
                    checked={asset.projectIds.includes(project.id)}
                    disabled={busy}
                    onChange={(event) =>
                      void update(() =>
                        assetApi.linkProject(
                          asset.id,
                          project.id,
                          event.target.checked,
                        ),
                      )
                    }
                  />
                  {project.name}
                </label>
              ))}
          </div>
          <div className="detail-links">
            <strong>分类文件夹</strong>
            {folders.map((folder) => (
              <label key={folder.id}>
                <input
                  type="checkbox"
                  checked={asset.folderIds.includes(folder.id)}
                  disabled={busy}
                  onChange={(event) =>
                    void update(() =>
                      assetApi.setFolder(
                        asset.id,
                        folder.id,
                        event.target.checked,
                      ),
                    )
                  }
                />
                {folder.name}
              </label>
            ))}
          </div>
        </div>
        {error && (
          <p className="error" role="alert">
            {error}
          </p>
        )}
        <div className="detail-actions">
          <button
            className="button secondary"
            disabled={busy}
            onClick={() =>
              void update(() => assetApi.favorite(asset.id, !asset.favorite))
            }
          >
            {asset.favorite ? "★ 已收藏" : "☆ 加入收藏"}
          </button>
          <button
            className="text-button danger"
            disabled={busy}
            onClick={() => void remove()}
          >
            移出素材列表
          </button>
        </div>
      </section>
    </div>
  );
}
