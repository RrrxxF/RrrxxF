// Shared S2 catalog UI for icons, component snapshots and reference images.
import { useState } from "react";
import type { Asset, AssetKind } from "../../services/assets";
import { assetApi } from "../../services/assets";
import { useProjects } from "../projects/ProjectsContext";
import { AssetDetail } from "./AssetDetail";
import { AssetThumbnail } from "./AssetThumbnail";
import { useAssetLibrary } from "./useAssetLibrary";

const labels: Record<AssetKind, string> = {
  icon: "图标库",
  component: "组件库",
  image: "设计图库",
};
const descriptions: Record<AssetKind, string> = {
  icon: "保存 SVG 或图片图标；编辑功能将在 S3 接入。",
  component: "保存静态 SVG/图片组件快照；Figma 粘贴将在 S4 接入。",
  image: "保存设计参考图片，按项目和分类整理。",
};

export function AssetLibraryPage({ kind }: { kind: AssetKind }) {
  const { selectedId, projects } = useProjects();
  const selectedProject = projects.find((project) => project.id === selectedId);
  const library = useAssetLibrary(kind, selectedId);
  const [selected, setSelected] = useState<Asset | null>(null);
  const [newFolder, setNewFolder] = useState("");

  async function createFolder() {
    if (!newFolder.trim()) return;
    try {
      await library.mutate(() =>
        assetApi.createFolder(newFolder, selectedId, null),
      );
      setNewFolder("");
    } catch {
      /* The hook shows the database error. */
    }
  }

  return (
    <>
      <div className="page-intro">
        <span className="eyebrow">
          LIBRARY / {selectedProject?.name ?? "全部资源"}
        </span>
        <h1>{labels[kind]}</h1>
        <p>{descriptions[kind]}</p>
      </div>
      <section className="asset-panel" aria-label={`${labels[kind]}素材`}>
        <div className="asset-toolbar">
          <label className="sr-only" htmlFor="asset-search">
            搜索素材
          </label>
          <input
            id="asset-search"
            className="asset-search"
            type="search"
            value={library.search}
            onChange={(event) => library.setSearch(event.target.value)}
            placeholder="搜索名称、文件名或标签…"
          />
          <button
            className="button primary"
            disabled={!library.nativeAvailable || !!library.progress}
            onClick={() => void library.chooseFiles()}
          >
            ＋ 导入文件
          </button>
        </div>
        <div className="asset-filters">
          <label>
            文件夹{" "}
            <select
              value={library.folderId ?? ""}
              onChange={(event) =>
                library.setFolderId(event.target.value || null)
              }
            >
              <option value="">全部文件夹</option>
              {library.folders.map((folder) => (
                <option key={folder.id} value={folder.id}>
                  {folder.name}
                </option>
              ))}
            </select>
          </label>
          <label>
            标签{" "}
            <select
              value={library.tag ?? ""}
              onChange={(event) => library.setTag(event.target.value || null)}
            >
              <option value="">全部标签</option>
              {library.tags.map((tag) => (
                <option key={tag.id} value={tag.label}>
                  {tag.label} ({tag.assetCount})
                </option>
              ))}
            </select>
          </label>
          <label className="favorite-filter">
            <input
              type="checkbox"
              checked={library.favoritesOnly}
              onChange={(event) =>
                library.setFavoritesOnly(event.target.checked)
              }
            />
            仅看收藏
          </label>
        </div>
        <div className="folder-create">
          <input
            value={newFolder}
            maxLength={80}
            onChange={(event) => setNewFolder(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter") void createFolder();
            }}
            placeholder="新建文件夹名称"
            aria-label="新建文件夹名称"
          />
          <button
            className="text-button"
            disabled={!library.nativeAvailable || !newFolder.trim()}
            onClick={() => void createFolder()}
          >
            ＋ 新建文件夹
          </button>
        </div>
        {!library.nativeAvailable && (
          <p className="notice" role="status">
            浏览器预览：导入和本地素材仅在桌面安装版中可用。
          </p>
        )}
        {library.error && (
          <p className="error" role="alert">
            {library.error}
          </p>
        )}
        {library.progress && (
          <div className="import-progress" role="status">
            <div>
              <strong>
                正在导入 {library.progress.done} / {library.progress.total}
              </strong>
              <span>{library.progress.name}</span>
            </div>
            <progress
              value={library.progress.done}
              max={library.progress.total}
            />
            <button className="button secondary" onClick={library.cancelImport}>
              取消
            </button>
          </div>
        )}
        {library.loading && library.assets.length === 0 && (
          <p className="notice">正在读取本地素材…</p>
        )}
        {library.assets.length === 0 && !library.loading && (
          <div className="library-empty">
            <span className="empty-symbol" aria-hidden="true">
              {kind === "icon" ? "◇" : kind === "component" ? "▣" : "▧"}
            </span>
            <h2>
              {library.search ||
              library.folderId ||
              library.tag ||
              library.favoritesOnly
                ? "没有匹配的素材"
                : `${labels[kind]}还是空的`}
            </h2>
            <p>选择文件或直接拖入窗口，素材会保存在你的电脑里。</p>
            <button
              className="button secondary"
              disabled={!library.nativeAvailable}
              onClick={() => void library.chooseFiles()}
            >
              选择文件
            </button>
          </div>
        )}
        {library.assets.length > 0 && (
          <div className="asset-grid">
            {library.assets.map((asset) => (
              <button
                className="asset-card"
                key={asset.id}
                onClick={() => setSelected(asset)}
              >
                <AssetThumbnail asset={asset} />
                <div className="asset-card-info">
                  <strong title={asset.name}>{asset.name}</strong>
                  <span>
                    {asset.favorite ? "★  " : ""}
                    {asset.mime.replace("image/", "").toUpperCase()} ·{" "}
                    {(asset.byteSize / 1024).toFixed(1)} KiB
                  </span>
                </div>
              </button>
            ))}
          </div>
        )}
        {library.hasMore && (
          <div className="load-more">
            <button
              className="button secondary"
              disabled={library.loading}
              onClick={() => void library.loadMore()}
            >
              {library.loading ? "加载中…" : "加载更多"}
            </button>
          </div>
        )}
      </section>
      {selected && (
        <AssetDetail
          asset={selected}
          folders={library.folders}
          onClose={() => setSelected(null)}
          onChanged={() => library.refresh().then(() => undefined)}
        />
      )}
    </>
  );
}
