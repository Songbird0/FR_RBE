# La structure `HashSet`

Voyez une `HashSet` comme une `HashMap` où nous nous soucions uniquement des clés (`HashSet<T>` est, en réalité, simplement un wrapper de `HashMap<T, ()>`).

Vous pourriez vous demander "Quel est le but ? Je pourrais simplement stocker mes clés dans un `Vec`".

La fonctionnalité unique de `HashSet` est qu'elle garantit l'inexistance d'éléments dupliqués. C'est le contrat que n'importe quel ensemble remplit. `HashSet` n'est qu'une implémentation (voir aussi: [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html "Documentation officielle: La structure BTreeSet")).

Si vous ajoutez une valeur déjà présente dans l'instance `HashSet`, (i.e. la nouvelle valeur est égale à l'existante et ont toutes deux le même hash), alors la nouvelle valeur remplacera l'ancienne.

C'est pratique lorsque vous ne souhaitez jamais plus d'une occurrence de quelque chose ou lorsque vous voulez savoir si vous possédez déjà quelque hose. Mais les ensembles peuvent faire bien plus que cela.

Les ensembles ont quatre (4) opérations inhérentes (chacune renvoie un itérateur):

1. `union`:  Récupère tous les éléments dans les deux ensembles;
2. `difference`: Récupère tous les éléments qui sont dans le premier ensemble mais pas dans le second;
3. `intersection`: Récupère uniquement les éléments présents dans les *deux* ensembles;
4. `symmetric_difference`: Récupère tous éléments qui sont dans le premier ou second ensemble mais *pas* les deux.

Essayons tout cela dans l'exemple suivant.

{{#playpen source/hashsetsource0.rs}}

Les exemples originaux proviennent de la [documentation](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.difference "Documentation officielle: la méthode difference()").
