var socket;

document.addEventListener("DOMContentLoaded", function(event) {

    // Create WebSocket connection.
    socket = new WebSocket("ws://localhost:3000/ws");

    // Connection opened
    socket.addEventListener("open", (event) => {});

    // Listen for messages
    socket.addEventListener("message", (event) => {
        console.log("Message from server: ", event.data);
        append_message(event.data, false);
    });

    // send user messages
    const form = document.getElementById('send-message-form');
    const input = form.querySelector("input[name=message]");

    form.addEventListener('submit', (e) => {
        e.preventDefault();

        let msg = input.value.trim();
        if (!msg) return;

        send_message(msg);
        input.value = "";
    });
});


function send_message(msg){
    socket.send(msg);
    append_message(msg, true);
};



function append_message(text, outgoing){
    const msg = document.createElement("p");
    msg.textContent = text;
    msg.className = outgoing ? "msg out" : "msg in";
    chat.appendChild(msg);
    chat.scrollTop = chat.scrollHeight;
}

