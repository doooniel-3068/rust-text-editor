fn main() {
    
    struct TextBuffer {
        content: String,

    }



    impl TextBuffer {
        fn new() -> TextBuffer {
            TextBuffer {content: String::new()} 

        }

        fn len(&self) -> usize{
            self.content.chars().count()

        }


        fn from_file(path: &str) -> Result<TextBuffer, std::io::Error> {
           let contents = std::fs::read_to_string(path)?;
           Ok(TextBuffer {content: contents}) 
        }

    }
    let buffer = TextBuffer::from_file("test.txt");
    match buffer {
        Ok(buf) => println!("Success! Length: {}", buf.len()),
        Err(e) => println!("Failed to load: {}", e),
    }

}

