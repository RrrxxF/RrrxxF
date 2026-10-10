-- S2 keeps physical content separate from logical assets and project links.
CREATE TABLE blobs (
  sha256 TEXT PRIMARY KEY CHECK (length(sha256) = 64),
  rel_path TEXT NOT NULL UNIQUE,
  mime TEXT NOT NULL,
  byte_size INTEGER NOT NULL CHECK (byte_size >= 0),
  thumbnail_rel_path TEXT,
  width INTEGER,
  height INTEGER,
  created_at TEXT NOT NULL
);

CREATE TABLE assets (
  id TEXT PRIMARY KEY,
  blob_sha256 TEXT NOT NULL REFERENCES blobs(sha256),
  kind TEXT NOT NULL CHECK (kind IN ('icon', 'component', 'image')),
  name TEXT NOT NULL CHECK (length(trim(name)) BETWEEN 1 AND 120),
  original_filename TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  deleted_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE project_assets (
  project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
  PRIMARY KEY (project_id, asset_id)
);

CREATE TABLE folders (
  id TEXT PRIMARY KEY,
  project_id TEXT REFERENCES projects(id) ON DELETE CASCADE,
  parent_id TEXT REFERENCES folders(id) ON DELETE CASCADE,
  name TEXT NOT NULL CHECK (length(trim(name)) BETWEEN 1 AND 80),
  created_at TEXT NOT NULL,
  CHECK (id <> parent_id)
);

CREATE TABLE asset_folders (
  asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
  folder_id TEXT NOT NULL REFERENCES folders(id) ON DELETE CASCADE,
  PRIMARY KEY (asset_id, folder_id)
);

CREATE TABLE tags (
  id TEXT PRIMARY KEY,
  label TEXT NOT NULL,
  normalized_label TEXT NOT NULL UNIQUE
);

CREATE TABLE asset_tags (
  asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
  tag_id TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY (asset_id, tag_id)
);

CREATE TABLE favorites (
  asset_id TEXT PRIMARY KEY REFERENCES assets(id) ON DELETE CASCADE,
  created_at TEXT NOT NULL
);

CREATE INDEX assets_kind_created ON assets(deleted_at, kind, created_at DESC);
CREATE INDEX assets_blob ON assets(blob_sha256);
CREATE INDEX project_assets_asset ON project_assets(asset_id);
CREATE INDEX asset_folders_folder ON asset_folders(folder_id, asset_id);
CREATE INDEX asset_tags_tag ON asset_tags(tag_id, asset_id);
CREATE INDEX folders_project ON folders(project_id, parent_id, name);
