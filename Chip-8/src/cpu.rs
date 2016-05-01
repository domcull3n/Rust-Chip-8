

pub struct CPU
{
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u16,
    delay_timer: u8,
    sound_timer: u8
}

impl CPU
{
    pub fn new()-> CPU
    {
        CPU
        {
            pc: 0x200,
            opcode: 0,
            i: 0,
            sp: 0,
            memory: [0; 4096],
            v: [0; 16];
            stack: [0; 16],
            sound_timer: 0,
            delay_timer: 0
        }
    }

}
