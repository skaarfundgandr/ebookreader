pub mod data;
pub mod handlers;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_sqlite_connection(){
        use crate::data::database;
        let _conn = database::connect_from_pool().await;

        assert!(_conn.is_ok());
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
