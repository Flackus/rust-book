fn main() {
    let s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = s.to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    let s2 = "bar";

    s.push_str(s2);
    println!("s2 is {}", s2);

    s.push('b');
    s.push('a');
    s.push('z');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    println!("s3 is {}", s3);
    println!("s2 is {}", s2);
    // won't compile: borrow of moved value
    //println!("s1 is {}", s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
    println!("s1 is {}", s1);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s is {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
