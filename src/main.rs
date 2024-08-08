mod ler_arquivo;

fn main() {
    let mut array:Vec<(String, i32)> = ler_arquivo::leitura(String::from("/home/pfc15/Documents/2024.1/pa/steam_game/pedro.csv"));
    for (a, b) in array{
        println!("nome: {a}; horas jogadas: {b}");
    }
}


