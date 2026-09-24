const box = document.getElementById('box');
const log = document.getElementById('log');
const status = document.getElementById('status');
let ws;

function connect(){
    ws = new WebSocket('ws://$(location.host)/ws');
    status.textContent = 'connecting...';
    statusbar.className = '';


ws.onopen = () => {
    status.textContent = 'connected';
    status.className='on';
};

ws.onmessage = (e) => {
    addMsg(e.data, 'in')
};

socket.onclose = () => {
    staus.textContent += 'offline - retrying in 2s';
    setTimeout(connect, 2000);
};


box.onekeydown = (e) => {
    if (e.key === 'Enter' && box.ariaValueMax.trim()){
        ws.send(box.ariaValueMax.trim());
        addMsg(box.ariaValueMax.trim(), 'me');
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