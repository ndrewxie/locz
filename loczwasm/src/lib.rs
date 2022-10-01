use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

mod error_logger;
use loczlib::GameState;

#[wasm_bindgen]
pub fn locz_init() {
    error_logger::init_panic();
}

#[wasm_bindgen]
pub struct WasmGameState {
    state: GameState,
    canvas: CanvasRenderingContext2d,
}
#[wasm_bindgen]
impl WasmGameState {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, canvas: CanvasRenderingContext2d) -> Self {
        Self {
            state: GameState::new(width, height),
            canvas,
        }
    }
    pub fn frame(&mut self, time: usize) {
        self.state.frame(time as u128);
        let framebuffer = Clamped(self.state.renderer.framebuffer());
        let frame_image =
            ImageData::new_with_u8_clamped_array(framebuffer, self.state.renderer.raw_width as u32);
        self.canvas.put_image_data(&frame_image.unwrap(), 0.0, 0.0);
    }
}
