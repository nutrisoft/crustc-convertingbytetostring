fn main() {
    let bytes: Vec<u8> = vec![
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64,
    ];
    match String::from_utf8(bytes) {
        Ok(string) => println!("{}", string),
        Err(e) => println!("Invalid UTF-8 sequence: {}", e),
    }

    let bytes_lossy: Vec<u8> = vec![0xF0, 0x90, 0x80];
    let string = String::from_utf8_lossy(&bytes_lossy);
    println!("{}", string); 
}
