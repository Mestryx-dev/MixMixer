use crossbeam_channel::Sender;
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};

use crate::error::{Error, Result};
use crate::AppEvent;

#[derive(Debug, Clone, Copy)]
pub enum TrayAction {
    OpenSettings,
    Quit,
    ToggleMonitor,
    ReloadConfig,
}

pub struct TrayManager {
    _tray: TrayIcon,
    event_tx: Sender<AppEvent>,
}

impl TrayManager {
    pub fn new(event_tx: Sender<AppEvent>) -> Result<Self> {
        let menu = Menu::new();
        let settings_item = MenuItem::new("Réglages...", true, None);
        let monitor_item = MenuItem::new("Activer/désactiver écoute", true, None);
        let reload_item = MenuItem::new("Recharger config", true, None);
        let quit_item = MenuItem::new("Quitter", true, None);

        menu.append(&settings_item)
            .map_err(|e| Error::Tray(format!("menu append: {e}")))?;
        menu.append(&PredefinedMenuItem::separator())
            .map_err(|e| Error::Tray(format!("menu append: {e}")))?;
        menu.append(&monitor_item)
            .map_err(|e| Error::Tray(format!("menu append: {e}")))?;
        menu.append(&reload_item)
            .map_err(|e| Error::Tray(format!("menu append: {e}")))?;
        menu.append(&PredefinedMenuItem::separator())
            .map_err(|e| Error::Tray(format!("menu append: {e}")))?;
        menu.append(&quit_item)
            .map_err(|e| Error::Tray(format!("menu append: {e}")))?;

        let icon = tray_icon::Icon::from_rgba(simple_tray_rgba(), 16, 16)
            .map_err(|e| Error::Tray(format!("icon: {e}")))?;

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("MixMixer — double-clic : Réglages")
            .with_icon(icon)
            .build()
            .map_err(|e| Error::Tray(format!("tray build: {e}")))?;

        let tray_id = tray.id().clone();

        let event_tx_clone = event_tx.clone();
        let settings_id = settings_item.id().clone();
        let monitor_id = monitor_item.id().clone();
        let reload_id = reload_item.id().clone();
        let quit_id = quit_item.id().clone();

        std::thread::spawn(move || {
            let menu_channel = MenuEvent::receiver();
            let tray_channel = TrayIconEvent::receiver();
            loop {
                if let Ok(event) = tray_channel.try_recv() {
                    match event {
                        TrayIconEvent::DoubleClick { id, button, .. }
                            if id == tray_id && button == MouseButton::Left =>
                        {
                            let _ =
                                event_tx_clone.send(AppEvent::Tray(TrayAction::OpenSettings));
                        }
                        TrayIconEvent::Click {
                            id,
                            button,
                            button_state,
                            ..
                        } if id == tray_id
                            && button == MouseButton::Left
                            && button_state == MouseButtonState::Up =>
                        {
                            let _ =
                                event_tx_clone.send(AppEvent::Tray(TrayAction::OpenSettings));
                        }
                        _ => {}
                    }
                }

                if let Ok(event) = menu_channel.try_recv() {
                    if event.id == settings_id {
                        let _ = event_tx_clone.send(AppEvent::Tray(TrayAction::OpenSettings));
                    } else if event.id == monitor_id {
                        let _ = event_tx_clone.send(AppEvent::Tray(TrayAction::ToggleMonitor));
                    } else if event.id == reload_id {
                        let _ = event_tx_clone.send(AppEvent::Tray(TrayAction::ReloadConfig));
                    } else if event.id == quit_id {
                        let _ = event_tx_clone.send(AppEvent::Tray(TrayAction::Quit));
                        break;
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });

        Ok(Self {
            _tray: tray,
            event_tx,
        })
    }

    pub fn poll(&mut self) -> Result<()> {
        let _ = &self.event_tx;
        Ok(())
    }
}

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
