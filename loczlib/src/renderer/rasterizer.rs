use super::line;
use line::YSampledLine;

pub struct Texture {
    data: Vec<u8>,
    size: usize,
}
pub struct Rasterizer {
    texture: Texture,

    locz_deltas: Vec<f32>,
    scanline_interps: Vec<f32>,

    left_edges: Vec<YSampledLine>,
    active_left_edge: usize,
    right_edges: Vec<YSampledLine>,
    active_right_edge: usize,
}
