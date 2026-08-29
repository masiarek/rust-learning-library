fn main() {
    let good = vec![104, 105];
    println!("{:?}", String::from_utf8(good));

    // The Vec comes back on failure -- the data is not lost.
    let bad = vec![104, 0xff, 105];
    match String::from_utf8(bad) {
        Ok(s) => println!("{s:?}"),
        Err(e) => {
            println!("failed at byte {}", e.utf8_error().valid_up_to());
            println!("recovered {:?}", e.into_bytes());
        }
    }

    // Truncated rather than invalid: error_len is None.
    let cut = "é".as_bytes()[..1].to_vec();
    println!("{:?}", String::from_utf8(cut).unwrap_err().utf8_error().error_len());

    // It moves rather than copying: the Vec is consumed.
    let owned = "héllo".as_bytes().to_vec();
    let text = String::from_utf8(owned).unwrap();
    println!("{text:?} {} bytes", text.len());
}
