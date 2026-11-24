use std::os::windows::process::CommandExt;
use std::process::Command;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[tauri::command]
fn show_win_notif(title: String, body: String) {
    let safe_title = title.replace('"', "'").replace('\n', " ");
    let safe_body = body.replace('"', "'").replace('\n', " ");

    let ps_script = format!(
        r#"
        Add-Type -AssemblyName System.Windows.Forms
        Add-Type -AssemblyName System.Drawing
        $notify = New-Object System.Windows.Forms.NotifyIcon
        $notify.Icon = [System.Drawing.SystemIcons]::Information
        $notify.BalloonTipIcon = [System.Windows.Forms.ToolTipIcon]::Info
        $notify.BalloonTipTitle = "{0}"
        $notify.BalloonTipText = "{1}"
        $notify.Visible = $true
        $notify.ShowBalloonTip(8000)
        Start-Sleep -Seconds 9
        $notify.Dispose()
        "#,
        safe_title, safe_body
    );

    let _ = Command::new("powershell")
        .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &ps_script])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show_win_notif])
        .run(tauri::generate_context!())
        .expect("Erreur lors du démarrage de l'application Tauri");
}
