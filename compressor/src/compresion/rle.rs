pub fn encode(data: &[u8]) -> Vec<u8> {
    let mut encoded: Vec<u8> = Vec::new();
    let mut count: u8 = 1;

    for i in 1..data.len() {
        if data[i] == data[i - 1] {
            count += 1;
        } else {
            encoded.push(data[i - 1]);
            encoded.push(count);
            count = 1;
        }
    }

    // Handle the last run
    if !data.is_empty() {
        encoded.push(data[data.len() - 1]);
        encoded.push(count);
    }

    encoded
}

pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::new();

    for i in (0..data.len()).step_by(2) {
        let value = data[i];
        let count = data[i + 1];
        decoded.extend(vec![value; count as usize]);
    }

    decoded
}
