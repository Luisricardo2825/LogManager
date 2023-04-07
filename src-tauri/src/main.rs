#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log_manager::{__cmd__get_log, commands::get_log::get_log};
use log_manager::{__cmd__get_log_btn, commands::get_log_btn::get_log_btn};
use log_manager::{__cmd__post_modulo_java, commands::post_modulo_java::post_modulo_java};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{collections::HashMap, sync::Mutex};
use tauri::State;
// here we use Mutex to achieve interior mutability
struct Storage {
    store: Mutex<HashMap<u64, String>>,
}
struct Connection;
struct DbConnection {
    db: Mutex<Option<Connection>>,
}

#[tauri::command]
fn connect(connection: State<DbConnection>) {
    // initialize the connection, mutating the state with interior mutability
    *connection.db.lock().unwrap() = Some(Connection {});
}

#[tauri::command]
fn storage_insert(key: u64, value: String, storage: State<Storage>) {
    // mutate the storage behind the Mutex
    storage.store.lock().unwrap().insert(key, value.clone());
    println!("{} {}", key, value)
}
#[derive(Debug)]
struct MyString(Mutex<String>);
#[tauri::command]
fn string_command<'r>(storage: State<Storage>) {
    let idx: u64 = 0;
    let field = storage.store.lock().unwrap()[&idx].clone();
    println!("state: {:?}", field);
}

fn main() {
    let value = "teste".to_owned();
    tauri::Builder::default()
        .manage(Storage {
            store: Default::default(),
        })
        .manage(DbConnection {
            db: Default::default(),
        })
        .manage(Mutex::new(value))
        .invoke_handler(tauri::generate_handler![
            get_log,
            get_log_btn,
            connect,
            storage_insert,
            string_command,
            r_connect,
            post_modulo_java
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn r_connect(process: tauri::State<Mutex<String>>) {
    // println!("{}", config_name);

    // -----------> Need access to process here
    let output = process.lock().unwrap();
    println!("Teste:{}", output)
}
