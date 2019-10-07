fn main() {
    another_function(5);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let z = five();

    println!("The value of z is: {}", z);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}
