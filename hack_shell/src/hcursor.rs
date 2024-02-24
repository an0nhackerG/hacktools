use termion::cursor;

pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub max_x: u16,
    pub max_y: u16,
}

impl Cursor {
    pub fn new(x_in: u16, y_in: u16) -> Cursor {
        let (max_x, max_y) = termion::terminal_size().unwrap(); 
        Cursor { x: x_in, y: y_in, max_x, max_y}
    }
    pub fn move_position(&mut self, x_in: u16, y_in: u16) {
        self.x = x_in;
        self.y = y_in;
    }
    pub fn move_forward(&mut self) {
        if self.x == self.max_x{
            self.move_position(0, self.y + 1);
        } else{
            self.move_position(self.x + 1, self.y)
        }
    }
    pub fn move_backward(&mut self) {
        if self.x == 0{
            self.move_position(self.max_x, self.y - 1);
        } else{
            self.move_position(self.x - 1, self.y)
        }
    }
    pub fn move_x_to_start(&mut self) {
        self.move_position(1, self.y)
    }
    pub fn move_y_to_start(&mut self) {
        self.move_position(self.x, 1)
    }
    pub fn move_cursor_to(&mut self, x_in: u16, y_in: u16) -> cursor::Goto {
        self.move_position(x_in, y_in);
        cursor::Goto(self.x, self.y)
    }
}
