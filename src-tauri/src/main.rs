// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri;

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
    num1: f64,
    num2: f64,
    op: Operation,
}

fn main() {
    tauri::Builder::default()
        .manage(Calculator {num1: 0.0, num2: 0.0, op: Operation::Nop})
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
