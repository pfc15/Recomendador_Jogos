use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct Jogo {
    pub nome: String,
    pub horas_jogadas: i32,
}

// Implement `Ord` and `PartialOrd` so the `BinaryHeap` can order `Task` by priority
impl Ord for Jogo {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we reverse the order to make the heap a max-heap by priority
        other.horas_jogadas.cmp(&self.horas_jogadas)
    }
}

impl PartialOrd for Jogo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
pub struct MergeSort {
    pub comparado: Vec<Jogo>,
    pub hash_posicoes: HashMap<String, usize>,
    pub inversoes: usize,
    pub tamanho: usize,
    pub recomendacoes: BinaryHeap<Jogo>
}

impl MergeSort {
    pub fn new(vetor_modelo:Vec<Jogo>, vetor_comparado: Vec<Jogo>) -> Self {
        let mut hash_posicoes = HashMap::new();
        let tamanho = vetor_modelo.len();
        for (i, jogo) in vetor_modelo.iter().enumerate() {
            hash_posicoes.insert(jogo.nome.clone(), i);
        }
        let mut merge_sort = MergeSort {
            comparado: vetor_comparado,
            hash_posicoes,
            inversoes: 0,
            tamanho: tamanho,
            recomendacoes: BinaryHeap::new()
        };
        
        merge_sort.sort(0, merge_sort.comparado.len());

        merge_sort
    }

    fn sort(&mut self, index_min:usize, index_max: usize ){
        let n =index_max - index_min;
        if n==1{
            return;
        }
        let meio = index_min +(n+1)/2;

        self.sort(index_min, meio);
        self.sort(meio, index_max);

        self.merge(index_min, meio, index_max);

    }
    
    fn merge(&mut self, index_min: usize, meio: usize, index_max: usize){
        let mut temp = Vec::new();
        let (mut min, mut max) = (index_min, meio);
        while min != meio && max != index_max{
            if !(self.hash_posicoes.contains_key(&self.comparado[min].nome)){
                self.hash_posicoes.insert(self.comparado[min].nome.clone(), self.tamanho);
                self.recomendacoes.push(self.comparado[min].clone());
            }
            if !(self.hash_posicoes.contains_key(&self.comparado[max].nome)){
                self.hash_posicoes.insert(self.comparado[max].nome.clone(), self.tamanho);
                self.recomendacoes.push(self.comparado[max].clone());
            }
            if self.hash_posicoes[&self.comparado[min].nome] < self.hash_posicoes[&self.comparado[max].nome]{
                temp.push(self.comparado[min].clone());
                min +=1;
            } else {
                temp.push(self.comparado[max].clone());
                max +=1;
                self.inversoes += meio-min;
            }
        }

        while min != meio {
            if !(self.hash_posicoes.contains_key(&self.comparado[min].nome)){
                self.hash_posicoes.insert(self.comparado[min].nome.clone(), self.tamanho);
                self.recomendacoes.push(self.comparado[min].clone());
            }
            temp.push(self.comparado[min].clone());
            min +=1;
        }

        while max != index_max {
            if !(self.hash_posicoes.contains_key(&self.comparado[max].nome)){
                self.hash_posicoes.insert(self.comparado[max].nome.clone(), self.tamanho);
                self.recomendacoes.push(self.comparado[max].clone());
            }
            temp.push(self.comparado[max].clone());
            max +=1;
        }

        for (i, jogo) in temp.into_iter().enumerate() {
            self.comparado[i + index_min] = jogo;
        }

    }
}


