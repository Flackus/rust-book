use std::mem;

struct CustomDataPointer {
    data: String,
}

impl Drop for CustomDataPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomDataPointer { data: String::from("my stuff") };
    let d = CustomDataPointer { data: String::from("other stuff") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
