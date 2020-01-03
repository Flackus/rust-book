use add_one;
use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(1, 11);
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
