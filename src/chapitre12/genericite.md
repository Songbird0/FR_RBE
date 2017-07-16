# La généricité

Comme son nom l'indique, cette section abordera les types et fonctionnalités génériques. La généricité peut être très utile pour réduire les répétitions au sein du code dans de nombreux cas, mais vous demandera, en échange, d'apporter quelques précisions supplémentaires à propos de la syntaxe. Notez également que rendre une ressource générique signifie que n'importe quelle ressource sera traitée de la même manière, il est nécessaire de savoir quels types de ressources peuvent être réellement traités(dans les cas où il est nécessaire de le spécifier).

La généricité est principalement utilisée pour rendre générique un (ou des) paramètre passé à une fonction. Par convention, un paramètre générique doit avoir un identificateur respectant la convention de nommage CamelCase et être déclaré entre un chevron ouvrant (`<`) et un chevron fermant(`>`) : `<Aaa, Bbb, ...>`, qui est souvent représenté par le paramètre `<T>.` En déclarant un paramètre générique de type `<T>`, on accepte de recevoir un, ou plusieurs, paramètre de ce type. Tout paramètre déclaré comme générique est générique, tout le reste est concret (non-générique).

Par exemple, voici une fonction générique nommée `foo` qui prend un paramètre de type `T` (de n'importe quel type, donc) :

```rust,ignore
fn foo<T>(p: T) { ... }
```

{{#playpen source/genericitesource0.rs}}

## Voir aussi

[Les structures](../chapitre3/struct.html)
