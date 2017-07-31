# Verrouillage des ressources

Lorsqu'une référence immuable d'une ressource est récupérée, cette dernière est également « gelée », « verrouillée ». Lorsqu'une ressource est gelée, l'objet initial (celui à partir duquel une référence a été récupérée) ne peut être modifié jusqu'à ce que toutes les références soient détruites(i.e. ne figurent plus dans le contexte).

{{#playpen source/freezingsource0.rs}}
