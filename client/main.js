import init, { locz_init, WasmGameState } from './locz/loczwasm.js';
function elem_id_lookup(id) { return document.getElementById(id); }
let game = undefined;

await init();
locz_init();
elem_id_lookup('startbutton').innerText = 'Start';
elem_id_lookup('startbutton').classList.remove('startbuttonloading');

elem_id_lookup('startbutton').addEventListener('click', function() {
    if (elem_id_lookup('debugmode').checked) {
        elem_id_lookup('debugpane').classList.remove('hidden');
    }
    elem_id_lookup('start').classList.add('hidden');

    let scale = elem_id_lookup('scalefactor').value / 100;
    let viewport = elem_id_lookup('viewport');
    viewport.width = scale * viewport.clientWidth;
    viewport.height = scale * viewport.clientHeight;
    let ctx = viewport.getContext('2d');
    
    game = new WasmGameState(viewport.width, viewport.height, ctx);
    window.log_message('Dims: ' + viewport.width + 'x' + viewport.height);
    register_handlers();
    render_frame();
});

function register_handlers() {
}

function render_frame() {
    window.requestAnimationFrame(render_frame);
    let frame_start = window.performance.now();
    game.frame(frame_start);
    let frame_end = window.performance.now();
    update_hud(frame_end);
}

let last_frame = window.performance.now();
let frame_count = 0;
function update_hud(frame_end) {
    frame_count += 1;
    if ((frame_end - last_frame < 750) || (frame_count < 5)) {
        return;
    }
    let fps = frame_count * 1000 / (frame_end - last_frame);
    frame_count = 0;
    last_frame = frame_end;
    elem_id_lookup('hud').innerText = 'FPS: ' + fps.toFixed(1);
}