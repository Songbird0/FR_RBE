# Capture

Les closures sont naturellement « souples » et feront leur possible pour fonctionner sans typage explicite. Ceci permet à la capture de s'adapter au contexte : parfois en prenant possession des ressources, parfois seulement en les empruntant. Les closures peuvent capturer les variables :

* Par référence : `&T` ;
* Par référence mutable : `&mut T` ;
* Par valeur `T`.

Par défaut, elles privilégient la capture par référence s'il n'est pas nécessaire de prendre possession des ressources.

{{#playpen source/capturesource0.rs}}

## Voir aussi

[Box](../chapitre17/boxpiletas.html) et [std::mem::drop](http://doc.rust-lang.org/std/mem/fn.drop.html).