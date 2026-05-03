use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileTreeNode {
    name: String,
    path: String,
    is_dir: bool,
    children: Vec<FileTreeNode>,
}

#[tauri::command]
fn get_default_root() -> Result<String, String> {
    let mut current = std::env::current_dir().map_err(|error| error.to_string())?;

    if current
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "src-tauri")
    {
        current.pop();
    }

    Ok(current.to_string_lossy().to_string())
}

fn build_tree(path: &Path) -> Result<FileTreeNode, String> {
    let metadata = fs::metadata(path).map_err(|error| error.to_string())?;
    let is_dir = metadata.is_dir();

    let mut node = FileTreeNode {
        name: path
            .file_name()
            .and_then(|name| name.to_str())
            .map_or_else(|| path.to_string_lossy().to_string(), |name| name.to_string()),
        path: path.to_string_lossy().to_string(),
        is_dir,
        children: Vec::new(),
    };

    if !is_dir {
        return Ok(node);
    }

    let mut entries: Vec<PathBuf> = fs::read_dir(path)
        .map_err(|error| error.to_string())?
        .filter_map(|entry| entry.ok().map(|item| item.path()))
        .filter(|entry| {
            let name = entry.file_name().and_then(|file_name| file_name.to_str());
            !matches!(name, Some("node_modules" | ".git" | "target"))
        })
        .collect();

    entries.sort_by(|left, right| {
        let left_dir = left.is_dir();
        let right_dir = right.is_dir();

        right_dir
            .cmp(&left_dir)
            .then_with(|| left.to_string_lossy().to_lowercase().cmp(&right.to_string_lossy().to_lowercase()))
    });

    node.children = entries
        .iter()
        .filter_map(|entry| build_tree(entry).ok())
        .collect();

    Ok(node)
}

#[tauri::command]
fn read_directory_tree(root_path: String) -> Result<FileTreeNode, String> {
    build_tree(Path::new(&root_path))
}

#[tauri::command]
fn read_text_file(file_path: String) -> Result<String, String> {
    fs::read_to_string(&file_path).map_err(|error| error.to_string())
}

#[tauri::command]
fn save_file(file_path: String, content: String) -> Result<(), String> {
    fs::write(&file_path, content).map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_default_root,
            read_directory_tree,
            read_text_file,
            save_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
