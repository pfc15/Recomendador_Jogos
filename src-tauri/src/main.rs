mod ler_arquivo;
mod merge_sort;
use ler_arquivo::leitura;
use rand::Rng;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn teste(num: i32) -> i32 {
    let mut rgn = rand::thread_rng();

    let _num = rgn.gen_range(1..101);

    _num
}


#[tauri::command]
fn get_jogos() -> Vec<ler_arquivo::card> {
    let caminho = "./dados/jogos.csv";
    let jogos = ler_arquivo::leitura_comeco(caminho);
    jogos
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![teste])
        .invoke_handler(tauri::generate_handler![get_jogos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
