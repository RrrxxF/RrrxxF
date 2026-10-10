//! Staged, content-addressed media writes; no source filename becomes a storage path.

use crate::CoreError;
use image::{DynamicImage, ImageFormat, ImageReader};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

const MAX_FILE_BYTES: u64 = 100 * 1024 * 1024;
const MAX_PIXELS: u64 = 40_000_000;

pub(crate) struct StagedMedia {
    pub(crate) file: NamedTempFile,
    pub(crate) sha256: String,
    pub(crate) byte_size: u64,
    pub(crate) mime: &'static str,
    pub(crate) dimensions: Option<(u32, u32)>,
    pub(crate) thumbnail: Option<Vec<u8>>,
}

struct InspectedMedia {
    mime: &'static str,
    dimensions: Option<(u32, u32)>,
    thumbnail: Option<Vec<u8>>,
}

pub(crate) fn stage(
    source: &Path,
    root: &Path,
    cancelled: &impl Fn() -> bool,
    available_override: Option<u64>,
) -> Result<StagedMedia, CoreError> {
    let metadata = fs::metadata(source)?;
    if !metadata.is_file() {
        return Err(CoreError::UnsupportedFormat);
    }
    if metadata.len() > MAX_FILE_BYTES {
        return Err(CoreError::FileTooLarge);
    }
    let staging = root.join("import-staging");
    fs::create_dir_all(&staging)?;
    let available = available_override.unwrap_or(fs2::available_space(&staging)?);
    if metadata.len().saturating_add(1024 * 1024) > available {
        return Err(CoreError::InsufficientSpace);
    }

    let mut input = File::open(source)?;
    let mut file = NamedTempFile::new_in(&staging)?;
    let mut hasher = Sha256::new();
    let mut byte_size = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    loop {
        if cancelled() {
            return Err(CoreError::Cancelled);
        }
        let count = input.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        byte_size += count as u64;
        if byte_size > MAX_FILE_BYTES {
            return Err(CoreError::FileTooLarge);
        }
        file.write_all(&buffer[..count]).map_err(map_write_error)?;
        hasher.update(&buffer[..count]);
    }
    file.flush().map_err(map_write_error)?;
    if cancelled() {
        return Err(CoreError::Cancelled);
    }
    let sha256 = format!("{:x}", hasher.finalize());
    let inspected = inspect(file.path())?;
    if cancelled() {
        return Err(CoreError::Cancelled);
    }
    Ok(StagedMedia {
        file,
        sha256,
        byte_size,
        mime: inspected.mime,
        dimensions: inspected.dimensions,
        thumbnail: inspected.thumbnail,
    })
}

fn inspect(path: &Path) -> Result<InspectedMedia, CoreError> {
    let mut prefix = [0u8; 512];
    let count = File::open(path)?.read(&mut prefix)?;
    let text_start = String::from_utf8_lossy(&prefix[..count]);
    if text_start
        .trim_start_matches('\u{feff}')
        .trim_start()
        .starts_with('<')
    {
        if fs::metadata(path)?.len() > 10 * 1024 * 1024 {
            return Err(CoreError::UnsupportedFormat);
        }
        let xml = fs::read_to_string(path).map_err(|_| CoreError::UnsupportedFormat)?;
        let lowercase = xml.to_ascii_lowercase();
        if lowercase.contains("<!doctype") || lowercase.contains("<!entity") {
            return Err(CoreError::UnsupportedFormat);
        }
        let document =
            roxmltree::Document::parse(&xml).map_err(|_| CoreError::UnsupportedFormat)?;
        if document.root_element().tag_name().name() != "svg" {
            return Err(CoreError::UnsupportedFormat);
        }
        // SVG remains stored as inert source data in S2; S3 will sanitize before rendering.
        return Ok(InspectedMedia {
            mime: "image/svg+xml",
            dimensions: None,
            thumbnail: None,
        });
    }

    let format = ImageReader::open(path)?
        .with_guessed_format()?
        .format()
        .ok_or(CoreError::UnsupportedFormat)?;
    let mime = match format {
        ImageFormat::Png => "image/png",
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::WebP => "image/webp",
        ImageFormat::Gif => "image/gif",
        ImageFormat::Bmp => "image/bmp",
        _ => return Err(CoreError::UnsupportedFormat),
    };
    let (width, height) = ImageReader::open(path)?
        .with_guessed_format()?
        .into_dimensions()
        .map_err(|_| CoreError::UnsupportedFormat)?;
    if width == 0 || height == 0 || u64::from(width) * u64::from(height) > MAX_PIXELS {
        return Err(CoreError::UnsupportedFormat);
    }
    let image = ImageReader::open(path)?
        .with_guessed_format()?
        .decode()
        .map_err(|_| CoreError::UnsupportedFormat)?;
    let thumb: DynamicImage = image.thumbnail(320, 320);
    let mut data = Cursor::new(Vec::new());
    thumb
        .write_to(&mut data, ImageFormat::WebP)
        .map_err(|_| CoreError::UnsupportedFormat)?;
    Ok(InspectedMedia {
        mime,
        dimensions: Some((width, height)),
        thumbnail: Some(data.into_inner()),
    })
}

pub(crate) fn commit_file(
    staged: &StagedMedia,
    root: &Path,
) -> Result<(PathBuf, bool, Option<PathBuf>), CoreError> {
    // Cache the thumbnail first so a thumbnail failure cannot orphan a newly linked blob.
    let thumbnail_path = staged
        .thumbnail
        .as_ref()
        .map(|_| thumbnail_path(root, &staged.sha256));
    if let (Some(bytes), Some(path)) = (&staged.thumbnail, &thumbnail_path) {
        fs::create_dir_all(path.parent().expect("thumbnail always has a parent"))?;
        let mut temporary = NamedTempFile::new_in(path.parent().expect("thumbnail parent"))?;
        temporary.write_all(bytes).map_err(map_write_error)?;
        match temporary.persist_noclobber(path) {
            Ok(_) => (),
            Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => (),
            Err(error) => return Err(error.error.into()),
        }
    }
    let blob = blob_path(root, &staged.sha256);
    fs::create_dir_all(blob.parent().expect("blob always has a parent"))?;
    let new_blob = match fs::hard_link(staged.file.path(), &blob) {
        Ok(()) => true,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => false,
        Err(error) => return Err(error.into()),
    };
    Ok((blob, new_blob, thumbnail_path))
}

pub(crate) fn blob_path(root: &Path, sha256: &str) -> PathBuf {
    root.join("media/blobs").join(&sha256[..2]).join(sha256)
}

pub(crate) fn thumbnail_path(root: &Path, sha256: &str) -> PathBuf {
    root.join("cache/thumbs").join(format!("{sha256}_320.webp"))
}

fn map_write_error(error: std::io::Error) -> CoreError {
    if error.kind() == std::io::ErrorKind::StorageFull {
        CoreError::InsufficientSpace
    } else {
        CoreError::Io(error)
    }
}
