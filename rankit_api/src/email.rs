use std::io::Write;
use std::path::PathBuf;
use std::fs::OpenOptions;
use chrono::Utc;
use derive_more::*;
use axum::async_trait;
use regex::Regex;


/// Dynamic wrapper around an [`EmailService`].
#[derive(Deref, DerefMut)]
pub struct DynEmailService(Box<dyn EmailService>);
impl DynEmailService {
    pub fn filesystem(directory: impl Into<PathBuf>) -> Self {
        Self(Box::new(FilesystemEmailService::new(directory)))
    }
}

/// Service for sending emails.
#[async_trait]
pub trait EmailService: Send + Sync + 'static {
    async fn send(&self, recipient: String, subject: String, body: String) -> Result<(), EmailServiceError>;
}


#[derive(Error, From, Debug, Display)]
pub enum EmailServiceError {
    #[display(fmt="Invalid email")]
    InvalidEmail,
    IOError(std::io::Error),
}

/// An email service that is simulated using the local filesystem.
#[derive(Clone)]
pub struct FilesystemEmailService {
    directory: PathBuf,
    email_pattern: Regex
}

impl FilesystemEmailService {
    pub fn new(directory: impl Into<PathBuf>) -> Self {
        Self {
            directory: directory.into(),
            email_pattern: Regex::new(r"^[a-zA-Z0-9~!@#$%^&()+=\[\]_-]+@[a-z]+\.[a-z]{2,3}$").unwrap()
        }
    }
}

#[async_trait]
impl EmailService for FilesystemEmailService {
    async fn send(&self, recipient: String, subject: String, body: String) -> Result<(), EmailServiceError> {
        log::trace!("Sending email to {recipient}");
        if !self.email_pattern.is_match(&recipient) {
            return Err(EmailServiceError::InvalidEmail);
        }
        let directory = self.directory.clone();
        tokio::task::spawn_blocking(move || send_sim_email(directory, recipient, subject, body))
            .await
            .expect("Panicked on sending email")
    }
}

fn send_sim_email(directory: PathBuf, recipient: String, subject: String, body: String) -> Result<(), EmailServiceError> {

    // Creates dir structure if necessary
    std::fs::create_dir_all(&directory)?;

    // Opens file that will contain simulated email
    let now = Utc::now().format("%Y-%m-%d %H_%M_%S");
    let file_name = format!("{recipient}_{now}.txt");
    let mut file_path = PathBuf::from(directory);
    file_path.push(file_name);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;

    // Writes contents
    write!(file, "Subject: {subject}\nBody:\n{body}")?;
    Ok(())
}