pub fn encode(data: &[u8]) -> Vec<u8> {
    // In a real scenario, this would contain the RLE encoding logic
    println!("Encoding {} bytes...", data.len());
    // Simple example: just clone the data (replace with actual RLE)
    let mut encoded = Vec::new();
    if data.is_empty() {
        return encoded;
    }
    let mut count = 1;
    let mut current_byte = data[0];
    for &byte in &data[1..] {
        if byte == current_byte {
            count += 1;
        } else {
            encoded.push(current_byte);
            encoded.push(count as u8); // Assuming count fits in u8 for simplicity
            current_byte = byte;
            count = 1;
        }
    }
    encoded.push(current_byte);
    encoded.push(count as u8);
    encoded
}

// rle decode function
pub fn decode(data: &[u8]) -> Vec<u8> {
    println!("Decoding {} bytes...", data.len());
    // Simple example: assumes pairs of (count, byte)
    let mut decoded = Vec::new();
    let mut i = 0;
    while i < data.len() {
        if i + 1 < data.len() {
            let byte = data[i];
            let count = data[i + 1] as usize;
            for _ in 0..count {
                decoded.push(byte);
            }
            i += 2;
        } else {
            // Handle incomplete pair (error or specific logic)
            eprintln!("Warning: Malformed RLE data at the end.");
            break;
        }
    }
    decoded
}