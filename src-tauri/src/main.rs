use tauri_plugin_dialog::DialogExt; // 引入 Dialog 插件
use std::fs;
use std::io::Write;
use std::process::Command;
use tauri::{command};

#[command]
fn select_folder(app_handle: tauri::AppHandle) -> String {
    let dialog = app_handle.dialog();
    dialog
        .file()
        .blocking_pick_folder()
        .map(|path| path.to_string()) // 使用 FilePath::to_string() 方法
        .unwrap_or_else(|| "".into())
}

#[command]
fn scan_flv_files(path: String) -> Vec<String> {
    fs::read_dir(&path)
        .expect("无法读取文件夹")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "flv"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect()
}

#[command]
fn generate_filelist_and_merge(files: Vec<String>, folder_path: String) {
    let mut filelist = fs::File::create(format!("{}/filelist.txt", folder_path)).unwrap();
    for file in &files {
        writeln!(filelist, "file '{}'", file).unwrap();
    }

    let output_path = format!("{}/output.mp4", folder_path);
    let status = Command::new("ffmpeg")
        .args([
            "-f", "concat",
            "-safe", "0",
            "-i", &format!("{}/filelist.txt", folder_path),
            "-c", "copy",
            &output_path,
        ])
        .status()
        .expect("FFmpeg 执行失败");

    if !status.success() {
        panic!("FFmpeg 合并失败");
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init()) // 初始化 Dialog 插件
        .invoke_handler(tauri::generate_handler![
            select_folder,
            scan_flv_files,
            generate_filelist_and_merge
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}