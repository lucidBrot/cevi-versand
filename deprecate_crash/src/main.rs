fn main() {
    println!("main.");
    foo();
}

#[deprecated(note=test)]
fn foo(){
    println!("Hello, world");
}
