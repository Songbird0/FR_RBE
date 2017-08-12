# Opérations sur le système de fichiers

Le module `std::io::fs` contient de nombreuses fonctions traitant avec le système de fichiers.

```rust,ignore
// Dans le fichier fs.rs
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;

// Une simple implémentation de la commande `cat path`.
fn cat(path: &Path) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Une simple implémentation de la commande `echo s > path`.
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = try!(File::create(path));

    f.write_all(s.as_bytes())
}

// Une simple implémentation de la commande `touch path` (ignore les fichiers existants).
fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("`mkdir a`");
    // Crée un répertoire et renvoie un objet `io::Result<()>`.
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    println!("`echo hello > a/b.txt`");
    // Le match précédent peut être simplifié en utilisant la méthode `unwrap_or_else()`.
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p a/c/d`");
    // Crée un répertoire récursivement, renvoie un objet `ìo::Result<()>`.
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    // Crée un lien symbolique, renvoie un objet `io::Result<()>`.
    if cfg!(target_family = "unix") {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    // Lit le contenu d'un répertoire, renvoie un objet `io::Result<Vec<Path>>`.
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }

    println!("`rm a/c/e.txt`");
    // Supprime un fichier, renvoie un objet `io::Result<()>`.
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir a/c/d`");
    // Supprime un répertoire vide, renvoie un objet `io::Result<()>`.
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
```

Voici le résultat attendu:

```bash
$ rustc fs.rs && ./fs
`mkdir a`
`echo hello > a/b.txt`
`mkdir -p a/c/d`
`touch a/c/e.txt`
`ln -s ../b.txt a/c/b.txt`
`cat a/c/b.txt`
> hello
`ls a`
> "a/b.txt"
> "a/c"
`rm a/c/e.txt`
`rmdir a/c/d`
```

Et l'état final du répertoire `a` est:

```bash
$ tree a
a
|-- b.txt
`-- c
    `-- b.txt -> ../b.txt

1 directory, 2 files
```

## Voir aussi

[La macro `cfg!`][cfg_macro].

[cfg_macro]: ../chapitre11/cfg.html
