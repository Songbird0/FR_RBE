// `Centimeters` est un tuple qui peut être comparé.
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches` est un tuple qui peut être affiché.
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds` est un tuple ne possédant aucun attribut.
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Erreur: `Seconds` ne peut pas être affiché; Il n'implémente pas le trait `Debug`.
    // println!("One second looks like: {:?}", _one_second);
    // TODO ^ Essayez de décommenter cette ligne.

    // Erreur: `Seconds` ne peut pas être comparé; Il n'implémente pas le trait `PartialEq`.
    // let _this_is_true = (_one_second == _one_second);
    // TODO ^ Essayez de décommenter cette ligne.

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
