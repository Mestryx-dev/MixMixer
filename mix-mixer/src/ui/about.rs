use std::path::Path;

use crate::i18n::Locale;

/// Native About dialog (Windows).
pub fn show_about(locale: Locale, config_path: &Path) {
    let texts = locale.texts();
    let body = format!(
        "{}\n\nVersion {}\n{}\n\nConfig:\n{}",
        texts.about_description,
        env!("CARGO_PKG_VERSION"),
        texts.about_url,
        config_path.display(),
    );
    show_message_box(texts.about_title, &body);
}

#[cfg(target_os = "windows")]
fn show_message_box(title: &str, body: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK};

    fn wide(s: &str) -> Vec<u16> {
        OsStr::new(s)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }

    let title = wide(title);
    let body = wide(body);
    unsafe {
        MessageBoxW(0, body.as_ptr(), title.as_ptr(), MB_OK | MB_ICONINFORMATION);
    }
}

#[cfg(not(target_os = "windows"))]
fn show_message_box(title: &str, body: &str) {
    eprintln!("{title}\n{body}");
}
