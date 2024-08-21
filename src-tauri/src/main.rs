mod ler_arquivo;
mod merge_sort;
use ler_arquivo::leitura;
use merge_sort::Jogo;
use std::collections::BinaryHeap;
use std::collections::HashMap;

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
fn get_resposta (jogos:String) ->  Jogo {
    let ranking_usuario = ler_json(jogos.clone());
    println!("olha oq eu recebi:");
    let generos = ["retro.csv", "estrategia.csv", "indie.csv", "fps.csv", "fighter.csv"];
    let mut mais_compativel = merge_sort::MergeSort{
        comparado: Vec::new(),
        inversoes: 0,
        tamanho: 0,
        recomendacoes: BinaryHeap::new(),
        hash_posicoes: HashMap::new()
    };


    for g in generos {
        let mut caminho = String::from("../src-tauri/dados/");
        caminho.push_str(g);
        let ranking_genero = leitura(caminho);
        let obj_inversoes = merge_sort::MergeSort::new(ranking_usuario.clone(), ranking_genero.clone());
        if mais_compativel.tamanho == 0 || mais_compativel.inversoes > obj_inversoes.inversoes{
            mais_compativel = obj_inversoes.clone();
        }
    }
    let retorno= mais_compativel.recomendacoes.pop().clone();

    retorno.expect("heap não pode estar vazio")
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_jogos, get_resposta])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
