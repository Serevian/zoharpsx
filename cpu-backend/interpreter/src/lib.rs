use bus::Bus;
use cpu::Backend;

pub struct Interpreter;

impl Backend for Interpreter {
    type Error = InterpreterError;

    fn step(&mut self, state: &mut cpu::Cpu, bus: &mut Bus) -> Result<u32, Self::Error> {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InterpreterError {
    #[error("Illegal instruction opcode={opcode:#04x} at pc={pc:#010x}")]
    IllegalInstruction { opcode: u32, pc: u32 },
}
