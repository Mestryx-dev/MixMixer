use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam_channel::Sender;
use eframe::egui;
use tracing::{error, info};

use crate::audio::metrics::AudioMetrics;
use crate::config::Config;
use crate::devices::{enumerate_device_lists, stable_device_query, DeviceLists};
use crate::error::{Error, Result};
use crate::ui::theme::{self, Theme};
use crate::AppEvent;

pub struct SettingsLauncher {
    open: Arc<AtomicBool>,
    event_tx: Sender<AppEvent>,
    metrics: Arc<AudioMetrics>,
}

impl SettingsLauncher {
    pub fn new(event_tx: Sender<AppEvent>, metrics: Arc<AudioMetrics>) -> Self {
        Self {
            open: Arc::new(AtomicBool::new(false)),
            event_tx,
            metrics,
        }
    }

    pub fn open(&self, config_path: PathBuf, config: &Config) -> Result<()> {
        if self
            .open
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            info!("settings window already open");
            return Ok(());
        }

        let devices = enumerate_device_lists()?;
        let event_tx = self.event_tx.clone();
        let metrics = Arc::clone(&self.metrics);
        let open = Arc::clone(&self.open);
        let initial = config.clone();

        thread::Builder::new()
            .name("mix-mixer-settings".into())
            .spawn(move || {
                let result = run_settings_window(config_path, initial, devices, event_tx, metrics);
                open.store(false, Ordering::SeqCst);
                if let Err(err) = result {
                    error!(%err, "settings window failed");
                }
            })
            .map_err(|e| Error::config(format!("spawn settings thread: {e}")))?;

        Ok(())
    }
}

struct SettingsApp {
    config_path: PathBuf,
    baseline: Config,
    draft: Config,
    devices: DeviceLists,
    status: String,
    status_ok: bool,
    event_tx: Sender<AppEvent>,
    metrics: Arc<AudioMetrics>,
}

impl SettingsApp {
    fn new(
        config_path: PathBuf,
        config: Config,
        devices: DeviceLists,
        event_tx: Sender<AppEvent>,
        metrics: Arc<AudioMetrics>,
    ) -> Self {
        let baseline = normalize_draft_devices(config, &devices);

        Self {
            config_path,
            draft: baseline.clone(),
            baseline,
            devices,
            status: String::new(),
            status_ok: false,
            event_tx,
            metrics,
        }
    }

    fn apply(&mut self) {
        normalize_config_devices(&mut self.draft);
        match self.draft.save(&self.config_path) {
            Ok(()) => {
                info!(path = %self.config_path.display(), "config saved from settings");
                let applied = self.draft.clone();
                if self.event_tx.send(AppEvent::SettingsApplied(applied)).is_err() {
                    self.status = "Impossible d'envoyer les réglages au moteur audio.".into();
                    self.status_ok = false;
                    return;
                }
                self.baseline = self.draft.clone();
                self.status = "Réglages appliqués.".into();
                self.status_ok = true;
            }
            Err(err) => {
                self.status = err.to_string();
                self.status_ok = false;
            }
        }
    }

    fn cancel(&mut self) {
        self.draft = self.baseline.clone();
        self.status.clear();
        self.status_ok = false;
    }

    fn quit(&mut self, ctx: &egui::Context) {
        let _ = self.event_tx.send(AppEvent::Shutdown);
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn set_routing(&mut self, enabled: bool) {
        if self.draft.enabled == enabled {
            return;
        }
        self.draft.enabled = enabled;
        match self.draft.save(&self.config_path) {
            Ok(()) => {
                if self
                    .event_tx
                    .send(AppEvent::SetRoutingEnabled(enabled))
                    .is_err()
                {
                    self.draft.enabled = !enabled;
                    self.status =
                        "Impossible d'envoyer la commande au moteur audio.".into();
                    self.status_ok = false;
                    return;
                }
                self.baseline.enabled = enabled;
                self.status = if enabled {
                    "Routage activé.".into()
                } else {
                    "Routage désactivé.".into()
                };
                self.status_ok = true;
            }
            Err(err) => {
                self.draft.enabled = !enabled;
                self.status = err.to_string();
                self.status_ok = false;
            }
        }
    }
}

fn normalize_draft_devices(config: Config, devices: &DeviceLists) -> Config {
    let mut draft = config;
    draft.devices.voice_input =
        resolve_device_name(&devices.inputs, &draft.devices.voice_input);
    draft.devices.virtual_mic_output =
        resolve_device_name(&devices.outputs, &draft.devices.virtual_mic_output);
    draft.devices.monitor_output =
        resolve_device_name(&devices.outputs, &draft.devices.monitor_output);
    draft
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(100));

