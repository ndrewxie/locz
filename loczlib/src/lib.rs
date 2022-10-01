mod assets;
mod renderer;

pub struct GameState {
    pub renderer: renderer::Renderer,
}
impl GameState {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            renderer: renderer::Renderer::new(screen_width, screen_height),
        }
    }
    pub fn frame(&mut self, time: u128) {
        self.renderer.render(time);
    }
}
