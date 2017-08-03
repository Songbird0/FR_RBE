# Tests

Les fonctions peuvent être testées en utilisant ces [attributs][attr]:

* `#[test]` désigne une fonction comme test unitaire. La fonction ne doit prendre aucun paramètre et ne rien renvoyer;
* `#[should_panic]` désigne une fonction comme un test voué à l'échec.

```rust,ignore
// Dans le fichier unit_test.rs
// Compile `main` à condition que la compilation des tests ne soit pas activée.
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

// Compile le module `test` seulement si la compilation des tests est activée.
#[cfg(test)]
mod test {
// Un test unitaire `distance_test` est nécessaire.
    fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
        (
            (b.0 - a.0).powi(2) +
            (b.1 - a.1).powi(2)
        ).sqrt()
    }

    #[test]
    fn distance_test() {
        assert!(distance((0f32, 0f32), (1f32, 1f32)) == (2f32).sqrt());
    }

    #[test]
    #[should_panic]
    fn failing_test() {
        assert!(1i32 == 2i32);
    }
}
```

Les tests peuvent être exécutés avec la commande `cargo test` ou `rustc --test`.
```bash
$ rustc --test unit_test.rs
$ ./unit_test 

running 2 tests
test test::distance_test ... ok
test test::failing_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Si `--test` n'a pas été inclut alors il devrait se passer ceci:
```bash
$ rustc unit_test.rs
$ ./unit_test
If you see this, the tests were not compiled nor ran!
```

## Voir aussi

[Les attributs][attr], [la compilation conditionnelle][cond_compile] et [mod][module].

[attr]: ../chapitre11/attributes.html
[cond_compile]: ../chapitre11/cfg.html
[module]: ../chapitre9/module.html
