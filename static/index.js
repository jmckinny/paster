function submit(){
    let text = document.getElementById("input_box").value;
    console.info("Got data: `" + text +"`");
    fetch("http://127.0.0.1:8000/paste/new",{
        method: 'POST',
        body: text
    }).then(response => response.text()).then(response => alert("Text can be found at\nhttp://localhost:8000/" + response));
}