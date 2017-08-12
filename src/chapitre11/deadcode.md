# L'avertissement `dead_code`

La compilateur fournit la [lint][analyse] `dead_code` qui vous avertira lorsqu'une instruction (ou une fonction) ne sera jamais exécutée. Un attribut peut être utilisé pour désactiver cette lint.

{{#playpen source/deadcodesource0.rs}}

Gardez tout de même à l'esprit que, dans un programme destiné à être mis en production, il est préférable de supprimer le code mort. Ici, le code mort sera conservé simplement pour l'exemple.

[analyse]: https://en.wikipedia.org/wiki/Lint_%28software%29
