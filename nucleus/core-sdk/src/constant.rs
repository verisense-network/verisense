// TODO
pub const MAX_GET_RETURN_SIZE: usize = 65536 * 1024;
pub const MAX_DELAY_SEC: u64 = 60 * 60 * 24 * 365;
pub const MAX_PARAMS_SIZE: usize = 1024 * 1024;
pub const MAX_FUNC_SIZE: usize = 1024;

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

impl core::fmt::Display for StorageError {
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
