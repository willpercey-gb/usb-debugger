use std::fs;
use std::process::Command;
use serde::Deserialize;
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::command;
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
struct USBDevice {
    _name: Option<String>,
    vendor_id: Option<String>,
    serial_num: Option<String>,
    location_id: Option<String>,
    Media: Option<Value>,
    _items: Option<Value>,
}

#[derive(Deserialize, Debug)]
struct USBController {
    _items: Option<Value>,
}

#[derive(Deserialize, Debug)]
struct USBData {
    SPUSBDataType: Vec<USBController>,
}

//TODO fn show_devices_in_use_by_process()
fn get_usb_info_macos() -> Result<Vec<(String, String, String, String)>, String> {
    let output = Command::new("system_profiler")
        .arg("SPUSBDataType")
        .arg("-json")
        .output()
        .map_err(|e| format!("Failed to execute system_profiler: {}", e))?;

    let usb_output = String::from_utf8_lossy(&output.stdout);
    let usb_data: USBData = serde_json::from_str(&usb_output)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let mut devices = Vec::new();

    for controller in usb_data.SPUSBDataType {
        if let Some(items) = &controller._items {
            process_item_macos(items, &mut devices);
        }
    }

    Ok(devices)
}

fn process_item_macos(item: &Value, devices: &mut Vec<(String, String, String, String)>) {
    match item {
        Value::Array(array) => {
            for value in array {
                process_item_macos(value, devices);
            }
        }
        Value::Object(map) => {
            if let Some(media) = map.get("Media") {
                process_item_macos(media, devices);
            }
            if let Some(items) = map.get("_items") {
                process_item_macos(items, devices);
            }
            if map.get("serial_num").is_some() {
                if let (Some(name), Some(vendor), Some(serial), Some(path)) = (
                    map.get("_name").and_then(Value::as_str).map(String::from),
                    map.get("vendor_id").and_then(Value::as_str).map(String::from),
                    map.get("serial_num").and_then(Value::as_str).map(String::from),
                    map.get("location_id").and_then(Value::as_str).map(String::from),
                ) {
                    devices.push((name, vendor, serial, path));
                }
            }
        }
        _ => {}
    }
}

#[command]
fn list_usb_devices() -> Result<String, String> {
    let usb_info = if cfg!(target_os = "windows") {
        get_usb_info_windows()
    } else if cfg!(target_os = "macos") {
        get_usb_info_macos()
    } else if cfg!(target_os = "linux") {
        get_usb_info_linux()
    } else {
        Err("Unsupported OS".into())
    };

    match usb_info {
        Ok(usb_devices) => {
            let result: Vec<_> = usb_devices
                .into_iter()
                .map(|(name, vendor, serial_number, path)| json!({ "name": name, "vendor": vendor, "serial_number": serial_number, "path": path }))
                .collect();
            Ok(serde_json::to_string(&result).unwrap())
        }
        Err(e) => Err(e),
    }
}

fn get_usb_info_windows() -> Result<Vec<(String, String, String, String)>, String> {
    //TODO
    Ok(vec![
        (
            "USB Flash Drive".to_string(),
            "VendorA".to_string(),
            "12345678".to_string(),
            "0001".to_string(),
        ),
    ])
}

fn get_usb_info_linux() -> Result<Vec<(String, String, String, String)>, String> {
    //TODO
    Ok(vec![
        (
            "USB Mouse".to_string(),
            "VendorB".to_string(),
            "ABC12345".to_string(),
            "0002".to_string(),
        ),
    ])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_usb_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
