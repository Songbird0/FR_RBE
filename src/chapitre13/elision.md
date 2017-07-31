# Annotation implicite

En règle générale, décrire explicitement la durée de vie d'une référence n'est pas nécessaire et nous préférerons passer la main au compilateur, qui se chargera de nous épargner l'écriture des annotations et d'améliorer la lisibilité du code. Ce processus d'annotation implicite se nomme [l'élision](http://www.linternaute.com/dictionnaire/fr/definition/elision/). Il s'agit ici d'omettre volontairement les annotations pour que le compilateur le fasse à notre place. L'élision ne peut être appliquée que lorsqu'un pattern de durée de vie est commun, simple à deviner.

Le code source qui suit présente quelques exemples où nous avons volontairement omis les annotations. Pour une description plus exhaustive du concept d'élision, n'hésitez pas à consulter la section [lifetime elision](https://doc.rust-lang.org/book/lifetimes.html#lifetime-elision) du livre.

{{#playpen source/elisionsource0.rs}}

## Voir aussi

[Le chapitre du livre sur l'élision](https://doc.rust-lang.org/book/lifetimes.html#lifetime-elision).
