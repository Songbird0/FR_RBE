# Le pattern `ref`

Lorsque vous vous servez du pattern matching ou de la déstructuration dans une assignation (`let`), le mot-clé `ref` peut être utilisé pour récupérer une référence d'un (ou plusieurs) champ d'une structure et/ou d'un tuple. Le code ci-dessous propose des exemples où ce modèle peut être utile :

{{#playpen source/refpatternsource0.rs}}
