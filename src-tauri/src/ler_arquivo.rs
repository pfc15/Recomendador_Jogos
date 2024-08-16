//use std::env;
use std::{clone, collections::btree_map::Range, fs, ptr::null};
use crate::merge_sort::Jogo;
use serde::Serialize;
use rand::Rng;

#[derive(Clone, Serialize)]
pub struct card{
    nome:String,
    genero:String,
}

fn bubble_sort(lista: Vec<Jogo>) -> Vec<Jogo> {
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

pub fn leitura(caminho: String) -> Vec<Jogo>{
    
    let contents = fs::read_to_string(caminho)
        .expect("deveria ter lido o arquivo");
    let mut nome_bool = true;
    let mut primeira = true;
    let mut nome = String::new();
    let mut horas_jogadas = String::new();
    let mut array:Vec<Jogo> = Vec::new(); 

    for  c in contents.chars() {

        if c == ','{
            nome_bool = !nome_bool;
        }else if c == '\n'{
            if primeira{
                primeira = false;
            } else{
                let num_horas_jogadas: i32 = horas_jogadas.parse().expect("esperava um número");
                array.push(Jogo{
                    nome:nome.clone(), 
                    horas_jogadas:num_horas_jogadas,
                });
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
        }
    }
    array = bubble_sort(array.clone());

    return array;
}



pub fn leitura_comeco(caminho: &str) ->Vec<card> {
    let contents = fs::read_to_string(caminho)
        .expect("deveria ter lido o arquivo");
    let mut nome_bool = true;
    let mut primeira = true;
    let mut nome = String::new();
    let mut genero = String::new();
    let mut array:Vec<card> = Vec::new();

    for  c in contents.chars() {
        if c == ','{
            nome_bool = !nome_bool;
        }else if c == '\n'{
            if primeira{
                primeira = false;
            } else{
                array.push(card{
                    nome:nome.clone(), 
                    genero:genero.clone(),
                });
            }
            nome_bool = !nome_bool;
            nome = String::new();
            genero = String::new();
            
        }
        else{
            if nome_bool{
                nome.push(c);
            } else {
                genero.push(c);
            }
        }
    }
    let mut rgn = rand::thread_rng();
    let mut retorno:Vec<card> = Vec::new();

    let mut num = 0;
    for i in 0..10 {
        num = rgn.gen_range(0..array.len()+1);
        retorno.push(array[num].clone());
    }

    retorno
}