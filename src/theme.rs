use dioxus::signals::{GlobalSignal, Readable};

use crate::storage::{use_persistent, UsePersistent};

pub(crate) static IS_DARK: GlobalSignal<UsePersistent<bool>> =
    GlobalSignal::new(|| use_dark(false));

pub(crate) fn use_dark(init: bool) -> UsePersistent<bool> {
    use_persistent("dark-theme", || init)
}

pub(crate) fn toggle(is_dark: bool) {
    if is_dark {
        js_sys::eval("document.documentElement.classList.add('dark');")
            .expect("Error toggling dark mode");
    } else {
        js_sys::eval("document.documentElement.classList.remove('dark');")
            .expect("Error toggling light mode");
    }
}

pub(crate) fn init() {
    let state = IS_DARK.cloned().get();
    toggle(state)
}
