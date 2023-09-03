fn main() {
    let string = String::from("Hello World!!");
    let string_slice = &string[10..];

    dbg!(&string);
    dbg!(string_slice);
}

