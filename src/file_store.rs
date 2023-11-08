use std::{path::PathBuf, error::Error};
use regex::Regex;
use derive_more::{From, Display, Deref, DerefMut};
use axum::async_trait;

/// CRUD interface for arbitrary file storage.
#[async_trait]
pub trait FileStore: Send + Sync + 'static {
    async fn create(&self, path: &str, bytes: &[u8]) -> Result<(), FileStoreError>;
    async fn delete(&self, path: &str) -> Result<(), FileStoreError>;
}

/// Helpful dynamic wrapper around a [`FileStore`].
#[derive(Deref, DerefMut)]
pub struct DynFileStore(Box<dyn FileStore>);
impl DynFileStore {
    pub fn new(store: impl FileStore) -> Self {
        Self(Box::new(store))
    }
}

/// Error that can occur when using [`FileStore`].
#[derive(From, Display, Debug)]
pub enum FileStoreError {
    IOError(std::io::Error),
    #[from(ignore)]
    InvalidFileName(String)
}
impl Error for FileStoreError {}

/// Implementation of [`FileStore`] that uses the local filesystem.
/// Useful for local testing, though should not to be used in production systems.
pub struct FilesystemFileStore {
    root: PathBuf,
    filename_pattern: Regex
}
impl FilesystemFileStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            filename_pattern: Regex::new(r"^([a-zA-Z0-9_]+\/)*[a-zA-Z0-9_]+(\.[a-zA-Z0-9_]+)?$").unwrap()
        }
    }
}

#[async_trait]
impl FileStore for FilesystemFileStore {

    async fn create(&self, path: &str, bytes: &[u8]) -> Result<(), FileStoreError> {
        if !self.filename_pattern.is_match(path) {
            return Err(FileStoreError::InvalidFileName(path.into()));
        }
        let mut full_path = PathBuf::from(&self.root);
        full_path.push(path);
        async_fs::create_dir_all(&full_path.parent().unwrap()).await?;
        std::fs::write(full_path, bytes)?;
        Ok(())
    }

    async fn delete(&self, path: &str) -> Result<(), FileStoreError> {
        if !self.filename_pattern.is_match(path) {
            return Err(FileStoreError::InvalidFileName(path.into()));
        }
        let mut full_path = PathBuf::from(&self.root);
        full_path.push(path);
        async_fs::remove_file(full_path).await?;
        Ok(())
    }
}