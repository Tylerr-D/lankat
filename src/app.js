const socket = new WebSocket(`ws://${location.host}/ws`);
const box = document.getElementById('box');
const log = document.getElementById('log');

function send(){
    socket.send(box.value);
    box.value = '';
}

socket.onopen = () => {
    log.textContent += 'connected\n';
};

socket.onmessage = (e) => {
    log.textContent += e.data + '\n';
};

socket.onclose = () => {
    log.textContent += 'disconnected\n';
};
