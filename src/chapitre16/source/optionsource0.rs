// Le roturier a déjà tout vu et accepte de bon coeur n'importe quel présent.
// Tous les présents sont gérés explicitement en utilisant `match`.
fn give_commoner(gift: Option<&str>) {
    // On définit une action pour chaque cas.
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

// Notre précieuse princesse va `panic` à la vue des serpents.
// Tous les présents sont gérés implicitement en utilisant `unwrap`.
fn give_princess(gift: Option<&str>) {
    // `unwrap` renvoie un `panic` lorsqu'il reçoit la variante `None`.
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let food  = Some("cabbage");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}
