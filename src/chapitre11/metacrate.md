# Méta-données relatives aux crates

L'attribut `crate_type` peut être utilisé pour renseigner au compilateur le type de la `crate` (i.e. exécutable ou bibliothèque(et quel type de bibliothèque)) et l'attribut `crate_name` est utilisé pour renseigner le nom de la `crate`.

```rust,ignore
// lib.rs
// Cette crate est une bibliothèque.
#![crate_type = "lib"]
// Cette bibliothèque est nommée "rary".
#![crate_name = "rary"]

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

Lorsque l'attribut `crate_type` est utilisé vous n'avez, bien entendu, plus besoin de passer le flag `--crate-type` à `rustc`.


```bash
$ rustc lib.rs
$ ls lib*
library.rlib
```
