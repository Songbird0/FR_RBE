# Le problème

Un `trait` qui possède des types génériques en entrée doit respecter certaines règles : Les utilisateurs du trait doivent spécifier tous les types génériques de données supportés par le conteneur.

Dans l'exemple ci-dessous, le `trait` `Contains` permet l'utilisation des types génériques `A` et `B`. Le `trait` est alors implémenté pour le type (la structure) `Container` en spécifiant le type `i32` pour les deux types génériques (i.e. `A` et `B`). Ils peuvent donc être soumis à la fonction `fn difference()`.

Puisque le type `Contains` est générique, nous sommes obligés de déclarer tous les types génériques supportés par `Contains` pour la fonction `difference()`. En pratique, nous opterions pour une approche nous permettant d'inférer les types génériques supportés par `Contains` à partir de l'entrée `C`. C'est ce que nous verrons dans la section suivante, car les types associés autorisent cette approche.

{{#playpen source/problemesource0.rs}}

## Voir aussi

[Les structures](../chapitre3/struct.html).
