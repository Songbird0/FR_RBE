# Hello World

Voici le code source d’un traditionnel « Hello World ».

{{#playpen source/helloworldsource0.rs}}

`println!` est une macro qui affiche du texte sur la console.

Un binaire peut être généré en utilisant le compilateur Rust : `rustc`.

```bash
$ rustc hello.rs
```

`rustc` va produire un binaire nommé « hello » qui pourra être exécuté :


```bash
$ ./hello
Hello World!
```

## Activité

Cliquez sur le bouton « Run » en début de section pour visualiser le résultat présenté. Ensuite, ajoutez une nouvelle ligne qui permettra de visualiser le résultat ci-dessous :

```text
Hello World!
I'm a Rustacean!
```
