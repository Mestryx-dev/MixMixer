use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crossbeam_channel::Sender;
use egui::Context;
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};

use crate::error::{Error, Result};
use crate::i18n::Locale;
use crate::AppEvent;

#[derive(Debug, Clone, Copy)]
pub enum TrayAction {
    OpenSettings,
    About,
    Quit,
}

/// Tray icon owned by the egui/winit thread (required on Windows).
pub struct TrayHandle {
    _tray: TrayIcon,
    about_id: tray_icon::menu::MenuId,
    quit_id: tray_icon::menu::MenuId,
}

impl TrayHandle {
    pub fn new(
        locale: Locale,
        ui_ctx: Arc<Mutex<Option<Context>>>,
        hidden_to_tray: Arc<AtomicBool>,
    ) -> Result<Self> {
        let texts = locale.texts();
        let menu = Menu::new();
        let about_item = MenuItem::new(texts.tray_about, true, None);
        let quit_item = MenuItem::new(texts.tray_quit, true, None);

        menu.append(&about_item)
            .map_err(|e| Error::Tray(format!("menu append: {e}")))?;
        menu.append(&PredefinedMenuItem::separator())
            .map_err(|e| Error::Tray(format!("menu append: {e}")))?;
        menu.append(&quit_item)
            .map_err(|e| Error::Tray(format!("menu append: {e}")))?;

        let icon = tray_icon::Icon::from_rgba(simple_tray_rgba(), 16, 16)
            .map_err(|e| Error::Tray(format!("icon: {e}")))?;

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_menu_on_left_click(false)
            .with_tooltip(texts.tray_tooltip)
            .with_icon(icon)
            .build()
            .map_err(|e| Error::Tray(format!("tray build: {e}")))?;

        let tray_id_for_handler = tray.id().clone();
        let ui_ctx_for_handler = Arc::clone(&ui_ctx);
        let hidden_for_handler = Arc::clone(&hidden_to_tray);

        TrayIconEvent::set_event_handler(Some(move |event| {
            let opens = match &event {
                TrayIconEvent::DoubleClick {
                    id,
                    button: MouseButton::Left,
                    ..
                } => id == &tray_id_for_handler,
                TrayIconEvent::Click {
                    id,
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => id == &tray_id_for_handler,
                _ => false,
            };
            if opens {
                hidden_for_handler.store(false, Ordering::Release);
                if let Ok(guard) = ui_ctx_for_handler.lock() {
                    if let Some(ctx) = guard.as_ref() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                        ctx.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(
                            egui::UserAttentionType::Informational,
                        ));
                        ctx.request_repaint();
                    }
                }
            }
        }));

        Ok(Self {
            about_id: about_item.id().clone(),
            quit_id: quit_item.id().clone(),
            _tray: tray,
        })
    }

    pub fn poll(&self, event_tx: &Sender<AppEvent>) {
        while let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == self.about_id {
                let _ = event_tx.send(AppEvent::Tray(TrayAction::About));
            } else if event.id == self.quit_id {
                let _ = event_tx.send(AppEvent::Tray(TrayAction::Quit));
            }
        }
    }
}

/// Simple 16×16 tray icon (blue square with inner highlight).
fn simple_tray_rgba() -> Vec<u8> {
    let mut rgba = vec![0u8; 16 * 16 * 4];
    for y in 0..16 {
        for x in 0..16 {
            let i = (y * 16 + x) * 4;
            rgba[i] = 79;
            rgba[i + 1] = 140;
            rgba[i + 2] = 255;
            rgba[i + 3] = if (4..12).contains(&x) && (4..12).contains(&y) {
                255
            } else if (3..13).contains(&x) && (3..13).contains(&y) {
                180
            } else {
                0
            };
        }
    }
    rgba
}
