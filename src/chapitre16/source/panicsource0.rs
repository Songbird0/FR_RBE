fn give_princess(gift: &str) {
    // Les princesses détestent les serpents, donc nous devons tout arrêter si elles n'acceptent pas !
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}
