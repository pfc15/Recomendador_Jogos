const { invoke } = window.__TAURI__.tauri;

window.onload = function() {
    arrayString = localStorage.getItem('ranking');
    array = JSON.parse(arrayString);
    console.log(array);
    elemento = document.getElementById("titulo");
    for (var i =0;i<5;i++){
        elemento.innerText += array[i] + '\n';
    }

    try {
        ver_resposta(arrayString)
    }catch{
        console.log("não conseguiu chamar o rust");
    }
    

}

async function ver_resposta(array) {
    const resposta = await invoke("get_resposta", {jogos: array});
    console.log(resposta)
}
