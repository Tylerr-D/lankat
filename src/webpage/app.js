const box = document.getElementById('box');
const chat = document.getElementById('chat');
const status = document.getElementById('status');
const sendBtn = document.getElementById('send');
let ws;

function send() {
    const text = box.value.trim();

    if (!text || ws.readyState !== WebSocket.OPEN) {
        return
    }

    ws.send(text);
    addMsg(text, 'me');
    box.value = '';
    box.focus();
}

function addMsg(text, who) {
    const div = document.createElement('div');
    div.className = who;

    const label = document.createElement('span');
    label.className = 'who';

    const time = new Date().toTimeString().slice(0, 5);
    label.textContent = (who === 'me' ? 'me' : 'peer') + ' - ' + time;

    const body = document.createElement("span");
    body.textContent = text;
    div.append(label, body);

    chat.appendChild(div);
    chat.scrollTop = chat.scrollHeight;
}


function connect() {
    ws = new WebSocket(`ws://${location.host}/ws`);
    status.textContent = 'connecting...';
    status.className = '';


    ws.onopen = () => {
        status.textContent = 'connected';
        status.className = 'on';
    };

    ws.onmessage = (e) => {
        addMsg(e.data, 'in')
    };

    ws.onclose = () => {
        status.textContent += 'offline - retrying in 2s';
        setTimeout(connect, 2000);
    };


    box.onkeydown = (e) => {
        if (e.key === 'Enter')
            send();
    };

}

sendBtn.onclick = send;

connect();

