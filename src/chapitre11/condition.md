# Condition personnalisée

Certaines conditions (e.g. `target_os`) sont fournies par `rustc`. Il est toutefois possible de passer des conditions personnalisées à `rustc` en utilisant le flag `--cfg`.

```rust,ignore
// Dans le fichier custom.rs
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}
```

Sans le flag personnalisé :

```bash
$ rustc custom.rs && ./custom
No such file or directory (os error 2)
```

Avec le flag personnalisé :

```bash
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```
