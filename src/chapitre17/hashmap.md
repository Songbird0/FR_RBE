# La structure `HashMap`

Là où les vecteurs stockent leurs valeurs en utilisant un index entier, les `HashMap`s stockent leurs valeurs en utilisant des clés. Les clés d'une `HashMap` peuvent être des booléens, des chaînes de caractères ou n'importe quel autre type qui implémente les traits `Eq` et `Hash`. Nous y reviendrons dans la section suivante.

Tout comme les vecteurs, les `HashMap` sont redimensionnables mais peuvent également se tronquer elles-mêmes lorsqu'elles atteignent la limite de leur capacité. 
Vous pouvez créer une `HashMap` avec une capacité donnée en utilisant `HashMap::with_capacity(uint)`, ou utiliser `HashMap::new()` pour récupérer une instance avec une capacité initiale par défaut (recommandé).

{{#playpen source/hashmapsource0.rs}}

Pour plus d"informations à propos du fonctionnement du hashage et des hash maps (parfois appelées hash tables), consultez [la page wikipédia dédiée aux Hash Tables][hashtables].

[hashtables]: https://en.wikipedia.org/wiki/Hash_table
