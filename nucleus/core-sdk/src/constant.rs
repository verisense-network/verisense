use std::fmt::Display;

pub const MAX_GET_RETURN_SIZE: usize = 65536 * 1024;
#[derive(Debug, Clone, Copy)]
pub enum StorageError {
    CannotPutInGetMethod,
    MemoryAccessOutOfBounds,
    DatabaseError,
    KeyNotFound,
    UnknownError,
}
impl From<i32> for StorageError {
    fn from(e: i32) -> Self {
        match e {
            1 => StorageError::CannotPutInGetMethod,
            2 => StorageError::MemoryAccessOutOfBounds,
            3 => StorageError::DatabaseError,
            4 => StorageError::KeyNotFound,
            _ => StorageError::UnknownError,
        }
    }
}
impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::CannotPutInGetMethod => write!(f, "Operation not allowed in GET method"),
            StorageError::MemoryAccessOutOfBounds => write!(f, "Memory access out of bounds"),
            StorageError::DatabaseError => write!(f, "Database error"),
            StorageError::KeyNotFound => write!(f, "Key not found"),
            StorageError::UnknownError => write!(f, "Unknown error"),
        }
    }
}
