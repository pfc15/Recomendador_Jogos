mod ler_arquivo;
mod merge_sort;
use merge_sort::MergeSort;

// Função fictícia de bubble_sort para compilar o código




fn main() {
    let mut pedro:Vec<merge_sort::Jogo> = ler_arquivo::leitura(String::from("/home/pfc15/Documents/2024.1/pa/steam_game/pedro.csv"));
    for i in pedro.iter_mut(){
        println!("nome: {}; horas jogadas: {}", i.nome, i.horas_jogadas);
    }
    let joao:Vec<merge_sort::Jogo> = ler_arquivo::leitura(String::from("/home/pfc15/Documents/2024.1/pa/steam_game/ferrugem/joao.csv"));

    let obj = MergeSort::new(joao.clone(), pedro.clone());
    println!("numeros de inversões {}", obj.inversoes );
    
}


