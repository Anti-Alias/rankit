use std::{path::PathBuf, error::Error};
use regex::Regex;
use derive_more::{From, Display, Deref, DerefMut};
use axum::async_trait;

/// Helpful dynamic wrapper around a [`FileStore`].
#[derive(Deref, DerefMut)]
pub struct DynFileStore(Box<dyn FileStore>);
impl DynFileStore {
    pub fn filesystem(root: impl Into<PathBuf>) -> Self {
        Self(Box::new(FilesystemFileStore::new(root)))
    }
}

/// A simple interface for file storage.
#[async_trait]
pub trait FileStore: Send + Sync + 'static {
    async fn create(&self, path: &str, bytes: &[u8]) -> Result<(), FileStoreError>;
    async fn delete(&self, path: &str) -> Result<(), FileStoreError>;
}

#[derive(From, Display, Debug)]
pub enum FileStoreError {
    IOError(std::io::Error),
    #[from(ignore)]
    InvalidFileName(String)
}
impl Error for FileStoreError {}

/// Implementation of [`FileStore`] that uses the local filesystem.
/// Useful for testing, though should not to be used in production systems.
pub struct FilesystemFileStore {
    root: PathBuf,
    file_pattern: Regex,
}
impl FilesystemFileStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            file_pattern: Regex::new(r"^([a-zA-Z0-9_]+\/)*[a-zA-Z0-9_]+(\.[a-zA-Z0-9_]+)?$").unwrap()
        }
    }
}

#[async_trait]
impl FileStore for FilesystemFileStore {

    async fn create(&self, file: &str, bytes: &[u8]) -> Result<(), FileStoreError> {
        log::trace!("Creating file {file}");
        if !self.file_pattern.is_match(file) {
            return Err(FileStoreError::InvalidFileName(file.into()));
        }
        let mut full_path = PathBuf::from(&self.root);
        full_path.push(file);
        async_fs::create_dir_all(&full_path.parent().unwrap()).await?;
        async_fs::write(full_path, bytes).await?;
        Ok(())
    }

    async fn delete(&self, file: &str) -> Result<(), FileStoreError> {
        log::trace!("Deleting file {file}");
        if !self.file_pattern.is_match(file) {
            return Err(FileStoreError::InvalidFileName(file.into()));
        }
        let mut full_path = PathBuf::from(&self.root);
        full_path.push(file);
        async_fs::remove_file(full_path).await?;
        Ok(())
    }
}