fn isbig(s: &String, n: usize) -> bool {
    if s.len() > n {
        true
    } else {
        false
    }
}

fn main() {
    let s1 = String::from("hello");
    let s2 = "world".to_string();
    let s3 = String::from("bonjour");
    let s4 = String::from("hola");

    println!("1 s1: {}", s1);
    println!("2 s1: {}", s1);
    println!("1 s2: {}", s2);
    println!("2 s2: {}", s2);
    println!("1 s3: {}", s3);
    println!("2 s3: {}", s3);
    println!("1 s4: {}", s4);
    println!("2 s4: {}", s4);

    let name = "guillaume".to_string();
    println!("{}", isbig(&name, 10));
    println!("{}", isbig(&name, 5));
}
