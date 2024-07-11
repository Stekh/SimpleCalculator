// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri;
use std::sync::Mutex;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
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
        .invoke_handler(tauri::generate_handler![display_number, add_to_number, del_from_number, clear_number, flip_sign, square_root, set_operation, calculate])
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
    let mut dec = state.dec.lock().unwrap();
    if *num == "0" && add != "." {
        *num = add.to_string();
    } else if add != "." || (add == "." && !*dec) {
        *num += add;
        if add == "." {
            *dec = true;
        }
    }
}

#[tauri::command]
fn del_from_number(state: tauri::State<Calculator>) {
    let mut num = state.num1.lock().unwrap();
    if num.len() <= 1 || (num.len() == 2 && num.contains('-')) {
        *num = "0".to_string();
    } else {
        if num.chars().last().unwrap() == '.' {
            let mut dec = state.dec.lock().unwrap();
            *dec = false;
        }
        *num = num[..num.len() - 1].to_string();
    }
}

#[tauri::command]
fn clear_number(state: tauri::State<Calculator>) {
    let mut num = state.num1.lock().unwrap();
    *num = "0".to_string();

    let mut dec = state.dec.lock().unwrap();
    *dec = false;

    let mut op = state.op.lock().unwrap();
    *op = Operation::Nop;
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

#[tauri::command]
fn square_root(state: tauri::State<Calculator>) {
    let mut num = state.num1.lock().unwrap();
    let tmp: f64 = num.parse().unwrap();
    *num = tmp.sqrt().to_string();
}

#[tauri::command]
fn set_operation(oper: &str, state: tauri::State<Calculator>) {
    let mut op = state.op.lock().unwrap();
    match oper {
        "add" => *op = Operation::Add,
        "sub" => *op = Operation::Subtract,
        "mul" => *op = Operation::Multiply,
        "div" => *op = Operation::Divide,
        _ => *op = Operation::Nop,
    }

    let num1 = state.num1.lock().unwrap();
    let mut num2 = state.num2.lock().unwrap();
    *num2 = num1.clone();
}

#[tauri::command]
fn calculate(state: tauri::State<Calculator>) {
    let mut num1 = state.num1.lock().unwrap();
    let num2 = state.num2.lock().unwrap();
    let op = state.op.lock().unwrap();

    let n1: f64 = num1.parse().unwrap();
    let n2: f64 = num2.parse().unwrap();

    match *op {
        Operation::Add => *num1 = (n2 + n1).to_string(),
        Operation::Subtract => *num1 = (n2 - n1).to_string(),
        Operation::Multiply => *num1 = (n2 * n1).to_string(),
        Operation::Divide => *num1 = (n2 / n1).to_string(),
        Operation::Nop => (),
    }
}
