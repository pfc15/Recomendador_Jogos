const { invoke } = window.__TAURI__.tauri;

document.getElementById("proximo").addEventListener("click", async () => {
    const name = "Tauri User"; // ou você pode pegar o nome de um input
    console.log('ola');
    const greeting = await invoke("teste", { num:1});
    document.getElementById("greeting").innerText = greeting;
});

