use std::env;
use std::path::PathBuf;

use anyhow::anyhow;
use axum_typed_multipart::FieldData;
use tempfile::{NamedTempFile, TempDir};
use tokio::sync::OnceCell;

static TEMP_DIR: OnceCell<PathBuf> = OnceCell::const_new();

async fn get_temp_dir_location() -> anyhow::Result<&'static PathBuf> {
    let path_buf = TEMP_DIR.get_or_try_init(init_temp_dir_location).await?;
    Ok(path_buf)
}

async fn init_temp_dir_location() -> anyhow::Result<PathBuf> {
    let temp_dir = env::current_dir()?.join(".tmp");
    tokio::fs::create_dir_all(&temp_dir).await?;
    Ok(temp_dir)
}

pub async fn group_temp_file_fields(
    file_fields: Vec<FieldData<NamedTempFile>>,
) -> anyhow::Result<TempDir> {
    let temp_dir = get_temp_dir_location().await?;
    let dir = TempDir::new_in(temp_dir.as_path())?;

    for file_field in file_fields {
        let filename = match file_field.metadata.file_name {
            Some(filename) => filename,
            None => return Err(anyhow!("file must have a filename")),
        };

        let path = dir.path().join(filename);
        tokio::fs::copy(file_field.contents.path(), path).await?;
    }

    Ok(dir)
}

pub async fn load_temp_file_fields_sorted(
    file_fields: Vec<FieldData<NamedTempFile>>,
) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut raw_files_with_name = Vec::with_capacity(file_fields.len());

    for file_field in file_fields {
        let filename = file_field.metadata.file_name.unwrap_or_default();

        let bytes = tokio::fs::read(file_field.contents.path()).await?;

        raw_files_with_name.push((filename, bytes));
    }

    raw_files_with_name.sort_by(|(filename1, _), (filename2, _)| filename1.cmp(filename2));
    let sorted_raw_files = raw_files_with_name.into_iter().map(|(_, bytes)| bytes).collect();
    Ok(sorted_raw_files)
}
