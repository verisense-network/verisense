//! Storage module provides a set of functions to interact with the kv storage.
//! Any data stored in the kv storage will be synchronized across all nodes in the subnet.
//! The nucleus will automatically update the state root after each modification and then
//! submit to Verisense chain.
//!
//! The `put` and `del` can only be called in the post functions. Otherwise, it will case panic.

use crate::{error::RuntimeError, CallResult};
use codec::Decode;

#[link(wasm_import_module = "env")]
extern "C" {
    fn storage_put(
        key_ptr: *const u8,
        key_len: i32,
        value_ptr: *const u8,
        value_len: i32,
        return_ptr: *mut u8,
    ) -> i32;

    fn storage_del(key_ptr: *const u8, key_len: i32, return_ptr: *mut u8) -> i32;

    fn storage_get(k_ptr: *const u8, k_len: i32, return_ptr: *mut u8, v_offset: i32) -> i32;

    fn storage_get_prefix(
        k_ptr: *const u8,
        k_len: i32,
        direction: i32,
        return_ptr: *mut u8,
        v_offset: i32,
    ) -> i32;

    fn storage_get_range(
        k_ptr: *const u8,
        k_len: i32,
        direction: i32,
        limit: i32,
        return_ptr: *mut u8,
        v_offset: i32,
    ) -> i32;

    fn storage_del_range(
        s0_ptr: *const u8,
        s0_len: i32,
        s1_ptr: *const u8,
        s1_len: i32,
        return_ptr: *mut u8,
    ) -> i32;
}

/// Put a key-value pair into the kvdb.
pub fn put(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> CallResult<()> {
    let key = key.as_ref();
    let value = value.as_ref();
    assert!(key.len() <= i32::MAX as usize);
    assert!(value.len() <= i32::MAX as usize);
    let mut buf = crate::allocate_buffer();
    let status = unsafe {
        storage_put(
            key.as_ptr(),
            key.len() as i32,
            value.as_ptr(),
            value.len() as i32,
            buf.as_mut_ptr(),
        )
    };
    assert!(status == crate::NO_MORE_DATA);
    CallResult::<()>::decode(&mut &buf[..]).map_err(|_| RuntimeError::DecodeReturnValueError)?
}

/// Delete a key-value pair from the kvdb.
pub fn del(key: impl AsRef<[u8]>) -> CallResult<()> {
    let key = key.as_ref();
    assert!(key.len() <= i32::MAX as usize);
    let mut buf = crate::allocate_buffer();
    let status = unsafe { storage_del(key.as_ptr(), key.len() as i32, buf.as_mut_ptr()) };
    assert!(status == crate::NO_MORE_DATA);
    CallResult::<()>::decode(&mut &buf[..]).map_err(|_| RuntimeError::DecodeReturnValueError)?
}

/// Get a value from the kvdb with the key.
pub fn get(key: impl AsRef<[u8]>) -> CallResult<Option<Vec<u8>>> {
    let key = key.as_ref();
    assert!(key.len() <= i32::MAX as usize);
    let mut buf = crate::allocate_buffer();
    let mut val = vec![];
    loop {
        let status = unsafe {
            storage_get(
                key.as_ptr(),
                key.len() as i32,
                buf.as_mut_ptr(),
                val.len() as i32,
            )
        };
        val.extend_from_slice(&buf);
        if status == crate::NO_MORE_DATA {
            break;
        }
    }
    CallResult::<Option<Vec<u8>>>::decode(&mut &val[..])
        .map_err(|_| RuntimeError::DecodeReturnValueError)?
}

/// The direction of the search
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Forward,
    Reverse,
}

impl Into<i32> for Direction {
    fn into(self) -> i32 {
        match self {
            Direction::Forward => 0,
            Direction::Reverse => 1,
        }
    }
}

impl From<i32> for Direction {
    fn from(v: i32) -> Self {
        match v {
            0 => Direction::Forward,
            1 => Direction::Reverse,
            _ => unreachable!(),
        }
    }
}

/// Get a batch of entries from the databass with "start_key" and direction , the limit maximum is 1000
pub fn get_range(
    start_key: impl AsRef<[u8]>,
    direction: Direction,
    limit: usize,
) -> CallResult<Vec<(Vec<u8>, Vec<u8>)>> {
    let start = start_key.as_ref();
    assert!(start.len() <= i32::MAX as usize);
    assert!(limit <= 1000);
    let mut buf = crate::allocate_buffer();
    let mut val = vec![];
    loop {
        let status = unsafe {
            storage_get_range(
                start.as_ptr(),
                start.len() as i32,
                direction.into(),
                limit as i32,
                buf.as_mut_ptr(),
                val.len() as i32,
            )
        };
        val.extend_from_slice(&buf);
        if status == crate::NO_MORE_DATA {
            break;
        }
    }
    CallResult::<Vec<(Vec<u8>, Vec<u8>)>>::decode(&mut &val[..])
        .map_err(|_| RuntimeError::DecodeReturnValueError)?
}

/// Search a key-value with a prefix and direction
///
/// # Examples
///
///  ```
///  use vrs_core_sdk::storage::{search, Direction};
///
///  pub fn search_blog_id() {
///     let key = [&b"blog:"[..], &0u64.to_be_bytes()[..]].concat();
///     let first_blog = search(&key, Direction::Forward).unwrap();
///     let key = [&b"blog:"[..], &u64::MAX.to_be_bytes()[..]].concat();
///     let last_blog = search(&key, Direction::Reverse).unwrap();
///     assert!(first_blog.is_some());
///     assert!(last_blog.is_some());
///  }
///  ```
pub fn search(
    key_prefix: impl AsRef<[u8]>,
    direction: Direction,
) -> CallResult<Option<(Vec<u8>, Vec<u8>)>> {
    let key = key_prefix.as_ref();
    assert!(key.len() <= i32::MAX as usize);
    let mut buf = crate::allocate_buffer();
    let mut val = vec![];
    loop {
        let status = unsafe {
            storage_get_prefix(
                key.as_ptr(),
                key.len() as i32,
                direction.into(),
                buf.as_mut_ptr(),
                val.len() as i32,
            )
        };
        val.extend_from_slice(&buf);
        if status == crate::NO_MORE_DATA {
            break;
        }
    }
    CallResult::<Option<(Vec<u8>, Vec<u8>)>>::decode(&mut &val[..])
        .map_err(|_| RuntimeError::DecodeReturnValueError)?
}

/// Removes the database entries in the range [start_key, end_key)
pub fn delete_range(start_key: impl AsRef<[u8]>, end_key: impl AsRef<[u8]>) -> CallResult<()> {
    let start = start_key.as_ref();
    let end = end_key.as_ref();
    assert!(start.len() <= i32::MAX as usize);
    assert!(end.len() <= i32::MAX as usize);
    let mut buf = crate::allocate_buffer();
    let status = unsafe {
        storage_del_range(
            start.as_ptr(),
            start.len() as i32,
            end.as_ptr(),
            end.len() as i32,
            buf.as_mut_ptr(),
        )
    };
    assert!(status == crate::NO_MORE_DATA);
    CallResult::<()>::decode(&mut &buf[..]).map_err(|_| RuntimeError::DecodeReturnValueError)?
}
