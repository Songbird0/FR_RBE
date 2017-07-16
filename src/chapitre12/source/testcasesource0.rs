use std::ops::Add;
use std::marker::PhantomData;

/// On créé des énumérations vides pour déclarer le type des 
// unités de mesures.
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` est une structure prenant un type générique fantôme `Unit`,
/// `f64` implémente déjà les traits `Clone` et `Copy`.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// Le trait `Add` définit le comportement de l'opérateur `+`.
impl<Unit> Add for Length<Unit> {
     type Output = Length<Unit>;

    // La méthode add() renvoie une nouvelle instance de la 
    // structure `Length` contenant la somme.
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // L'opérateur `+` appelle l'implémentation 
        // du trait `Add` pour le type `f64`.
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // On initialise `one_foot` pour avoir un type générique fantôme `Inch`.
    // Ce "fantôme" sert de marqueur et classe cette instance dans 
    // l'unité de mesure "Pied".
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    // On initialise `one_meter` pour avoir un type générique fantôme `Mm`.
    // Ce "fantôme" sert de marqueur et classe cette instance dans 
    // l'unité de mesure "Millimètre".
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // L'opérateur `+` appelle la méthode `add()` que nous avons 
    // précédemment implémentée pour `Length<Unit>`.
    // Maintenant que `Length` implémente le trait `Copy`, `add()` ne 
    // prend pas possession (ne consomme pas) `one_foot` et `one_meter` mais 
    // les copie dans `self` et `rhs`.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // L'addition fonctionne.
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // Les opérations illogiques échouent comme prévu:
    // Erreur à la compilation: mismatched type.
    //let one_feter = one_foot + one_meter;
}
