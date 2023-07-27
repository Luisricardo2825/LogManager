#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::Write;

use log_manager::schemas::latest_jar_schema::LatestJarSchema;
use log_manager::{__cmd__get_log_btn, commands::get_log_btn::get_log_btn};
use log_manager::{__cmd__post_mod, commands::post_modulo_java::post_mod};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_log_btn, post_mod])
        .on_page_load(|_window, _payload| {
            tauri::async_runtime::spawn(get_latest_jar());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn get_latest_jar() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("{\"message\":\"Erro ao iniciar o client\"}");

    let response = client
        .get("https://api.github.com/repos/Luisricardo2825/UtilityLibJava/releases/latest")
        .header("User-Agent", "Dbknower 1.0")
        .send()
        .await?;
    let body = response.text().await?;
    let json: LatestJarSchema = serde_json::from_str(&body).unwrap();
    let a = json.assets.last().unwrap();
    let url = &a.browser_download_url;
    let file_name = &a.name;

    let response = client
        .get(url)
        .header("User-Agent", "Dbknower 1.0")
        .send()
        .await?;
    let body_file = response.bytes().await?;
    let mut file = File::create(format!("../lib/{}", file_name)).expect("Error creating file");
    file.write_all(&body_file)
        .expect("Error ao escrever o arquivo");

    println!("Pacotes Atualizados!");
    Ok(())
}
