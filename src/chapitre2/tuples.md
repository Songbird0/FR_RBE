# Les tuples

Un tuple est une collection de valeurs de différents types(ou pas). Les tuples peuvent être construits en utilisant les parenthèses `()` et chaque tuple est lui-même un type possédant sa propre signature `(T1, T2, …)`, où `T1, T2` sont les types de ses membres. Les fonctions peuvent se servir des tuples pour renvoyer plusieurs valeurs, puisque ces derniers peuvent être extensibles à volonté.

{{#playpen source/tuplessource0.rs}}

## Activité

1. Récapitulatif : Implémentez les services du trait `fmt::Display` pour la structure `Matrix` dans l'exemple ci-dessus. Donc si vous passez de l'affichage de débogage `{:?}` à l'affichage plus « user friendly » `{}`, vous devriez voir le résultat suivant :

```text
( 1.1 1.2 )
( 2.1 2.2 )
```

Vous pouvez vous référer à l'exemple précédemment donné pour [l'implémentation du trait Display](../chapitre1/display.html).

2. Ajoutez une fonction `transpose`, en vous appuyant sur l'implémentation de la fonction `reverse`, qui accepte une matrice passée en paramètre et renvoie une matrice dans laquelle deux éléments ont été inversés. Exemple :

```bash
println!("Matrix:\n{}", matrix);
println!("Transpose:\n{}", transpose(matrix));
```

Affiche:

```text
Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
```
