struct Container(i32, i32);

// Nous déclarons un trait qui vérifie si deux items sont stockés 
// dans le conteneur.
// Le conteneur pourra également fournir la première ou dernière valeur.
trait Contains {
    // Nous définissons une bonne fois pour toute les types génériques 
    // que les méthodes/fonctions pourront utiliser.
    type A;
    type B;

    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // On précise le type de `A` et `B`. Si le type d'entrée est 
    // `Container(i32, i32)`, les types de sorties seront alors 
    // typés tous les deux `i32`.
    type A = i32; // typé i32
    type B = i32; // typé i32

    // `&Self::A` et `&Self::B` sont également valides ici.
    // Essayez de remplacer la référence number_1(`&i32`) par `&Self::A`, 
    // et la référence number_2(`&i32) par `&Self::B`!
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Récupère la première valeur.
    fn first(&self) -> i32 { self.0 }

    // Récupère la dernière valeur.
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
