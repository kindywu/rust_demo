const MSG: &'static str = "Hello 你好";
static MSG2: &'static str = "Hello 你好";

fn main() {
    println!("MSG={}", MSG);

    println!("MSG2={}", MSG2);

    let mut msg3: &'static str = "Hello 你好";
    println!("msg3={}", msg3);
    msg3 = "Hello 世界";
    println!("msg3={}", msg3);

    str_to_other();
    string_to_other();
    u8_to_other();
    let vec_str: Vec<u8> = vec![104, 101, 108, 108, 111, 32, 228, 189, 160, 229, 165, 189];
    let str = std::str::from_utf8(&vec_str).unwrap();
    println!("{}", str);
    let string = String::from_utf8(vec_str.clone()).unwrap();
    println!("{}", string);
    let byte_str = vec_str.as_slice();
    println!("{:?}", byte_str);
}

fn str_to_other() {
    println!("str to other");
    let str = "hello 你好";
    // &str -> String
    let mut string = String::from(str);
    println!("{}", string);
    string = str.to_string();
    println!("{}", string);
    string = str.to_owned();
    println!("{}", string);
    // &str -> &[u8] space ascii is 32
    let byte_str = str.as_bytes();
    println!("{:?}", byte_str);
    let vec_str = str.as_bytes().to_vec();
    // &str -> Vec<u8>
    println!("{:?}", vec_str);
    let vec_str = str.as_bytes().to_owned();
    println!("{:?}", vec_str);
}

fn string_to_other() {
    println!("String to other");
    let string = String::from("hello 你好");
    // String -> &str
    println!("{}", string);
    let mut str: &str = &string;
    println!("{}", str);
    str = string.as_str();
    println!("{}", str);
    // String -> &[u8]
    let byte_str = string.as_bytes();
    println!("{:?}", byte_str);
    // String -> Vec<u8>
    let vec_str = string.into_bytes();
    println!("{:?}", vec_str);
}

fn u8_to_other() {
    let byte_str: &[u8] = &[104, 101, 108, 108, 111, 32, 228, 189, 160, 229, 165, 189];
    //&[u8]->&str
    let str = std::str::from_utf8(byte_str).unwrap();
    println!("{}", str);
    //&[u8]->String
    let string = String::from_utf8_lossy(byte_str);
    println!("{}", string);
    //&[u8]->Vec<u8>
    let vec_str = byte_str.to_vec();
    println!("{:?}", vec_str);
}
