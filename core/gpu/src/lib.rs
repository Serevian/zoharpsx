// Instead of sending the framebuffer to the renderer, what about sending the gpu commands to them?
// This way, each renderer can render (duh) in its own way
pub trait Renderer {
    fn render_frame();
}

pub struct Framebuffer {
    pub pixels: Vec<u32>,
}
