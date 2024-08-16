const { invoke } = window.__TAURI__.tauri;

async function FetchJogos() {
    try {
        const jogos = await invoke('get_jogos');
        const container = document.getElementById("jogos");
        jogos.forEach(jogo => {
        const div_card = document.createElement('div');
        const img = document.createElement('img')
        img.src = "assets/celeste.jpg"
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
    const name = "Tauri User"; // ou você pode pegar o nome de um input
    console.log('ola');
    const greeting = await invoke("teste", { num:1});
    document.getElementById("greeting").innerText = greeting;
});


function colocacao(){
    container = document.getElementById("jogos")
    filhos = container.children;
    ranking = Array(5).fill("");
    for (var i=0; i<filhos.length; i++){
        posicao = filhos[i].getElementsByTagName("select")[0].value;
        ranking[posicao-1] = filhos[i].getElementsByTagName("h3")[0].textContent;
    }
    resp = document.getElementById("greeting");
    resp.innerText = "";
    for (var i=0; i<5;i++){
        resp.innerText += ranking[i] + "; ";
    }
}
