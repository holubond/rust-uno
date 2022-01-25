use gloo_console::log;
use gloo_storage::Storage;
use serde::Serialize;

pub fn set<T>(key: impl AsRef<str>, value: T)
where
    T: Serialize,
{
    if gloo_storage::LocalStorage::set(key, value).is_ok() {
        return;
    }

    let window = match web_sys::window() {
        None => panic!("Failed attempt to call web_sys::window() in local_storage_set()"),
        Some(x) => x,
    };

    if window.alert_with_message("Local storage error").is_err() {
        log!("Alert failed to pop up!")
    }
}
