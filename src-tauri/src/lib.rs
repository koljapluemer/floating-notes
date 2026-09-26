use std::fs;
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NoteEntry {
    filename: String,
    body: String,
}

#[tauri::command]
fn list_notes(folder: String) -> Result<Vec<NoteEntry>, String> {
    let dir = Path::new(&folder);
    let mut entries: Vec<NoteEntry> = fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "json" {
                let filename = path.file_name()?.to_str()?.to_string();
                let raw = fs::read_to_string(&path).ok()?;
                let json: serde_json::Value = serde_json::from_str(&raw).ok()?;
                let body = json.get("body")?.as_str()?.to_string();
                Some(NoteEntry { filename, body })
            } else {
                None
            }
        })
        .collect();

    // Sort by filename descending (newer timestamps first)
    entries.sort_by(|a, b| b.filename.cmp(&a.filename));
    Ok(entries)
}

#[tauri::command]
fn save_note(folder: String, filename: String, body: String) -> Result<(), String> {
    let path = Path::new(&folder).join(&filename);
    let json = serde_json::json!({ "body": body });
    let content = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_note(folder: String, filename: String) -> Result<(), String> {
    let path = Path::new(&folder).join(&filename);
    fs::remove_file(path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![list_notes, save_note, delete_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
