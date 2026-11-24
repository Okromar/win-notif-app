use std::panic;
use std::thread;
use winrt_notification::{Duration, Sound, Toast};

#[tauri::command]
fn show_win_notif(title: String, body: String) -> Result<(), String> {
    let result = thread::spawn(move || {
        panic::catch_unwind(|| {
            Toast::new("Microsoft.MicrosoftEdge.Stable_8wekyb3d8bbwe!App")
                .title(&title)
                .text1(&body)
                .sound(Some(Sound::Default))
                .duration(Duration::Short)
                .show()
        })
    })
    .join()
    .map_err(|_| "Thread notification interrompu".to_string())?;

    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(format!("Toast refusé par Windows : {:?}", e)),
        Err(_) => Err("Erreur interne WinRT interceptée".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show_win_notif])
        .run(tauri::generate_context!())
        .expect("Erreur lors du démarrage de l'application Tauri");
}
