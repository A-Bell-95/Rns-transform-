use rns_rs::Rns;

fn main() {
    let x = 10;
    println!("Десятичное представление: {:?}", x);
    let rns = Rns::<10>::new(x);
    println!(
        "RNS представление в базисе [3, 5, 7, 11, 13, 17, 19, 23, 29, 31]: {:?}",
        rns
    );
    let reconstructed = rns.into_u64();
    println!("Восстановленное десятичное: {}", reconstructed);
}
