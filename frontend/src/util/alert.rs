pub fn alert(message: &str) {
    let window = match web_sys::window() {
        None => panic!("web_sys::window not available"),
        Some(w) => w,
    };

    if window.alert_with_message(message).is_err() {
        gloo_console::log!("Alert failed to pop up! Message: ", message);
    }
}