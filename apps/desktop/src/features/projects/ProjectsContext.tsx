// Shared project state is refreshed from SQLite after each mutation; selection is also stored there.
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import type { ReactNode } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { projectApi, type Project } from "../../services/projects";

type ProjectsState = {
  projects: Project[];
  selectedId: string | null;
  loading: boolean;
  error: string | null;
  nativeAvailable: boolean;
  refresh: () => Promise<void>;
  select: (id: string) => Promise<void>;
  clearSelection: () => Promise<void>;
  create: (name: string) => Promise<void>;
  rename: (id: string, name: string) => Promise<void>;
  archive: (id: string, archived: boolean) => Promise<void>;
  remove: (id: string) => Promise<void>;
};

const Context = createContext<ProjectsState | null>(null);

export function ProjectsProvider({ children }: { children: ReactNode }) {
  const nativeAvailable = isTauri();
  const [projects, setProjects] = useState<Project[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [loading, setLoading] = useState(nativeAvailable);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    if (!nativeAvailable) return;
    try {
      const [items, savedId] = await Promise.all([
        projectApi.list(),
        projectApi.selectedId(),
      ]);
      setProjects(items);
      setSelectedId(
        savedId && items.some((item) => item.id === savedId && !item.archivedAt)
          ? savedId
          : null,
      );
      setError(null);
    } catch (cause) {
      setError(String(cause));
    } finally {
      setLoading(false);
    }
  }, [nativeAvailable]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const mutate = useCallback(
    async (action: () => Promise<unknown>) => {
      setError(null);
      try {
        await action();
        await refresh();
      } catch (cause) {
        setError(String(cause));
        throw cause;
      }
    },
    [refresh],
  );

  const value = useMemo<ProjectsState>(
    () => ({
      projects,
      selectedId,
      loading,
      error,
      nativeAvailable,
      refresh,
      select: (id) => mutate(() => projectApi.select(id)),
      clearSelection: () => mutate(() => projectApi.clearSelection()),
      create: (name) =>
        mutate(async () => {
          const item = await projectApi.create(name);
          await projectApi.select(item.id);
        }),
      rename: (id, name) => mutate(() => projectApi.rename(id, name)),
      archive: (id, archived) => mutate(() => projectApi.archive(id, archived)),
      remove: (id) => mutate(() => projectApi.delete(id)),
    }),
    [projects, selectedId, loading, error, nativeAvailable, refresh, mutate],
  );

  return <Context.Provider value={value}>{children}</Context.Provider>;
}

export function useProjects(): ProjectsState {
  const state = useContext(Context);
  if (!state) throw new Error("ProjectsProvider is missing");
  return state;
}
