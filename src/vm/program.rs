pub struct Program {
    code: Vec<u8>,
}

impl Program {
    pub fn new(code: Vec<u8>) -> Self {
        Self { code }
    }
    pub fn slice_from(&self, start: usize) -> &[u8] {
        &self.code[start..]
    }

    pub fn size(&self) -> usize {
        self.code.len()
    }
}
