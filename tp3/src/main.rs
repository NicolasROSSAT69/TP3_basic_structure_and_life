fn isbig(s: &String, n: usize) -> bool {
    if s.len() > n {
        true
    } else {
        false
    }
}

fn inverse(v: f32) -> f32 {
    1.0 / v
}

fn safe_inverse(v: f32) -> Option<f32> {
    if v == 0.0 {
        None
    } else {
        Some(1.0 / v)
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

    let v = Some(10);

    match v {
        Some(n) => println!("Pas vide : {}", n),
        None => println!("Vide"),
    }

    println!("Inverse de 2 : {}", inverse(2.0));
    println!("Inverse de 1 : {}", inverse(1.0));
    println!("Inverse de -1 : {}", inverse(-1.0));
    println!("Inverse de 0 : {}", inverse(0.0));

    println!("Inverse de 2 : {:?}", safe_inverse(2.0));
    println!("Inverse de 1 : {:?}", safe_inverse(1.0));
    println!("Inverse de -1 : {:?}", safe_inverse(-1.0));
    println!("Inverse de 0 : {:?}", safe_inverse(0.0));
}
