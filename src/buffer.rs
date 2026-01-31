pub struct GapBuffer {
    buffer: Vec<u8>,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    pub fn new() -> Self {
       GapBuffer {
            buffer: vec![0;32],
            gap_start: 0,
            gap_end: 32,
        } 
    }


    pub fn from_str(text: &str) -> Self {
        let mut vec = Vec::from(text.as_bytes());
        vec.resize(vec.len() + 64, 0);
        let len = text.len();

        GapBuffer {
            buffer: vec,
            gap_start: len,
            gap_end: len+32,
        }
    }

    pub fn len(&self)-> usize {
        self.buffer.len() - (self.gap_end - self.gap_start)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_string(&self) -> String {

        let mut result = Vec::new();
        result.extend(&self.buffer[0..self.gap_start]);

        result.extend(&self.buffer[self.gap_end..]);

        String::from_utf8_lossy(&result).to_string()

    }

    pub fn grow_gap(&mut self){
        let current_gap = self.gap_end - self.gap_start;
        let new_gap = current_gap * 2;
        let bytes_to_add = new_gap - current_gap;

        // insert empty space into gap to make room
        self.buffer.splice(self.gap_end..self.gap_end,vec![0; bytes_to_add]);
        self.gap_end += bytes_to_add;
    }

    pub fn move_gap(&mut self, pos: usize){

        if pos < self.gap_start {
            let amount = self.gap_start - pos;
            let destination = self.gap_end - amount;

            self.buffer.copy_within(pos..self.gap_start, destination);

            self.gap_start = pos;
            self.gap_end = destination;
             
        }

        else if pos>self.gap_start {
            let amount = pos - self.gap_start;
            let src_start = self.gap_end;
            let src_end = self.gap_end + amount;

            self.buffer.copy_within(src_start..src_end, self.gap_start);

            self.gap_start = pos;
            self.gap_end = src_end;
        }

    }



    pub fn insert(&mut self, pos:usize, ch:char){
        self.move_gap(pos);
        if self.gap_start == self.gap_end {
            self.grow_gap();
        }
        let mut b = [0; 4];
        let ch_bytes = ch.encode_utf8(&mut b).as_bytes();
        self.buffer[self.gap_start..self.gap_start + ch_bytes.len()].copy_from_slice(ch_bytes);
        self.gap_start += ch_bytes.len();
    }

    pub fn delete_range(&mut self, start:usize, end:usize) -> String {
        self.move_gap(start);
        let actual_end = self.gap_end - self.gap_start + end;
        let deleted_text = String::from_utf8_lossy(&self.buffer[self.gap_end..actual_end]).to_string();
        self.gap_end = actual_end;
        deleted_text
    }

    pub fn insert_str(&mut self, pos: usize, text: &str){
        let textlen = text.len();
        let text_bytes = text.as_bytes();
        self.move_gap(pos);
        while (self.gap_end-self.gap_start)< textlen {
            self.grow_gap();
        }
        self.buffer[self.gap_start..self.gap_start+textlen].copy_from_slice(text_bytes);
        self.gap_start += textlen;
    }

    // getters

    pub fn get_buffer_len(&self) -> usize {
        self.buffer.len()
    } 

    pub fn get_gap_start(&self) -> usize {
        self.gap_start
    } 

    pub fn get_gap_end(&self) -> usize {
        self.gap_end
    }

    pub fn print_internal_state(&self){
        println!("{},{},{}", self.get_buffer_len(), self.get_gap_start(), self.get_gap_end());
    }

}







