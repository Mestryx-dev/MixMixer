use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use eframe::egui;
use tracing::{error, info};

use crate::audio::metrics::AudioMetrics;
use crate::config::Config;
use crate::devices::{enumerate_device_lists, stable_device_query, DeviceLists};
use crate::error::{Error, Result};
use crate::i18n::Locale;
use crate::ui::theme::{self, Theme};
use crate::ui::tray::TrayHandle;
use crate::AppEvent;

enum SettingsCommand {
    Show(Config),
    Hide,
    Shutdown,
}

struct Toast {
    message: String,
    ok: bool,
    at: Instant,
}

pub struct SettingsLauncher {
    cmd_tx: Mutex<Option<Sender<SettingsCommand>>>,
    running: Arc<AtomicBool>,
    ui_ctx: Arc<Mutex<Option<egui::Context>>>,
    event_tx: Sender<AppEvent>,
    metrics: Arc<AudioMetrics>,
}

impl SettingsLauncher {
    pub fn new(event_tx: Sender<AppEvent>, metrics: Arc<AudioMetrics>) -> Self {
        Self {
            cmd_tx: Mutex::new(None),
            running: Arc::new(AtomicBool::new(false)),
            ui_ctx: Arc::new(Mutex::new(None)),
            event_tx,
            metrics,
        }
    }

