# Les traits

Un `trait` est un agrégat de méthodes définies pour un type inconnu: `Self`. Elles peuvent accéder aux autres méthodes déclarées dans le même trait.

Les traits peuvent être implémentés pour n'importe quel type de donnée. Dans l'exemple ci-dessous, nous définissons `Animal`, un groupe de méthodes. 
Le `trait` `Animal` est alors implémenté pour le type `Sheep`, permettant l'utilisation des méthodes de `Animal` avec une instance du type `Sheep`.

{{#playpen source/traitssource0.rs}}
