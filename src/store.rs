use std::{path::PathBuf, error::Error};
use std::sync::Arc;
use regex::Regex;
use derive_more::{From, Display, Deref, DerefMut};
use axum::async_trait;

/// CRUD interface for arbitrary file storage.
#[async_trait]
pub trait FileStore: Send + Sync + 'static {

    /// Creates a file with the supplied bytes.
    async fn create(
        &self,
        directory: Option<&str>,
        name: &str,
        bytes: &[u8]
    ) -> Result<(), FileStoreError>;

    /// Deletes an existing file.
    async fn delete(
        &self,
        directory: Option<&str>,
        name: &str
    ) -> Result<(), FileStoreError>;
}

/// Helpful dynamic wrapper around a [`FileStore`].
#[derive(Clone, Deref, DerefMut)]
pub struct DynFileStore(Arc<dyn FileStore>);
impl DynFileStore {
    pub fn new(store: impl FileStore) -> Self {
        Self(Arc::new(store))
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
            filename_pattern: Regex::new("[a-zA-Z0-9_]*").unwrap()
        }
    }
}

#[async_trait]
impl FileStore for FilesystemFileStore {

    async fn create(&self, directory: Option<&str>, name: &str, bytes: &[u8]) -> Result<(), FileStoreError> {
        if self.filename_pattern.is_match(name) {
            return Err(FileStoreError::InvalidFileName(name.into()));
        }
        let mut path = PathBuf::from(&self.root);
        if let Some(directory) = directory {
            path.push(directory);
        }
        async_fs::create_dir_all(&path).await?;
        path.push(name);
        std::fs::write(path, bytes)?;
        Ok(())
    }

    async fn delete(&self, directory: Option<&str>, name: &str) -> Result<(), FileStoreError> {
        if self.filename_pattern.is_match(name) {
            return Err(FileStoreError::InvalidFileName(name.into()));
        }
        let mut path = PathBuf::from(&self.root);
        if let Some(directory) = directory {
            path.push(directory);
        }
        path.push(name);
        async_fs::remove_file(path).await?;
        Ok(())
    }
}