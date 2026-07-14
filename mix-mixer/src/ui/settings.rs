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
use crate::devices::{best_device_index, enumerate_device_lists, stable_device_query, DeviceLists};
use crate::error::{Error, Result};
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

    fn toggle_routing(&mut self) {
        self.draft.enabled = !self.draft.enabled;
        match self.draft.save(&self.config_path) {
            Ok(()) => {
                let enabled = self.draft.enabled;
                if self.event_tx.send(AppEvent::SetRoutingEnabled(enabled)).is_err() {
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
                self.draft.enabled = !self.draft.enabled;
                self.status = err.to_string();
                self.status_ok = false;
            }
        }
    }
}

fn normalize_draft_devices(config: Config, devices: &DeviceLists) -> Config {
    let mut draft = config;
    draft.devices.voice_input = resolve_device_name(&devices.inputs, &draft.devices.voice_input);
    draft.devices.virtual_mic_output =
        resolve_device_name(&devices.outputs, &draft.devices.virtual_mic_output);
    draft.devices.monitor_output =
        resolve_device_name(&devices.outputs, &draft.devices.monitor_output);
    draft
}

fn show_metrics_overlay(ctx: &egui::Context, metrics: &AudioMetrics) {
    let snap = metrics.snapshot();

    egui::Area::new(egui::Id::new("metrics_overlay"))
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -10.0))
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(egui::Color32::from_black_alpha(200))
                .inner_margin(8.0)
                .rounding(4.0)
                .show(ui, |ui| {
                    ui.set_min_width(120.0);
                    ui.label(
                        egui::RichText::new(format!("Délai ~ {:.1} ms", snap.estimated_latency_ms))
                            .monospace()
                            .size(12.0),
                    );
                    ui.label(
                        egui::RichText::new(format!("Buffer {:.0}%", snap.voice_buffer_pct))
                            .monospace()
                            .size(12.0),
                    );
                    let route_color = if snap.routing_live {
                        egui::Color32::LIGHT_GREEN
                    } else if snap.reconnect_pending {
                        egui::Color32::LIGHT_YELLOW
                    } else {
                        egui::Color32::GRAY
                    };
                    let route_label = if snap.routing_live {
                        "Audio actif"
                    } else if snap.reconnect_pending {
                        "Reconnexion…"
                    } else {
                        "Audio off"
                    };
                    ui.label(
                        egui::RichText::new(format!("{route_label} · {} flux", snap.streams_active))
                            .monospace()
                            .size(12.0)
                            .color(route_color),
                    );
                });
        });
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(100));

        egui::CentralPanel::default().show(ctx, |ui| {
            device_combo(
                ui,
                "Micro (entrée)",
                &self.devices.inputs,
                &mut self.draft.devices.voice_input,
            );
            device_combo(
                ui,
                "Sortie VAC (CABLE Input)",
                &self.devices.outputs,
                &mut self.draft.devices.virtual_mic_output,
            );
            device_combo(
                ui,
                "Monitor casque (optionnel)",
                &self.devices.outputs,
                &mut self.draft.devices.monitor_output,
            );

            ui.add_space(12.0);
            ui.checkbox(
                &mut self.draft.monitor.enabled,
                "Écoute casque (monitor)",
            );

            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.label("Gain voix");
                ui.add(
                    egui::Slider::new(&mut self.draft.gains.voice, 0.0..=2.0).fixed_decimals(2),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Master");
                ui.add(
                    egui::Slider::new(&mut self.draft.gains.master, 0.0..=2.0).fixed_decimals(2),
                );
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Buffer (latence)");
                ui.add(
                    egui::Slider::new(&mut self.draft.buffer_frames, 128..=512)
                        .step_by(128.0)
                        .logarithmic(false),
                );
            });

            if !self.status.is_empty() {
                ui.add_space(8.0);
                let color = if self.status_ok {
                    egui::Color32::LIGHT_GREEN
                } else {
                    egui::Color32::LIGHT_RED
                };
                ui.colored_label(color, &self.status);
            }

            ui.add_space(12.0);
            ui.horizontal(|ui| {
                let routing_label = if self.draft.enabled {
                    "Désactiver"
                } else {
                    "Activer"
                };
                if ui.button(routing_label).clicked() {
                    self.toggle_routing();
                }
            });

            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if ui.button("Appliquer").clicked() {
                    self.apply();
                }
                if ui.button("Annuler").clicked() {
                    self.cancel();
                }
                if ui.button("Quitter").clicked() {
                    self.quit(ctx);
                }
            });
        });

        show_metrics_overlay(ctx, &self.metrics);
    }
}

fn normalize_config_devices(config: &mut Config) {
    config.devices.voice_input = stable_device_query(&config.devices.voice_input);
    config.devices.virtual_mic_output = stable_device_query(&config.devices.virtual_mic_output);
    config.devices.monitor_output = stable_device_query(&config.devices.monitor_output);
}

fn device_combo(ui: &mut egui::Ui, label: &str, names: &[String], selected: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        if names.is_empty() {
            ui.text_edit_singleline(selected);
            return;
        }

        let display = if names.iter().any(|name| name == selected) {
            selected.clone()
        } else {
            names
                .get(best_device_index(names, selected))
                .cloned()
                .unwrap_or_else(|| selected.clone())
        };

        egui::ComboBox::from_id_salt(label)
            .selected_text(display)
            .show_ui(ui, |ui| {
                for name in names {
                    ui.selectable_value(selected, name.clone(), name);
                }
            });
    });
}

fn resolve_device_name(names: &[String], query: &str) -> String {
    if names.is_empty() {
        return query.to_string();
    }
    names
        .get(best_device_index(names, query))
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
            .with_inner_size([480.0, 380.0])
            .with_min_inner_size([400.0, 320.0])
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
        Box::new(move |_cc| {
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
