fn caclulate_length(s: &mut String) -> usize {
    let length: usize = s.len();
    length
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

enum IpAddrKind{
    V4,
    V6,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}  

fn main() {
    let mut s: String = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..];
    let len: usize = caclulate_length(&mut s);
    println!("The string: {} and the length is: {}", s, len);
    let len = first_word(&s);
    let four = IpAddrKind::V4;
    let six =IpAddrKind::V6;
    let localhost = IpAddr{
        kind: IpAddrKind::V4,
        address = String::from("127.0.0.1");

    }


    println!("{}", len);
}