        let snap = self.metrics.snapshot();

        egui::TopBottomPanel::top("header")
            .min_height(Theme::HEADER_H)
            .frame(theme::header_frame())
            .show(ctx, |ui| {
                theme::header(ui, &snap);
            });

        egui::TopBottomPanel::bottom("footer")
            .min_height(Theme::FOOTER_H)
            .frame(theme::footer_frame())
            .show(ctx, |ui| {
                ui.set_min_height(Theme::FOOTER_H - 24.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    if theme::btn_text(ui, "Quitter") {
                        self.quit(ctx);
                    }
                    if theme::btn_secondary(ui, "Annuler") {
                        self.cancel();
                    }
                    if theme::btn_primary(ui, "Appliquer") {
                        self.apply();
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(theme::panel_frame())
            .show(ctx, |ui| {
                let scroll_h = ui.available_height();
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .max_height(scroll_h)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());

                        theme::section_header(ui, "ROUTAGE", true);
                        theme::group_box(ui, |ui| {
                            let mut enabled = self.draft.enabled;
                            theme::toggle_row(ui, "Activer le routage", true, &mut enabled);
                            if enabled != self.draft.enabled {
                                self.set_routing(enabled);
                            }
                        });

                        theme::section_header(ui, "PÉRIPHÉRIQUES", false);
                        theme::group_box(ui, |ui| {
                            theme::picker_row(
                                ui,
                                "micro",
                                "Micro",
                                true,
                                &self.devices.inputs,
                                &mut self.draft.devices.voice_input,
                            );
                            theme::picker_row(
                                ui,
                                "vac",
                                "Sortie VAC",
                                false,
                                &self.devices.outputs,
                                &mut self.draft.devices.virtual_mic_output,
                            );
                            theme::picker_row(
                                ui,
                                "monitor",
                                "Monitor casque",
                                false,
                                &self.devices.outputs,
                                &mut self.draft.devices.monitor_output,
                            );
                        });

                        theme::section_header(ui, "AUDIO", false);
                        theme::group_box(ui, |ui| {
                            theme::toggle_row(
                                ui,
                                "Écoute casque",
                                true,
                                &mut self.draft.monitor.enabled,
                            );
                            theme::slider_block(
                                ui,
                                "Gain voix",
                                false,
                                &mut self.draft.gains.voice,
                                0.0..=2.0,
                            );
                            theme::slider_block(
                                ui,
                                "Master",
                                false,
                                &mut self.draft.gains.master,
                                0.0..=2.0,
                            );
                            theme::buffer_block(ui, false, &mut self.draft.buffer_frames);
                        });
                        theme::section_footer(
                            ui,
                            "Buffer bas = moins de latence, plus de charge CPU.",
                        );

                        if !self.status.is_empty() {
                            theme::status_banner(ui, &self.status, self.status_ok);
                        }

                        ui.add_space(20.0);
                    });
            });
    }
}

fn normalize_config_devices(config: &mut Config) {
    config.devices.voice_input = stable_device_query(&config.devices.voice_input);
    config.devices.virtual_mic_output = stable_device_query(&config.devices.virtual_mic_output);
    config.devices.monitor_output = stable_device_query(&config.devices.monitor_output);
}

fn resolve_device_name(names: &[String], query: &str) -> String {
    if names.is_empty() {
        return query.to_string();
    }
    names
        .get(crate::devices::best_device_index(names, query))
        .cloned()
        .unwrap_or_else(|| query.to_string())
}

fn run_settings_window(
    config_path: PathBuf,
    config: Config,
    devices: DeviceLists,
    event_tx: Sender<AppEvent>,
    metrics: Arc<AudioMetrics>,
) -> Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([Theme::WINDOW_W, Theme::WINDOW_H])
            .with_min_inner_size([Theme::WINDOW_W, Theme::WINDOW_MIN_H])
            .with_resizable(true)
            .with_active(true),
        event_loop_builder: Some(Box::new(|builder| {
            use winit::platform::windows::EventLoopBuilderExtWindows;
            builder.with_any_thread(true);
        })),
        ..Default::default()
    };

    eframe::run_native(
        "MixMixer — Réglages",
        native_options,
        Box::new(move |cc| {
            theme::Theme::apply(&cc.egui_ctx);
            Ok(Box::new(SettingsApp::new(
                config_path.clone(),
                config.clone(),
                devices.clone(),
                event_tx,
                metrics,
            )) as Box<dyn eframe::App>)
        }),
    )
    .map_err(|e| Error::config(format!("settings window: {e}")))?;

    Ok(())
}
