# Définition d'un type d'erreur

Rust nous permet de définir nos propres types d'erreur. En général, un "bon" type d'erreur:

* Représente différentes erreurs avec le même type;
* Présente un message d'erreur intelligible pour l'utilisateur;
* Est facilement comparable aux autres types;
    * Bien: `Err(EmptyVec)`,
    * Pas bien: `Err("Please use a vector with at least one element".to_owned())`.
* Peut supporter l'ajout d'informations à propos de l'erreur;
    * Bien: `Err(BadChar(c, position))`,
    * Pas bien: `Err("+ cannot be used here".to_owned())`.

Notez qu'une `String` (que nous utilisons jusqu'ici) remplit les deux premiers critères, mais pas les deux derniers. 
Cela rend la création d'erreurs, simplement en utilisant `String`, verbeuse et difficile à maintenir. Il ne devrait pas être nécessaire de polluer la logique du code avec le formattage des chaînes de caractères seulement pour avoir un affichage intelligible.

{{#playpen source/definitiontypesource0.rs}}

## Voir aussi

[`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) et 
[`io::Result`](https://doc.rust-lang.org/std/io/type.Result.html).
