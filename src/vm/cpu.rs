pub struct CPU<T> {
    registers: [T; 4],
    status_flags: StatusFlags,
    pc: usize,
}

impl<T> CPU<T>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        Self {
            registers: [T::default(); 4],
            status_flags: StatusFlags::default(),
            pc: 0,
        }
    }

    pub fn run(&mut self) {}
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StatusFlags {
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,
    pub negative: bool,
}
