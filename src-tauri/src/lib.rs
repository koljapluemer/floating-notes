use std::fs;
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NoteEntry {
    filename: String,
    content: String,
}

#[tauri::command]
fn list_notes(folder: String) -> Result<Vec<NoteEntry>, String> {
    let dir = Path::new(&folder);
    let mut entries: Vec<NoteEntry> = fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "md" {
                let filename = path.file_name()?.to_str()?.to_string();
                let content = fs::read_to_string(&path).ok()?;
                Some(NoteEntry { filename, content })
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
fn save_note(folder: String, filename: String, content: String) -> Result<(), String> {
    let path = Path::new(&folder).join(&filename);
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
