use winnow_learn::Hex;



fn main() {
    let input = "0x1a2b";
    assert_eq!(input.parse::<Hex>().unwrap(), Hex(0x1a2b));

    let input = "0x1a2b Hello";
    assert!(input.parse::<Hex>().is_err());
    let input = "ghiHello";
    assert!(input.parse::<Hex>().is_err());
}
