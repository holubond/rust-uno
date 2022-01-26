use gloo_storage::Storage;
use serde::Serialize;

use super::alert::alert;

pub fn set<T>(key: impl AsRef<str>, value: T)
where
    T: Serialize,
{
    if gloo_storage::LocalStorage::set(key, value).is_ok() {
        return;
    }

    alert("Local storage error");
}
