# Les arguments du programme

Les arguments passés en ligne de commande peuvent être récupérés en utilisant `std::env::args` qui renvoie un itérateur fournissant une `String` pour chaque argument:

{{#playpen source/programargssource0.rs}}

```bash
$ ./args 1 2 3
My path is ./args.
I got 3 arguments: ["1", "2", "3"].
```
