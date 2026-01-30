mod buffer;
use buffer::GapBuffer;
fn main() {       
    // create empty buffer and verify length is 0 
    let buf1 = GapBuffer::new();
    println!("{}", buf1.len());

    // create a buffer from hello, verify length is 5
    let buf2 = GapBuffer::from_str("hello");
    println!("{}", buf2.len());

    println!("{}", buf2.to_string());

    println!("{},{},{}", buf2.get_buffer_len(), buf2.get_gap_start(), buf2.get_gap_end());
}

