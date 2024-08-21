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
}
