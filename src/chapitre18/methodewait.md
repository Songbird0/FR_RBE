# La méthode `wait`

Vous souhaiteriez peut-être attendre qu'un processus, dont un objet `process::Child` est responsable, se termine. 
Pour cela vous devez appeler la méthode `Child::wait` qui renverra un objet `process::ExitStatus`.

```rust,ignore
// Dans le fichier wait.rs
use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
```

```bash
$ rustc wait.rs && ./wait
reached end of main
# `wait` s'est exécuté pendant 5 secondes.
# Une fois la commande `sleep 5` terminée notre programme `wait` a pris fin.
```
