
fn hello_world() {
    println!("Hello World")
}

fn hello(world: &str) {
    println!("Hello {}", world)
}

fn main() {
    hello("earth")
    hello_world()
}
