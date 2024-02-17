use super::buffer::Buffer;

pub struct History {
    pub data: Vec<Buffer>,
    pub size: usize,
    pub search_index: usize,
}
impl History {
    pub fn new() -> History {
        History {
            data: Vec::new(),
            size: 0,
            search_index: 0,
        }
    }

    pub fn add_buffer(&mut self, buffer: Buffer) {
        self.data.push(buffer);
        self.size = self.data.len();
        self.search_index = self.size.clone();
    }
}
