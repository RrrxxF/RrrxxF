// Library hook coordinates paged queries and cancellable native imports; SQLite stays authoritative.
import { useCallback, useEffect, useRef, useState } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open } from "@tauri-apps/plugin-dialog";
import {
  assetApi,
  type Asset,
  type AssetKind,
  type Folder,
  type Tag,
} from "../../services/assets";

export function useAssetLibrary(kind: AssetKind, projectId: string | null) {
  const [assets, setAssets] = useState<Asset[]>([]);
  const [folders, setFolders] = useState<Folder[]>([]);
  const [tags, setTags] = useState<Tag[]>([]);
  const [folderId, setFolderId] = useState<string | null>(null);
  const [tag, setTag] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [favoritesOnly, setFavoritesOnly] = useState(false);
  const [loading, setLoading] = useState(false);
  const [hasMore, setHasMore] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [progress, setProgress] = useState<{
    done: number;
    total: number;
    name: string;
  } | null>(null);
  const activeJob = useRef<string | null>(null);
  const cancelled = useRef(false);
  const requestNumber = useRef(0);
  const nativeAvailable = isTauri();

  const refreshCatalog = useCallback(async () => {
    if (!nativeAvailable) return;
    try {
      const [nextFolders, nextTags] = await Promise.all([
        assetApi.folders(projectId),
        assetApi.tags(),
      ]);
      setFolders(nextFolders);
      setTags(nextTags);
    } catch (cause) {
      setError(String(cause));
    }
  }, [nativeAvailable, projectId]);

  const load = useCallback(
    async (offset = 0) => {
      if (!nativeAvailable) return;
      const request = ++requestNumber.current;
      setLoading(true);
      try {
        const next = await assetApi.list({
          kind,
          projectId,
          folderId,
          tag,
          search,
          favoritesOnly,
          offset,
        });
        if (request !== requestNumber.current) return;
        setAssets((current) => (offset === 0 ? next : [...current, ...next]));
        setHasMore(next.length === 50);
        setError(null);
      } catch (cause) {
        if (request === requestNumber.current) setError(String(cause));
      } finally {
        if (request === requestNumber.current) setLoading(false);
      }
    },
    [nativeAvailable, kind, projectId, folderId, tag, search, favoritesOnly],
  );

  useEffect(() => {
    void load();
  }, [load]);
  useEffect(() => {
    void refreshCatalog();
  }, [refreshCatalog]);
  useEffect(() => {
    setFolderId(null);
  }, [projectId]);

  const importPaths = useCallback(
    async (paths: string[]) => {
      if (!nativeAvailable || !paths.length || activeJob.current) return;
      const jobId = crypto.randomUUID();
      activeJob.current = jobId;
      cancelled.current = false;
      setError(null);
      const failures: string[] = [];
      let done = 0;
      setProgress({ done, total: paths.length, name: "准备导入…" });
      try {
        for (const path of paths) {
          if (cancelled.current) break;
          const name = path.split(/[\\/]/).pop() ?? path;
          setProgress({ done, total: paths.length, name });
          try {
            await assetApi.importOne(
              path,
              { kind, projectId, folderId },
              jobId,
            );
          } catch (cause) {
            if (cancelled.current) break;
            failures.push(`${name}：${String(cause)}`);
          }
          done += 1;
          setProgress({ done, total: paths.length, name });
        }
      } finally {
        await assetApi.finish(jobId).catch(() => undefined);
        activeJob.current = null;
        setProgress(null);
        await load();
        await refreshCatalog();
        if (failures.length)
          setError(
            `${failures.length} 个文件导入失败。${failures.slice(0, 3).join("；")}`,
          );
      }
    },
    [nativeAvailable, kind, projectId, folderId, load, refreshCatalog],
  );

  const chooseFiles = useCallback(async () => {
    if (!nativeAvailable) return;
    try {
      const selection = await open({
        multiple: true,
        directory: false,
        filters: [
          {
            name: "设计素材",
            extensions: ["png", "jpg", "jpeg", "webp", "gif", "bmp", "svg"],
          },
        ],
      });
      if (selection)
        await importPaths(Array.isArray(selection) ? selection : [selection]);
    } catch (cause) {
      setError(String(cause));
    }
  }, [nativeAvailable, importPaths]);

  useEffect(() => {
    if (!nativeAvailable) return;
    let live = true;
    let unlisten: (() => void) | undefined;
    getCurrentWebview()
      .onDragDropEvent((event) => {
        if (event.payload.type === "drop")
          void importPaths(event.payload.paths);
      })
      .then((stop) => {
        if (live) unlisten = stop;
        else stop();
      })
      .catch((cause: unknown) => setError(String(cause)));
    return () => {
      live = false;
      unlisten?.();
    };
  }, [nativeAvailable, importPaths]);

  const cancelImport = useCallback(() => {
    cancelled.current = true;
    if (activeJob.current) void assetApi.cancel(activeJob.current);
  }, []);

  useEffect(
    () => () => {
      if (activeJob.current) cancelImport();
    },
    [cancelImport],
  );

  const mutate = useCallback(
    async (action: () => Promise<unknown>) => {
      setError(null);
      try {
        await action();
        await Promise.all([load(), refreshCatalog()]);
      } catch (cause) {
        setError(String(cause));
        throw cause;
      }
    },
    [load, refreshCatalog],
  );

  return {
    assets,
    folders,
    tags,
    folderId,
    setFolderId,
    tag,
    setTag,
    search,
    setSearch,
    favoritesOnly,
    setFavoritesOnly,
    loading,
    hasMore,
    error,
    progress,
    nativeAvailable,
    refresh: () => load(),
    loadMore: () => load(assets.length),
    chooseFiles,
    importPaths,
    cancelImport,
    mutate,
  };
}
