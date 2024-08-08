use std::collections::HashMap;

#[derive(Clone)]
pub struct Jogo {
    pub nome: String,
    pub horas_jogadas: i32,
}



pub struct MergeSort {
    pub comparado: Vec<Jogo>,
    hash_posicoes: HashMap<String, usize>,
    pub inversoes: usize,
}

impl MergeSort {
    pub fn new(vetor_modelo:Vec<Jogo>, vetor_comparado: Vec<Jogo>) -> Self {
        let mut hash_posicoes = HashMap::new();
        for (i, jogo) in vetor_modelo.iter().enumerate() {
            hash_posicoes.insert(jogo.nome.clone(), i);
        }
        let mut merge_sort = MergeSort {
            comparado: vetor_comparado,
            hash_posicoes,
            inversoes: 0,
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
            temp.push(self.comparado[min].clone());
            min +=1;
        }
        while max != index_max {
            temp.push(self.comparado[max].clone());
            max +=1;
        }

        for (i, jogo) in temp.into_iter().enumerate() {
            self.comparado[i + index_min] = jogo;
        }

    }
}

