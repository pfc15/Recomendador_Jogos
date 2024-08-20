window.onload = function() {
    arrayString = localStorage.getItem('ranking');
    array = JSON.parse(arrayString);
    console.log(array);
    elemento = document.getElementById("titulo");
    for (var i =0;i<5;i++){
        elemento.innerText += array[i] + '\n';
    }
}


