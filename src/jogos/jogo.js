const { invoke } = window.__TAURI__.tauri;
ranking = ["","","","",""];

async function FetchJogos() {
    try {
        const jogos = await invoke('get_jogos');
        const container = document.getElementById("jogos");
        jogos.forEach(jogo => {
        const div_card = document.createElement('div');
        div_card.id = jogo.nome
        const img = document.createElement('img')
        img.src = jogo.imagem
      const div = document.createElement('div');
      div_card.classList.add('card')
      div_card.appendChild(img);
      div_card.appendChild(div);
      
      
      // Create and append h3 element for the game name
      const h3 = document.createElement('h3');
      h3.textContent = jogo.nome;
      div.appendChild(h3);
      const quebra = document.createElement('br');
      div.appendChild(quebra);

      
      // Create and append p element for the genre
      const p = document.createElement('p');
      p.textContent = jogo.genero;
      div.appendChild(p);
      
      // Create and append select element
      const select = document.createElement('select');
      select.name = 'ranking';
      select.id = 'ranking';
      select.innerHTML = `
        <option value="0" selected>escolha sua posição</option>
        <option value="1">1°</option>
        <option value="2">2°</option>
        <option value="3">3°</option>
        <option value="4">4°</option>
        <option value="5">5°</option>
      `;
      select.addEventListener('change', colocacao);
      div.appendChild(select);
      
      // Append the div to the container
      container.appendChild(div_card);
    });
    }catch {
        console.error('deu ruim pegando  os jogos', error);
    }
}

window.onload = function() {
    FetchJogos();

}


document.getElementById("proximo").addEventListener("click", async () => {
    localStorage.setItem('ranking', JSON.stringify(ranking));
    window.location.href = '../resposta/resposta.html';
});


function colocacao(event){
    if (event && event.target){
        const disparador = event.target.parentNode;
        const disparador_nome = disparador.getElementsByTagName("h3")[0].innerText
        disparador_posicao = event.target.value
        container = document.getElementById("jogos")

        filhos = container.children;
        resp = document.getElementById("greeting");

        if (disparador_nome in ranking){
            ranking[ranking.indexOf(disparador_nome)] = ""
        }
        for (var i=0;i<filhos.length;i++){
            if (filhos[i].getElementsByTagName("select")[0].value == disparador_posicao && disparador_nome != filhos[i].getElementsByTagName("h3")[0].textContent){
                filhos[i].getElementsByTagName("select")[0].value = 0;
            }
        }
        
        ranking[disparador_posicao-1] = disparador_nome
        
        resp.innerText = "";
        for (var i=0; i<5;i++){
            resp.innerText += `${i+1}° `+ranking[i] + ";\n";
        }
    } else{
        console.log("deu ruim rapasiada");
    }
    
}
