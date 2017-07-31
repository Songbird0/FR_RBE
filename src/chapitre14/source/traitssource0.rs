struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Méthode statique; `Self` fait référence au type ayant implémenté 
    // le trait.
    fn new(name: &'static str) -> Self;

    // Méthode d'instance; Elles renverront une chaîne de caractères.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Les traits peuvent fournir une implémentation par défaut.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Les méthodes de `Self` peuvent utiliser les méthodes déclarées 
            // par le trait.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implémentation des services du trait `Animal` 
// pour le type `Sheep`.
impl Animal for Sheep {
    // En l'occurrence, `Self` fait référence à `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // L'implémentation par défaut fournie par le trait 
    // peut être réécrite.
    fn talk(&self) {
        // Par exemple, nous pourrions fournir une description plus précise.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // Typer l'identificateur est nécessaire dans ce cas de figure.
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Essayez de supprimer le type annoté.

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
