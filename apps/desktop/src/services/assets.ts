// Typed IPC for the S2 asset catalog; files and SQL remain inside the Rust boundary.
import { native } from "./projects";

export type AssetKind = "icon" | "component" | "image";
export type Asset = {
  id: string;
  name: string;
  kind: AssetKind;
  originalFilename: string;
  mime: string;
  byteSize: number;
  width: number | null;
  height: number | null;
  sha256: string;
  hasThumbnail: boolean;
  favorite: boolean;
  projectIds: string[];
  folderIds: string[];
  tags: string[];
  createdAt: string;
};
export type Folder = {
  id: string;
  name: string;
  projectId: string | null;
  parentId: string | null;
};
export type Tag = { id: string; label: string; assetCount: number };
export type AssetFilter = {
  kind: AssetKind;
  projectId: string | null;
  folderId: string | null;
  tag: string | null;
  search: string;
  favoritesOnly: boolean;
  offset: number;
};
export type ImportOptions = {
  kind: AssetKind;
  projectId: string | null;
  folderId: string | null;
};

export const assetApi = {
  list: (filter: AssetFilter) => native<Asset[]>("list_assets", { filter }),
  importOne: (path: string, options: ImportOptions, jobId: string) =>
    native<Asset>("import_asset", { path, options, jobId }),
  cancel: (jobId: string) => native<void>("cancel_import", { jobId }),
  finish: (jobId: string) => native<void>("finish_import", { jobId }),
  thumbnail: (id: string) =>
    native<number[] | null>("get_asset_thumbnail", { id }),
  rename: (id: string, name: string) =>
    native<Asset>("rename_asset", { id, name }),
  remove: (id: string) => native<void>("delete_asset", { id }),
  favorite: (id: string, favorite: boolean) =>
    native<Asset>("set_asset_favorite", { id, favorite }),
  linkProject: (assetId: string, projectId: string, linked: boolean) =>
    native<Asset>("link_asset_project", { assetId, projectId, linked }),
  setTags: (assetId: string, labels: string[]) =>
    native<Asset>("set_asset_tags", { assetId, labels }),
  createFolder: (
    name: string,
    projectId: string | null,
    parentId: string | null,
  ) => native<Folder>("create_folder", { name, projectId, parentId }),
  folders: (projectId: string | null) =>
    native<Folder[]>("list_folders", { projectId }),
  setFolder: (assetId: string, folderId: string, linked: boolean) =>
    native<Asset>("set_asset_folder", { assetId, folderId, linked }),
  tags: () => native<Tag[]>("list_tags"),
};
