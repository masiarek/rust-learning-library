fn main() {
    let le = [0x68, 0x00, 0x69, 0x00];
    println!("{:?}", String::from_utf16le_lossy(&le));

    // A trailing odd byte is replaced, not rejected.
    let truncated = [0x68, 0x00, 0x69];
    println!("{:?}", String::from_utf16le(&truncated).is_err());
    println!("{:?}", String::from_utf16le_lossy(&truncated));

    // An unpaired surrogate, likewise.
    let broken = [0x68, 0x00, 0x3D, 0xD8];
    println!("{:?}", String::from_utf16le_lossy(&broken));
}
