//use std::env;
use std::fs;

pub fn da_oi() {
    println!("oi :)")
}

pub fn leitura(caminho: String) -> Vec<(String, i32)>{
    
    let contents = fs::read_to_string(caminho)
        .expect("deveria ter lido o arquivo");
    let mut nome_bool = true;
    let mut primeira = true;
    let mut nome = String::new();
    let mut horas_jogadas = String::new();
    let mut array:Vec<(String, i32)> = Vec::new(); 
    for  c in contents.chars() {

        if c == ','{
            nome_bool = !nome_bool;
        }else if c == '\n'{
            if primeira{
                primeira = false;
            } else{
                let num_horas_jogadas: i32 = horas_jogadas.parse().expect("");
                array.push((nome, num_horas_jogadas));
            }
            nome_bool = !nome_bool;
            nome = String::new();
            horas_jogadas = String::new();
            
        }
        else{
            if nome_bool{
                nome.push(c);
            } else {
                horas_jogadas.push(c);
            }
            println!("nome: {nome}; horas jogadas: {horas_jogadas}");
        }
    }

    println!("texto: {contents}");

    return array;
}