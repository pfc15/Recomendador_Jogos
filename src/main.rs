mod ler_arquivo;
mod merge_sort;
use merge_sort::MergeSort;


// Função fictícia de bubble_sort para compilar o código




fn main() {
    let mut pedro:Vec<merge_sort::Jogo> = ler_arquivo::leitura(String::from("./dados/pedro.csv"));
    for i in pedro.iter_mut(){
        println!("nome: {}; horas jogadas: {}", i.nome, i.horas_jogadas);
    }


    let joao:Vec<merge_sort::Jogo> = ler_arquivo::leitura(String::from("./dados/joao.csv"));

    let obj = MergeSort::new(pedro.clone(), joao.clone());
    println!("numeros de inversões {}", obj.inversoes );
    if let Some(top_game) = obj.recomendacoes.peek() {
        println!("a recomendação de jogo é: {}", top_game.nome);
    } else {
        println!("The heap is empty.");
    }
}


