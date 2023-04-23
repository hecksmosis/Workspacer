use std::{fmt, error::Error};

pub struct CommandReturn(pub Result<(), WorkspaceError>);

impl From<WorkspaceError> for CommandReturn {
    fn from(error: WorkspaceError) -> Self {
        CommandReturn(Err(error))
    }
}

impl From<String> for CommandReturn {
    fn from(message: String) -> Self {
        CommandReturn(Err(WorkspaceError::new(message, Severity::Error)))
    }
}

impl From<&str> for CommandReturn {
    fn from(message: &str) -> Self {
        CommandReturn(Err(WorkspaceError::new(String::from(message), Severity::Error)))
    }
}

impl From<()> for CommandReturn {
    fn from(_: ()) -> Self {
        CommandReturn(Ok(()))
    }
}

impl Into<Result<(), WorkspaceError>> for CommandReturn {
    fn into(self) -> Result<(), WorkspaceError> {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Message
}

#[derive(Debug)]
pub struct WorkspaceError {
    pub message: String,
    pub severity: Severity,
}

impl WorkspaceError {
    pub fn new(message: String, severity: Severity) -> Self {
        Self {
            message,
            severity,
        }
    }
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WorkspaceError {
    fn description(&self) -> &str {
        &self.message
    }
}
