use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host, HostId};
use tracing::info;

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct DeviceLists {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

pub fn enumerate_device_lists() -> Result<DeviceLists> {
    let host = default_host();
    let inputs = collect_device_names(host.input_devices().map_err(map_device_err)?)?;
    let outputs = collect_device_names(host.output_devices().map_err(map_device_err)?)?;
    Ok(DeviceLists { inputs, outputs })
}

fn collect_device_names(devices: impl Iterator<Item = Device>) -> Result<Vec<String>> {
    let mut names = Vec::new();
    for device in devices {
        if let Ok(name) = device.name() {
            names.push(name);
        }
    }
    names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    Ok(names)
}

pub fn stable_device_query(full_name: &str) -> String {
    let lower = full_name.to_lowercase();
    if lower.contains("fifine") && lower.contains("microphone") {
        return "fifine Microphone".to_string();
    }
    if lower.contains("cable") && lower.contains("input") {
        return "CABLE Input".to_string();
    }
    if lower.contains("fifine") && (lower.contains("sc3") || lower.contains("haut-parleurs")) {
        return "fifine SC3".to_string();
    }
    full_name.trim().to_string()
}

pub fn best_device_index(names: &[String], query: &str) -> usize {
    if names.is_empty() {
        return 0;
    }
    let needle = query.to_lowercase();
    names
        .iter()
        .position(|name| name.to_lowercase().contains(&needle))
        .unwrap_or(0)
}

pub fn default_host() -> Host {
    cpal::host_from_id(HostId::Wasapi).unwrap_or_else(|_| cpal::default_host())
}

pub fn list_all_devices() -> Result<()> {
    let host = default_host();
    println!("Host: {}", host.id().name());
    println!();

    println!("=== INPUT DEVICES ===");
    for device in host.input_devices().map_err(map_device_err)? {
        print_device("in ", &device);
    }

    println!();
    println!("=== OUTPUT DEVICES ===");
    for device in host.output_devices().map_err(map_device_err)? {
        print_device("out", &device);
    }

    Ok(())
}

fn print_device(prefix: &str, device: &Device) {
    match device.name() {
        Ok(name) => {
            let cfg = device
                .default_input_config()
                .or_else(|_| device.default_output_config());
            match cfg {
                Ok(c) => println!(
                    "[{prefix}] {name} — {} Hz, {} ch, {:?}",
                    c.sample_rate().0,
                    c.channels(),
                    c.sample_format()
                ),
                Err(_) => println!("[{prefix}] {name}"),
            }
        }
        Err(err) => println!("[{prefix}] <unnamed: {err}>"),
    }
}

pub fn find_input_device(host: &Host, name_substr: &str) -> Result<Device> {
    find_device(
        host.input_devices().map_err(map_device_err)?,
        name_substr,
        "input",
    )
}

pub fn find_output_device(host: &Host, name_substr: &str) -> Result<Device> {
    find_device(
        host.output_devices().map_err(map_device_err)?,
        name_substr,
        "output",
    )
}

pub fn try_find_input_device(host: &Host, name_substr: &str) -> Option<Device> {
    find_device_optional(host.input_devices().ok()?, name_substr)
}

fn find_device(
    devices: impl Iterator<Item = Device>,
    name_substr: &str,
    kind: &str,
) -> Result<Device> {
    find_device_optional(devices, name_substr).ok_or_else(|| {
        Error::device(format!(
            "no {kind} device matching '{name_substr}' — run --list-devices"
        ))
    })
}

fn find_device_optional(
    devices: impl IntoIterator<Item = Device>,
    name_substr: &str,
) -> Option<Device> {
    let needle = name_substr.trim();
    if needle.is_empty() {
        return None;
    }

    let mut list: Vec<(String, Device)> = devices
        .into_iter()
        .filter_map(|device| device.name().ok().map(|name| (name, device)))
        .collect();

    let needle_lower = needle.to_lowercase();

    if let Some(idx) = list
        .iter()
        .position(|(name, _)| name.eq_ignore_ascii_case(needle))
    {
        return Some(list.swap_remove(idx).1);
    }

    let mut matches: Vec<usize> = list
        .iter()
        .enumerate()
        .filter(|(_, (name, _))| name.to_lowercase().contains(&needle_lower))
        .map(|(i, _)| i)
        .collect();

    if matches.is_empty() {
        return None;
    }

    if matches.len() > 1 {
        info!(
            query = needle,
            picked = %list[matches[0]].0,
            count = matches.len(),
            "multiple device matches — using first"
        );
    }

    Some(list.swap_remove(matches[0]).1)
}

fn map_device_err(err: cpal::DevicesError) -> Error {
    Error::device(format!("enumerate devices: {err}"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn substring_match_is_case_insensitive() {
        assert!("Microphone (2- fifine Microphone)"
            .to_lowercase()
            .contains(&"fifine microphone".to_lowercase()));
    }
}
