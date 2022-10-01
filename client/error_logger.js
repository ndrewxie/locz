export function add_message(msgtype, msgorigin, message) {
    let row = document.createElement('tr');

    let type_td = document.createElement('td');
    type_td.innerText = msgtype;
    row.appendChild(type_td);

    let origin_td = document.createElement('td');
    origin_td.innerText = msgorigin;
    row.appendChild(origin_td);

    let message_td = document.createElement('td');
    message_td.innerText = message;
    row.appendChild(message_td);

    document.getElementById('debugmsgs').appendChild(row);
}

export function pause_panic() {
    let pauseoverlay = document.getElementById('pauseoverlay');
    pauseoverlay.innerText = 'Panicked';
    pauseoverlay.classList.remove('hidden');
}