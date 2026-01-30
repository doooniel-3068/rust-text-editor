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
        vec.resize(vec.len() + 32, 0);
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
        
        // before goes from o = gap_start -1
        //
        // after goes from gap_end to buffer len
        
        let mut result = Vec::new();
        result.extend(&self.buffer[0..self.gap_start]);
        result.extend(&self.buffer[self.gap_end..]);

        String::from_utf8_lossy(&result).to_string()

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

}







