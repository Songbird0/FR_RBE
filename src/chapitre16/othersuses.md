# D'autres cas d'utilisation de `try!`

Vous avez remarquez, dans l'exemple précédent, que notre réaction immédiate à l'appel de `parse` est de passer l'erreur de la bibliothèque dans notre nouveau type:

```rust,ignore
.and_then(|s| s.parse::<i32>()
    .map_err(DoubleError::Parse)
```

Puisque c'est une opération plutôt commune, il ne serait pas inutile qu'elle soit élidée. Hélas, `and_then` n'étant pas suffisamment flexible pour cela, ce n'est pas possible. Nous pouvons, à la place, utiliser `try!`.

La macro `try!` a été précédemment présentée comme permettrant la récupération de la ressource (`unwrap`) ou le renvoi prématuré, si une erreur survient (`return Err(err)`). C'est plus ou moins vrai. En réalité, elle utilise soit `unwrap` soit `return Err(From::from(err))`.  Puisque `From::from` est un utilitaire permettant la convertion entre différents types, cela signifie que si vous utilisez `try!` where l'erreur peut être convertie au type de retour, elle le sera automatiquement.

Ici, nous réécrivons l'exemple précédent en utilisant `try!`. Résultat, la méthode `map_err` disparaîtra lorsque `From::from` sera implémenté pour notre type d'erreur:

{{#playpen source/othersusessource0.rs}}

C'est effectivement plus acceptable. Comparé à l'original `panic`, en remplaçant les appels de `unwrap` par `try!` nous conservons une utilisation assez familière, à l'exception que `try!` renvoie les types dans un conteneur `Result`, rendant leur déstructuration plus abstraite.

Notez toutefois que vous ne devriez pas systématiquement gérer les erreurs de cette manière pour remplacer les appels de `unwrap`. Cette méthode de gestion d'erreur a triplé le nombre de lignes de code et ne peut pas être considéré comme "simple" (même si la taille du code n'est pas énorme).

En effet, modifier une bibliothèque de 1000 lignes pour remplacer des appels de `unwrap` et établir une gestion des erreurs plus "propre" pourrait être faisable en une centaine de lignes supplémentaires. En revanche, le recyclage nécessaire en aval ne serait pas évident.

Nombreuses sont les bibliothèques qui pourraient s'en sortir en implémentant seulement `Display` et ajouter `From` comme base pour la gestion. Cependant, pour des bibliothèques plus importantes, l'implémentation de la gestion des erreurs peut potentiellement répondre à des besoins plus spécifiques.

## Voir aussi

[`From::from`](https://doc.rust-lang.org/std/convert/trait.From.html) 
et [`try!`](https://doc.rust-lang.org/std/macro.try!.html).
