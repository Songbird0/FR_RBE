# La hiérarchie des fichiers

Il est possible de transformer nos modules en un ensemble de fichiers et de répertoires. Recréons l'exemple utilisé pour illustrer le concept de [la visibilité](../chapitre9/visibilite.html) en un ensemble de fichiers :

```bash
$ tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- split.rs
```

```rust,ignore
// my/mod.rs
// De la même manière `mod inaccessible` et `mod nested` vont essayer de localiser 
// les fichiers `nested.rs` et `inaccessible.rs` pour récupérer et ajouter leur contenu 
// dans leur module respectif.
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}
```

```rust,ignore
// my/nested.rs
pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
```

```rust,ignore
// my/inaccessible.rs
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}
```

```bash
$ rustc split.rs && ./split
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```
