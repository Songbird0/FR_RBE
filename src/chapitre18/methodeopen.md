# La méthode `open`

La méthode statique `open()` peut être utilisé pour ouvrir un fichier en lecture seule.

Un objet `File` est responsable d'une ressource, du descripteur de fichier et prend soin de fermer le fichier lorsqu'il est libéré.


```rust,ignore
// Dans le fichier open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Crée un chemin vers le fichier désiré.
    let path = Path::new("hello.txt");
    let display = path.display();

    // Ouvre le chemin en lecture seule, renvoie un objet `io::Result<File>`.
    let mut file = match File::open(&path) {
        // La méthode `description` de `io::Error` renvoie une chaîne de caractères
        // qui décrit l'erreur.
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Lit le contenu du fichier dans une chaîne de caractères, renvoie un objet `io::Result<usize>`.
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` sort du contexte, le flux ouvert sur le fichier "hello.txt" 
    // va être fermé.
}
```

Voici le résultat attendu:

```text
$ echo "Hello World!" > hello.txt
$ rustc open.rs && ./open
hello.txt contains:
Hello World!
```

Nous vous encourageons à confronter l'exemple précédent à des cas d'échec différents (e.g. `hello.txt` n'existe pas, `hello.txt` ne peut pas être lu).
