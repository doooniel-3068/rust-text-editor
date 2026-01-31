mod buffer;
use buffer::GapBuffer;
fn main() {
    println!("=== Checkpoint 4.2 Tests ===\n");
    
    // Test 1: Create buffer with "Hello"
    let mut buffer = GapBuffer::from_str("Hello");
    println!("1. Initial buffer: '{}'", buffer.to_string());
    
    buffer.insert(0, 'X');
    println!("2. After insert 'X' at 0: '{}'", buffer.to_string());
    
    buffer.insert(3, 'Y');
    println!("3. After insert 'Y' at 3: '{}'", buffer.to_string());
    
    let deleted = buffer.delete_range(1, 3);
    println!("4. After delete range (1,3): '{}'", buffer.to_string());
    println!("   Deleted text: '{}'", deleted);
    
    buffer.insert_str(5, "WORLD");
    println!("5. After insert 'WORLD' at 5: '{}'", buffer.to_string());
    
}

