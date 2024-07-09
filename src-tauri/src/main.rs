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
        .invoke_handler(tauri::generate_handler![display_number, add_to_number, del_from_number, clear_number, flip_sign])
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

#[tauri::command]
fn del_from_number(state: tauri::State<Calculator>) {
    let mut num = state.num1.lock().unwrap();
    if num.len() <= 1 || (num.len() == 2 && num.contains('-')) {
        *num = "0".to_string();
    } else {
        *num = num[..num.len() - 1].to_string();
    }
}

#[tauri::command]
fn clear_number(state: tauri::State<Calculator>) {
    let mut num = state.num1.lock().unwrap();
    *num = "0".to_string();
}

#[tauri::command]
fn flip_sign(state: tauri::State<Calculator>) {
    let mut num = state.num1.lock().unwrap();
    if !num.contains('-') && *num != "0" {
        num.insert(0, '-');
    } else if *num != "0" {
        *num = num[1..].to_string();
    }
}
