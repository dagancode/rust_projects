use std::sync::{RwLock, RwLockReadGuard};

use tracing::warn;

#[deprecated]
pub fn read_lock_handler<'a, T>(read_lock: &'a RwLock<Vec<T>>) -> RwLockReadGuard<'a, Vec<T>> {
    let guard = match read_lock.read() {
        Ok(lock) => lock,
        Err(poison_error) => {
            let lock = poison_error.into_inner();
            warn!(
                "Lock was poisoned - recovering from last stable state ({} items)",
                lock.len()
            );

            lock
        }
    };
    guard
}
