# Créer une bibliothèque

Commençons par créer une bibliothèque dont nous nous servirons ensuite pour l'importer dans une autre `crate`.

```rust,ignore
// Dans le fichier rary.rs
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

```bash
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```

Les bibliothèques sont préfixées par la séquence « lib » et possèdent, par défaut, *le nom du fichier utilisé pour créer la crate* (en l'occurrence `rary.rs`). Ce comportement peut, bien entendu, être modifié en utilisant l'attribut [`crate_name`][crate_name].

[crate_name]: ../chapitre11/metacrate.html 
