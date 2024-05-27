
function setFullScreen() {
    const canvas = document.querySelector('mycanvas');
    if (canvas) {
        if (canvas.requestFullscreen) {
            canvas.requestFullscreen();
        } else if (canvas.mozRequestFullScreen) { /* Firefox */
            canvas.mozRequestFullScreen();
        } else if (canvas.webkitRequestFullscreen) { /* Chrome, Safari & Opera */
            canvas.webkitRequestFullscreen();
        } else if (canvas.msRequestFullscreen) { /* IE/Edge */
            canvas.msRequestFullscreen();
        }
    }
}

import init, { greet, run_bevy_app } from './hello_wasm.js';
async function run() {
    await init()
    console.log(greet())
    try {
        run_bevy_app("#mycanvas");
    } catch (error) {
        if (!error.message.startsWith("Using exceptions for control flow,")) {
            throw error;
        } else {
            console.warn("Ignoring error:", error.message);
        }
    }
}
run();
