


// Public Trait
pub trait Pricing {
    fn fetch_price(&self);
    fn save_to_file(&self);
}
// Structs for Bitcoin, Ethereum, SP500
struct Bitcoin {
    val: f64, 
}
struct Ethereum {
    val: f64,
}
struct SP500 {
    val: f64,
}

fn main() {
    println!("Hello, world!");
}
