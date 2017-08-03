// #![crate_name = "doc"]

/// Un être humain est représenté ici.
pub struct Person {
    /// Une personne doit avoir un nom.
    name: String,
}

impl Person {
    /// Renvoie une personne avec le nom qu'on lui a donné.
    ///
    /// # Arguments
    ///
    /// * `name` - Une slice qui contient le nom de la personne.
    ///
    /// # Example
    ///
    /// ```
    /// // Vous pouvez écrire du code rust entre les balises
    /// // dans les commentaires.
    /// // Si vous passez --test à Rustdoc, il testera même la source pour vous !
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person { name: name.to_string() }
    }

    /// Affiche un salut amical !
    ///
    /// Affiche "Hello, [name]" à l'objet `Person` en question.
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}
