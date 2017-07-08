# La déclaration `extern crate`

Pour importer une `crate` à cette nouvelle bibliothèque, il vous faudra utiliser la déclaration `extern crate`. Cette déclaration a aussi pour effet d'importer toutes les ressources sous un même module, possédant le même nom que la bibliothèque. Les règles régissant la visibilité des ressources s'appliquent également aux modules des bibliothèques importées.

```rust,ignore
// executable.rs
// On attache la bibliothèque `library` et on importe ses ressources 
// sous un module nommé `rary`.
extern crate rary;

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}
```

```bash
# Où `library.rlib` est le chemin de la bibliothèque compilée, nous assumerons ici 
# que la bibliothèque se trouve dans le répertoire courant.
$ rustc executable.rs --extern rary=library.rlib && ./executable
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```
