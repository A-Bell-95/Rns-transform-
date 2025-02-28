use rns_rs::convert::{from_rns, to_rns};

fn main() {
    let x = 10;
    println!("Десятичное представление: {:?}", x);
    let rns = to_rns(x);
    println!("RNS представление: {:?}", rns);
    let reconstructed = from_rns(&rns);
    println!("Восстановленное десятичное: {}", reconstructed);
}
