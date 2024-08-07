//use std::env;
use std::fs;

pub fn da_oi() {
    println!("oi :)")
}

pub fn leitura(caminho: String){
    
    let contents = fs::read_to_string(caminho)
        .expect("deveria ter lido o arquivo");
    
    for  c in contents.chars(){
        print!("{c}")
    }

    println!("texto: {contents}");

}