fn main() {
    let s1 = String::from("Hello");

    let mut len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = s1; // can't use s1 now
    change(&mut s2);

    len = calculate_length(&s2);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!")
}
