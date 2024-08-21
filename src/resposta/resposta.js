const { invoke } = window.__TAURI__.tauri;

window.onload = function() {
    arrayString = localStorage.getItem('ranking');
    array = JSON.parse(arrayString);
    console.log(array);
    
    try {
        ver_resposta(arrayString)
    }catch{
        console.log("não conseguiu chamar o rust");
    }
}

async function ver_resposta(array) {
    const resposta = await invoke("get_resposta", {jogos: array});
    elemento = document.getElementById("titulo");
    elemento.innerText = `jogo recomendado: ${resposta.nome}`
    elemento = document.getElementById("foto_jogo");
    caminho = resposta.nome.replaceAll(" ", "_").toLowerCase()
    elemento.src = `../assets/${caminho}.jpg`
}


function replaceAll(str, find, replace) {
    return str.replace(new RegExp(escapeRegExp(find), 'g'), replace);
  }
  