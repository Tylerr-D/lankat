const box = document.getElementById('box');
const chat = document.getElementById('chat');
const status = document.getElementById('status');
let ws;

function connect(){
    ws = new WebSocket(`ws://${location.host}/ws`);
    status.textContent = 'connecting...';
    status.className = '';


ws.onopen = () => {
    status.textContent = 'connected';
    status.className='on';
};

ws.onmessage = (e) => {
    addMsg(e.data, 'in')
};

ws.onclose = () => {
    status.textContent += 'offline - retrying in 2s';
    setTimeout(connect, 2000);
};


box.onkeydown = (e) => {
    if (e.key === 'Enter' && box.value.trim()){
        ws.send(box.value.trim());
        addMsg(box.value.trim(), 'me');
        box.value = '';
  }
 };
}

function addMsg(text, who){
    const div = document.createElement('div');
    div.className = who;
    div.textContent = text;
    chat.appendChild(div);
    chat.scrollTop = chat.scrollHeight;
}

connect();