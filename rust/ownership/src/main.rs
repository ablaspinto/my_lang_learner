fn caclulate_length(s: &mut String) -> usize {
    let length: usize = s.len();
    length
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let mut str: String = String::from("hello world");
    let hello: &String = &str[0..5];
    let world: &String = &str[6..];
    let len: usize = caclulate_length(&mut str);
    println!("The string: {} and the length is: {}", str, len);
    let len = first_word(&str);
    
    println!("{}",len);
}
