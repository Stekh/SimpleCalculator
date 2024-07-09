// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri;
use std::sync::Mutex;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    SQRT,
    Flip,
    Del,
    Clear,
    Calc,
    Nop,
}

struct Calculator {
    num1: Mutex<String>,
    num2: Mutex<String>,
    dec: Mutex<bool>,
    op: Mutex<Operation>,
}

fn main() {
    tauri::Builder::default()
        .manage(Calculator { num1: Mutex::new("5".to_string()), num2: Mutex::new("0".to_string()), dec: Mutex::new(false), op: Mutex::new(Operation::Nop) })
        .invoke_handler(tauri::generate_handler![display_number, add_to_number])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn display_number(state: tauri::State<Calculator>) -> String {
    let num = state.num1.lock().unwrap();
    format!("{}", *num)
}

#[tauri::command]
fn add_to_number(add: &str, state: tauri::State<Calculator>) {
    let mut num = state.num1.lock().unwrap();
    if *num == "0" {
        *num = add.to_string();
    } else {
        *num += add;
    }
}
