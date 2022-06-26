#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri;
use notify_rust::Notification;

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![testfunc])
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}

#[tauri::command]
fn testfunc() {
  Notification::new()
  .summary("You have a message!")
    .body("hello 🦀")
    .appname("Ferris Assist")
    .icon("korg-todo")
    .action("later", "Meh!")
    .action("ok", "Ok")
    .show().unwrap();
}