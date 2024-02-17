#[derive(Clone)]
pub struct Buffer {
    pub data: String,
    pub data_size: u16,
    pub prefix: String,
    pub prefix_size: u16,
    pub buffer_size: u16,
}

impl Buffer {
    pub fn new(prefix_in: String, prefix_size_in: u16) -> Buffer {
        Buffer {
            data: String::new(),
            data_size: 0,
            prefix: prefix_in,
            prefix_size: prefix_size_in,
            buffer_size: prefix_size_in,
        }
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
        self.buffer_size -= self.data_size;
        self.data_size = 0;
    }
    pub fn write_data(&mut self, content: char, pos: u16) {
        self.data_size = self.data.len() as u16;
        if pos < self.prefix_size{
            self.data
                .insert(pos as usize, content);
        } else{
        self.data
            .insert(pos as usize - (self.prefix_size as usize), content);
        }
        self.data_size += 1;
        self.buffer_size += 1;
    }
    pub fn remove_char(&mut self, pos: u16) {
        self.data_size = self.data.len() as u16;
        if pos >= self.buffer_size {
            match self.data.pop() {
                Some(_) => 0,
                None => 0,
            };
        } else {
            self.data
                .remove(pos as usize - self.prefix_size as usize - 1);
        }
        self.data_size -= 1;
        self.buffer_size -= 1;
    }
}
