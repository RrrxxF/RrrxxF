// Typed native IPC keeps all project records in SQLite; the browser has no mock database.
import { invoke, isTauri } from "@tauri-apps/api/core";

export type Project = {
  id: string;
  name: string;
  description: string;
  color: string | null;
  archivedAt: string | null;
  createdAt: string;
  updatedAt: string;
};

export type AppInfo = { version: string; dataDirectory: string };

function native<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  if (!isTauri())
    return Promise.reject(new Error("请在桌面安装版中管理本地项目"));
  return invoke<T>(command, args);
}

export const projectApi = {
  info: () => native<AppInfo>("get_app_info"),
  list: () => native<Project[]>("list_projects"),
  selectedId: () => native<string | null>("get_selected_project_id"),
  select: (id: string) => native<void>("select_project", { id }),
  clearSelection: () => native<void>("clear_project_selection"),
  create: (name: string) => native<Project>("create_project", { name }),
  rename: (id: string, name: string) =>
    native<Project>("rename_project", { id, name }),
  archive: (id: string, archived: boolean) =>
    native<Project>("set_project_archived", { id, archived }),
  delete: (id: string) => native<void>("delete_project", { id }),
};
