const box = document.getElementById('box');
const chat = document.getElementById('chat');
const status = document.getElementById('status');
const you = document.getElementById('you');
const peers = document.getElementById('peers');
const sendBtn = document.getElementById('send');
const clearBtn = document.getElementById('clear');
chat.innerHTML = localStorage.getItem('chat') || '';
let ws;

function send(){
    const text = box.value.trim();
    if (!text || ws.readyState !== WebSocket.OPEN) return;
    ws.send(text);
    addMsg(text, 'me');
    box.value = '';
    box.focus();
}

// this would be cool right?
// yk a notifiction if you are some other tab

function ping() {
    if (document.hidden){
        document.title = '(1) lankat';
        const ctx = new AudioContext();
        ctx.resume();
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.connect(gain).connect(ctx.destination);
        // this is the notification sound stuff
        osc.frequency.value = 880;
        gain.gain.value = 0.05;
        osc.start();
        osc.stop(ctx.currentTime + 0.15);
    }
}

function addMsg(text, who){
    ping();
    const div = document.createElement('div');
    div.className = who;
    const label = document.createElement('span');
    label.className = 'who';
    const time = new Date().toTimeString().slice(0, 5);
    label.textContent = (who === 'me' ? 'me' : 'peer') +' - ' + time;
    const body = document.createElement("span");
    body.textContent = text;
    div.append(label, body);
    chat.appendChild(div);
    chat.scrollTop = chat.scrollHeight;
    localStorage.setItem('chat', chat.innerHTML);
}


function connect(){
    ws = new WebSocket(`ws://${location.host}/ws`);
    status.textContent = 'connecting...';
    status.className = '';


ws.onopen = () => {
    status.textContent = 'connected';
    status.className='on';
    document.title = 'lankat';
    document.addEventListener('visibilitychange', () => {
        if (!document.hidden) document.title = 'lankat';
    });
};

ws.onmessage = (e) => {
    const m = e.data;
    if (m.startsWith('name:')) you.textContent = 'you: ' + m.slice(5);
    else if (m.startsWith('peers:')) peers.textContent = m.slice(6) + ' online';
    else addMsg(m, 'in');
};

ws.onclose = () => {
    status.textContent += 'offline - retrying in 2s';
    setTimeout(connect, 2000);
};


box.onkeydown = (e) => {
    if (e.key === 'Enter' && !e.shiftKey) { 
        e.preventDefault();
        send();
    }
};

};
sendBtn.onclick = send;

clearBtn.onclick = () => {
    chat.innerHTML = '';
    localStorage.removeItem('chat');
}

connect();
