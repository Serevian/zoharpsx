use cpu::Cpu;

// Investigate using a scheduler because some components run at different clock speeds
// Also, make everything state serializable. This way implementing save states is easy as fuck
pub struct Psx<B: cpu::Backend, R: gpu::Renderer> {
    pub cpu: Cpu,
    pub backend: B,
    pub renderer: R,
}
