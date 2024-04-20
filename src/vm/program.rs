pub struct Program {
    code: Vec<u8>,
}

impl Program {
    pub fn new(code: &[u8]) -> Self {
        Program {
            code: code.to_vec(),
        }
    }
    pub fn slice_from(&self, start: usize) -> &[u8] {
        &self.code[start..]
    }

    pub fn size(&self) -> usize {
        self.code.len()
    }
}
