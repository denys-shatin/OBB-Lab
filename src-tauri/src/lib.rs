use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_webview_window("main").unwrap();
      
      // Отключаем DevTools в релизе
      #[cfg(not(debug_assertions))]
      {
        // DevTools недоступны в release build по умолчанию
      }
      
      // Отключаем контекстное меню правой кнопкой
      window.eval("document.addEventListener('contextmenu', e => e.preventDefault());")?;
      
      // Отключаем браузерные хоткеи (Ctrl+W, Ctrl+R, F5, F12 и т.д.)
      window.eval(r#"
        document.addEventListener('keydown', function(e) {
          // Блокируем Ctrl+W, Ctrl+R, Ctrl+Shift+I, Ctrl+Shift+J, F5, F12
          if (
            (e.ctrlKey && (e.key === 'w' || e.key === 'W' || e.key === 'r' || e.key === 'R')) ||
            (e.ctrlKey && e.shiftKey && (e.key === 'i' || e.key === 'I' || e.key === 'j' || e.key === 'J')) ||
            e.key === 'F5' ||
            e.key === 'F12'
          ) {
            e.preventDefault();
            e.stopPropagation();
            return false;
          }
        }, true);
      "#)?;
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
