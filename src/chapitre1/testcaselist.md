# Exemple d'utilisation : La structure List

Implémenter le `trait` `fmt::Display` pour une structure où les éléments doivent être gérés séquentiellement est assez délicat. Le problème réside dans le fait que chaque appel de la macro `write!` génère une instance de `fmt::Result`. Une bonne gestion de ces appels nécessite de tester chaque résultat. Rust vous permet de gérer les erreurs de deux manières :

1. En utilisant la macro `try!`;
2. En utilisant l'opérateur `?` (qui est l'équivalent de `try!` mais intégré directement au langage).

La macro `try!` vient envelopper la fonction (ou la macro) cible comme ceci :

```rust,ignore
// On ‘test' write! Pour voir si une erreur survient. S'il y a une erreur, 
// elle sera renvoyée. Sinon, l'exécution continue. 
try!(write!(f, "{}", value));
```

L'opérateur `?`, bien qu'équivalent à la macro `try!`, vient se positionner devant l'appel de la fonction (ou macro).


```rust,ignore
write!(f, "{}", value)?;
```

Avec l'opérateur `?`, l'implémentation du trait `fmt::Display` pour un `Vec` est simple et lisible.

{{#playpen source/testcaselistsource0.rs}}

## Activité

Essayez de modifier le programme pour que l'index de chaque élément du vector soit également affiché durant l'exécution. Le résultat devrait ressembler à ceci :

```text
[0: 1, 1: 2, 2: 3]
```

## Voir aussi

[La boucle for][for], [le pattern `ref`][ref], [`Result`][result], [les structures][struct], [`try!`][try], [`vec!`][vec].

[for]: ../chapitre7/forintervalles.html
[ref]: ../chapitre13/refpattern.html
[result]: ../chapitre17/enumresult.html
[struct]: ../chapitre3/struct.html
[try]: ../chapitre17/trymacro.html
[vec]: ../chapitre17/vecteurs.html