    fn raise_window(&self) {
        if let Ok(guard) = self.ui_ctx.lock() {
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

    pub fn open(&self, config_path: PathBuf, config: &Config) -> Result<()> {
        if !self.running.load(Ordering::Acquire) {
            let (cmd_tx, cmd_rx) = crossbeam_channel::bounded(8);
            *self
                .cmd_tx
                .lock()
                .map_err(|e| Error::config(format!("settings channel lock: {e}")))? =
                Some(cmd_tx.clone());

            let event_tx = self.event_tx.clone();
            let metrics = Arc::clone(&self.metrics);
            let running = Arc::clone(&self.running);
            let ui_ctx = Arc::clone(&self.ui_ctx);
            let initial = config.clone();

            self.running.store(true, Ordering::Release);

            thread::Builder::new()
                .name("mix-mixer-settings".into())
                .spawn(move || {
                    let result = run_settings_window(
                        config_path,
                        initial,
                        cmd_rx,
                        event_tx,
                        metrics,
                        ui_ctx,
                    );
                    running.store(false, Ordering::Release);
                    if let Err(err) = result {
                        error!(%err, "settings window failed");
                    }
                })
                .map_err(|e| Error::config(format!("spawn settings thread: {e}")))?;
        }

        let tx = self
            .cmd_tx
            .lock()
            .map_err(|e| Error::config(format!("settings channel lock: {e}")))?
            .clone()
            .ok_or_else(|| Error::config("settings channel not ready"))?;
        tx.send(SettingsCommand::Show(config.clone()))
            .map_err(|e| Error::config(format!("settings show command: {e}")))?;

        // Wake the window from the main thread — the egui loop may idle while hidden.
        self.raise_window();

        Ok(())
    }
}

struct SettingsApp {
    config_path: PathBuf,
    baseline: Config,
    draft: Config,
    devices: DeviceLists,
    toast: Option<Toast>,
    event_tx: Sender<AppEvent>,
    metrics: Arc<AudioMetrics>,
    window_height: f32,
    last_pixels_per_point: f32,
    cmd_rx: Receiver<SettingsCommand>,
    ui_ctx: Arc<Mutex<Option<egui::Context>>>,
    tray: TrayHandle,
    hidden_to_tray: Arc<AtomicBool>,
    last_minimized: bool,
}

impl SettingsApp {
    fn new(
        config_path: PathBuf,
        config: Config,
        devices: DeviceLists,
        event_tx: Sender<AppEvent>,
        metrics: Arc<AudioMetrics>,
        cmd_rx: Receiver<SettingsCommand>,
        ui_ctx: Arc<Mutex<Option<egui::Context>>>,
        tray: TrayHandle,
        hidden_to_tray: Arc<AtomicBool>,
    ) -> Self {
        let baseline = normalize_draft_devices(config, &devices);

        Self {
            config_path,
            draft: baseline.clone(),
            baseline,
            devices,
            toast: None,
            event_tx,
            metrics,
            window_height: Theme::window_height(),
            last_pixels_per_point: 0.0,
            cmd_rx,
            ui_ctx,
            tray,
            hidden_to_tray,
            last_minimized: false,
        }
    }

    fn store_ctx(&self, ctx: &egui::Context) {
        if let Ok(mut guard) = self.ui_ctx.lock() {
            *guard = Some(ctx.clone());
        }
    }

    fn texts(&self) -> &'static crate::i18n::UiText {
        self.draft.locale.texts()
    }

    fn show_toast(&mut self, message: impl Into<String>, ok: bool) {
        self.toast = Some(Toast {
            message: message.into(),
            ok,
            at: Instant::now(),
        });
    }

    fn poll_toast(&mut self, ctx: &egui::Context) {
        let Some(toast) = &self.toast else {
            return;
        };
        let age = toast.at.elapsed().as_secs_f32();
        let total = Theme::TOAST_DURATION_SECS + Theme::TOAST_FADE_SECS;
        if age >= total {
            self.toast = None;
            return;
        }
        theme::toast(ctx, &toast.message, toast.ok, age);
        ctx.request_repaint();
    }

    /// Keep a fixed logical size across monitors / DPI changes / accidental maximize.
    fn sync_window_size(&mut self, ctx: &egui::Context) {
        let desired = egui::vec2(Theme::WINDOW_W, Theme::window_height());
        let (maximized, inner_size, ppp) = ctx.input(|i| {
            let vp = i.viewport();
            (
                vp.maximized == Some(true),
                vp.inner_rect.map(|r| r.size()),
                i.pixels_per_point(),
            )
        });

        let ppp_changed = self.last_pixels_per_point > 0.0
            && (self.last_pixels_per_point - ppp).abs() > 0.01;
        self.last_pixels_per_point = ppp;

        let size_wrong = match inner_size {
            Some(size) => {
                (size.x - desired.x).abs() > 1.5 || (size.y - desired.y).abs() > 1.5
            }
            None => false,
        };
        let theme_changed = (self.window_height - desired.y).abs() > 0.5;

        if maximized {
            // Fixed dialog: never stay maximized (broken restore on mixed-DPI setups).
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(false));
        }

        if maximized || ppp_changed || size_wrong || theme_changed {
            self.window_height = desired.y;
            ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::EnableButtons {
                close: true,
                minimized: true,
                maximize: false,
            });
            ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(desired));
            ctx.send_viewport_cmd(egui::ViewportCommand::MaxInnerSize(desired));
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(desired));
        }
    }

    fn hide_to_tray(&mut self, ctx: &egui::Context) {
        self.hidden_to_tray.store(true, Ordering::Release);
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }

    fn show_window(&mut self, ctx: &egui::Context, config: Config) {
        self.hidden_to_tray.store(false, Ordering::Release);
        self.baseline = normalize_draft_devices(config, &self.devices);
        self.draft = self.baseline.clone();
        self.toast = None;
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        ctx.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(
            egui::UserAttentionType::Informational,
        ));
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(
            self.texts().window_title.to_string(),
        ));
        self.sync_window_size(ctx);
        ctx.request_repaint();
    }

    fn poll_commands(&mut self, ctx: &egui::Context) {
        while let Ok(cmd) = self.cmd_rx.try_recv() {
            match cmd {
                SettingsCommand::Show(config) => {
                    self.show_window(ctx, config);
                }
                SettingsCommand::Hide => self.hide_to_tray(ctx),
                SettingsCommand::Shutdown => {
                    let _ = self.event_tx.send(AppEvent::Shutdown);
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
    }

    fn poll_viewport_hide(&mut self, ctx: &egui::Context) {
        let (close, minimized) = ctx.input(|i| {
            let vp = i.viewport();
            (vp.close_requested(), vp.minimized == Some(true))
        });

        if close {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.hide_to_tray(ctx);
        } else if minimized && !self.hidden_to_tray.load(Ordering::Acquire) {
            self.hidden_to_tray.store(true, Ordering::Release);
        }
    }

    fn poll_viewport_restore(&mut self, ctx: &egui::Context) {
        let minimized = ctx.input(|i| i.viewport().minimized == Some(true));
        let was_minimized = self.last_minimized;
        self.last_minimized = minimized;

        if self.hidden_to_tray.load(Ordering::Acquire) && was_minimized && !minimized {
            self.show_window(ctx, self.baseline.clone());
        }
    }

    fn poll_tray(&mut self, _ctx: &egui::Context) {
        self.tray.poll(&self.event_tx);
    }

    fn apply(&mut self) {
        normalize_config_devices(&mut self.draft);
        match self.draft.save(&self.config_path) {
            Ok(()) => {
                info!(path = %self.config_path.display(), "config saved from settings");
                let applied = self.draft.clone();
                if self
                    .event_tx
                    .send(AppEvent::SettingsApplied(applied))
                    .is_err()
                {
                    self.show_toast(self.texts().status_send_failed, false);
                    return;
                }
                self.baseline = self.draft.clone();
                self.show_toast(self.texts().status_apply_ok, true);
            }
            Err(err) => {
                self.show_toast(err.to_string(), false);
            }
        }
    }

    fn cancel(&mut self) {
        self.draft = self.baseline.clone();
        self.toast = None;
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
                    self.show_toast(self.texts().status_command_failed, false);
                    return;
                }
                self.baseline.enabled = enabled;
                let msg = if enabled {
                    self.texts().status_routing_on
                } else {
                    self.texts().status_routing_off
                };
                self.show_toast(msg, true);
            }
            Err(err) => {
                self.draft.enabled = !enabled;
                self.show_toast(err.to_string(), false);
            }
        }
    }

    fn set_locale(&mut self, ctx: &egui::Context, locale: Locale) {
        if self.baseline.locale == locale {
            return;
        }
        self.draft.locale = locale;
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(
            self.texts().window_title.to_string(),
        ));
        if let Err(err) = self.draft.save(&self.config_path) {
            self.show_toast(err.to_string(), false);
            return;
        }
        self.baseline.locale = locale;
        match TrayHandle::new(
            locale,
            Arc::clone(&self.ui_ctx),
            Arc::clone(&self.hidden_to_tray),
        ) {
            Ok(tray) => self.tray = tray,
            Err(err) => error!(%err, "tray rebuild after locale change failed"),
        }
        let _ = self.event_tx.send(AppEvent::LocaleChanged(locale));
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

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.store_ctx(ctx);
        ctx.request_repaint_after(Duration::from_millis(100));
        self.poll_viewport_hide(ctx);
        self.poll_tray(ctx);
        self.poll_commands(ctx);
        self.poll_viewport_restore(ctx);
        self.sync_window_size(ctx);

        let snap = self.metrics.snapshot();
        let texts = self.texts();

        egui::TopBottomPanel::top("header")
            .exact_height(Theme::header_height())
            .frame(theme::header_frame())
            .show(ctx, |ui| {
                if theme::header(ui, &snap, texts, &mut self.draft.locale) {
                    self.set_locale(ctx, self.draft.locale);
                }
            });

        egui::TopBottomPanel::bottom("footer")
            .exact_height(Theme::footer_height())
            .frame(theme::footer_frame())
            .show(ctx, |ui| {
                let has_unsaved = self.draft != self.baseline;
                let actions =
                    theme::settings_footer(ui, texts, env!("CARGO_PKG_VERSION"), has_unsaved);
                if actions.cancel {
                    self.cancel();
                }
                if actions.apply {
                    self.apply();
                }
            });

        egui::CentralPanel::default()
            .frame(theme::panel_frame())
            .show(ctx, |ui| {
                ui.set_width(ui.available_width());

                theme::section_header(ui, texts.section_general, true);
                theme::group_box(ui, |ui| {
                    let mut enabled = self.draft.enabled;
                    theme::routing_row(ui, texts, &mut enabled);
                    if enabled != self.draft.enabled {
                        self.set_routing(enabled);
                    }
                });

                theme::section_header(ui, texts.section_devices, false);
                theme::group_box(ui, |ui| {
                    theme::picker_row(
                        ui,
                        "micro",
                        texts.device_mic,
                        true,
                        &self.devices.inputs,
                        &mut self.draft.devices.voice_input,
                    );
                    theme::picker_row(
                        ui,
                        "vac",
                        texts.device_vac,
                        false,
                        &self.devices.outputs,
                        &mut self.draft.devices.virtual_mic_output,
                    );
                    theme::picker_row(
                        ui,
                        "monitor",
                        texts.device_monitor,
                        false,
                        &self.devices.outputs,
                        &mut self.draft.devices.monitor_output,
                    );
                });

                theme::section_header(ui, texts.section_audio, false);
                theme::group_box(ui, |ui| {
                    theme::toggle_row(
                        ui,
                        texts.monitor_headphones,
                        true,
                        &mut self.draft.monitor.enabled,
                    );
                    theme::slider_block(
                        ui,
                        texts.monitor_volume,
                        false,
                        &mut self.draft.monitor.volume,
                        0.0..=2.0,
                    );
                    theme::buffer_block(
                        ui,
                        false,
                        &mut self.draft.buffer_frames,
                        texts.buffer_latency,
                    );
                });
                theme::section_footer(ui, texts.buffer_hint);
            });

        self.poll_toast(ctx);
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
    cmd_rx: Receiver<SettingsCommand>,
    event_tx: Sender<AppEvent>,
    metrics: Arc<AudioMetrics>,
    ui_ctx: Arc<Mutex<Option<egui::Context>>>,
) -> Result<()> {
    let devices = enumerate_device_lists()?;
    let window_title = config.locale.texts().window_title.to_string();
    let height = Theme::window_height();
    let locale = config.locale;
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([Theme::WINDOW_W, height])
            .with_min_inner_size([Theme::WINDOW_W, height])
            .with_max_inner_size([Theme::WINDOW_W, height])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_active(true),
        // Don't restore a previous maximize / oversized rect across mixed-DPI monitors.
        persist_window: false,
        event_loop_builder: Some(Box::new(|builder| {
            use winit::platform::windows::EventLoopBuilderExtWindows;
            builder.with_any_thread(true);
        })),
        ..Default::default()
    };

    eframe::run_native(
        &window_title,
        native_options,
        Box::new(move |cc| {
            theme::Theme::apply(&cc.egui_ctx);
            let hidden_to_tray = Arc::new(AtomicBool::new(false));
            let tray = TrayHandle::new(locale, Arc::clone(&ui_ctx), Arc::clone(&hidden_to_tray))
                .expect("tray icon must initialize on winit thread");
            Ok(Box::new(SettingsApp::new(
                config_path.clone(),
                config.clone(),
                devices.clone(),
                event_tx,
                metrics,
                cmd_rx,
                ui_ctx,
                tray,
                hidden_to_tray,
            )) as Box<dyn eframe::App>)
        }),
    )
    .map_err(|e| Error::config(format!("settings window: {e}")))?;

    Ok(())
}
