// mod ler_arquivo;
mod merge_sort;
use merge_sort::MergeSort;

// Função fictícia de bubble_sort para compilar o código
fn bubble_sort(lista: Vec<merge_sort::Jogo>) -> Vec<merge_sort::Jogo> {
    let mut lista = lista;
    let len = lista.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if lista[j].horas_jogadas < lista[j + 1].horas_jogadas {
                lista.swap(j, j + 1);
            }
        }
    }
    lista
}



fn main() {
    //let mut array:Vec<(String, i32)> = ler_arquivo::leitura(String::from("/home/pfc15/Documents/2024.1/pa/steam_game/pedro.csv"));
    //for (a, b) in array{
    //    println!("nome: {a}; horas jogadas: {b}");
    //}

    let xs = vec![89, 60, 12, 45, 37, 52, 33, 97, 20];
    let nomes = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
        String::from("e"),
        String::from("f"),
        String::from("g"),
        String::from("h"),
        String::from("i"),
    ];

    let mut lista: Vec<merge_sort::Jogo> = Vec::new();
    for i in 0..9 {
        let novo = merge_sort::Jogo {
            horas_jogadas: xs[i],
            nome: nomes[i].clone(),
        };
        lista.push(novo);
    }

    let mut nova_lista = bubble_sort(lista.clone());

    for i in nova_lista.iter_mut() {
        print!("{}; {}", i.nome, i.horas_jogadas);
    }
        

    let obj = MergeSort::new(nova_lista.clone(), lista.clone());
    println!("numeros de inversões {}", obj.inversoes );


}


