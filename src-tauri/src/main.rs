mod ler_arquivo;
mod merge_sort;
use ler_arquivo::leitura;
use merge_sort::Jogo;
use rand::Rng;
use serde_json::json;
use std::error::Error;
use serde::Deserialize;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}




#[tauri::command]
fn get_jogos() -> Vec<ler_arquivo::card> {
    let caminho = "./dados/jogos.csv";
    let jogos = ler_arquivo::leitura_comeco(caminho);
    jogos
}

fn ler_json(arquivo:String) -> Vec<Jogo> {
    let arquivo:String = arquivo[1..arquivo.len() - 1].to_string();
    let lista:Vec<String> = arquivo.split(",")
    .map(|s| s[1..s.len() - 1].trim().to_string()) // Convert each &str to String
    .collect();
    let mut retorno:Vec<Jogo> = Vec::new(); 
    for nome in lista {
        retorno.push(
            Jogo{
                nome:nome,
                horas_jogadas:0
            }
        )
    }
    retorno
}

#[tauri::command]
fn get_resposta (jogos:String) ->  Vec<Jogo>{
    let lista = ler_json(jogos.clone());
    println!("olha oq eu recebi:");
    for item in &lista{
        println!("{}",item.nome);
    }
    print!("{}", lista.len());
    lista
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_jogos, get_resposta])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
