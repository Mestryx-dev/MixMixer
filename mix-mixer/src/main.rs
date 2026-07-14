//! MixMixer — micro post-E-APO → VB-Cable (latence minimale).

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod audio;
mod config;
mod devices;
mod error;
mod ui;

use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use clap::Parser;
use crossbeam_channel::{bounded, Receiver, Sender};
use tracing::{error, info};

use crate::audio::engine::{AudioCommand, AudioEngine};
use crate::audio::metrics::AudioMetrics;
use crate::config::Config;
use crate::error::Result;
use crate::ui::settings::SettingsLauncher;
use crate::ui::tray::{TrayAction, TrayManager};

#[derive(Parser, Debug)]
#[command(name = "mix-mixer", about = "Forward microphone to VB-Cable virtual mic")]
struct Cli {
    #[arg(short, long, default_value = "config.json")]
    config: PathBuf,

    #[arg(long)]
    list_devices: bool,
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    Tray(TrayAction),
    SettingsApplied(Config),
    SetRoutingEnabled(bool),
    Shutdown,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("mix_mixer=info")),
        )
        .init();

    if let Err(err) = run() {
        error!(%err, "fatal error");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    if cli.list_devices {
        devices::list_all_devices()?;
        return Ok(());
    }

    let config_path = cli.config.clone();
    let mut config = Config::load(&config_path)?;
    info!(path = %config_path.display(), "config loaded");

    let (cmd_tx, cmd_rx) = bounded::<AudioCommand>(64);
    let (event_tx, event_rx) = bounded::<AppEvent>(64);
    let metrics = Arc::new(AudioMetrics::new());

    let config_for_audio = config.clone();
    let metrics_for_audio = Arc::clone(&metrics);
    let audio_handle = thread::Builder::new()
        .name("mix-mixer-audio".into())
        .spawn(move || {
            if let Err(err) = run_audio_thread(config_for_audio, cmd_rx, metrics_for_audio) {
                error!(%err, "audio thread failed");
            }
        })
        .map_err(|e| crate::error::Error::Audio(format!("spawn audio thread: {e}")))?;

    let tray = TrayManager::new(event_tx.clone())?;
    let settings = SettingsLauncher::new(event_tx.clone(), metrics);

    if let Err(err) = settings.open(config_path.clone(), &config) {
        error!(%err, "open settings on startup failed");
    }

    info!("mix-mixer running — micro → VAC");

    run_event_loop(
        &config_path,
        &mut config,
        event_rx,
        cmd_tx,
        tray,
        settings,
    )?;

    let _ = audio_handle.join();
    info!("mix-mixer stopped");
    Ok(())
}

fn run_audio_thread(
    config: Config,
    cmd_rx: Receiver<AudioCommand>,
    metrics: Arc<AudioMetrics>,
) -> Result<()> {
    let mut engine = AudioEngine::new(config, metrics)?;
    engine.run(cmd_rx)
}

fn run_event_loop(
    config_path: &PathBuf,
    config: &mut Config,
    event_rx: Receiver<AppEvent>,
    cmd_tx: Sender<AudioCommand>,
    mut tray: TrayManager,
    settings: SettingsLauncher,
) -> Result<()> {
    loop {
        tray.poll()?;

        while let Ok(event) = event_rx.try_recv() {
            match event {
                AppEvent::Tray(TrayAction::OpenSettings) => {
                    if let Err(err) = settings.open(config_path.clone(), config) {
                        error!(%err, "open settings failed");
                    }
                }
                AppEvent::SettingsApplied(new_cfg) => {
                    let old_cfg = config.clone();
                    *config = new_cfg.clone();
                    if config.needs_stream_restart(&old_cfg) {
                        let _ = cmd_tx.send(AudioCommand::RestartWithConfig(Box::new(new_cfg)));
                    } else {
                        let _ = cmd_tx.send(AudioCommand::ReloadConfig(Box::new(new_cfg)));
                    }
                }
                AppEvent::SetRoutingEnabled(enabled) => {
                    config.enabled = enabled;
                    let _ = cmd_tx.send(AudioCommand::SetRoutingEnabled(enabled));
                }
                AppEvent::Tray(TrayAction::Quit) => {
                    let _ = cmd_tx.send(AudioCommand::Shutdown);
                    return Ok(());
                }
                AppEvent::Tray(TrayAction::ToggleMonitor) => {
                    let _ = cmd_tx.send(AudioCommand::ToggleMonitor);
                }
                AppEvent::Tray(TrayAction::ReloadConfig) => {
                    match Config::load(config_path) {
                        Ok(new_cfg) => {
                            *config = new_cfg.clone();
                            let _ = cmd_tx.send(AudioCommand::RestartWithConfig(Box::new(new_cfg)));
                        }
                        Err(err) => error!(%err, "reload config failed"),
                    }
                }
                AppEvent::Shutdown => {
                    let _ = cmd_tx.send(AudioCommand::Shutdown);
                    return Ok(());
                }
            }
        }

        thread::sleep(Duration::from_millis(16));
    }
}
