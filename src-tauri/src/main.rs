// Disables the command prompt window that would normally pop up on Windows if you run a bundled app
#[cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

use tauri::Manager;

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .setup(|app| {
      #[cfg(all(debug_assertions, not(target_os = "macos")))] // only include this code on debug builds
      {
        let window = app.get_window("main").unwrap();
        window.open_devtools();
        // window.close_devtools();
      }
      Ok(())
    })
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .invoke_handler(tauri::generate_handler![greet])
    .run(context)
    .expect("error while running tauri application");
}
