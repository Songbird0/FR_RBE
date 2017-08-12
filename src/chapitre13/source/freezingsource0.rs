fn main() {
    let mut _mutable_integer = 7i32;

    {
        // On emprunte `_mutable_integer`.
        // Accès en lecture uniquement.
        let _large_integer = &_mutable_integer;

        // Erreur! `_mutable_integer` est gelé dans ce contexte
        // (i.e. la ressource est empruntée).
        // _mutable_integer = 50;
        // FIXME ^ Décommentez/commentez cette ligne.

        // On sort du contexte de `_large_integer`.
    }

    // Aucun problème! `_mutable_integer` n'est plus gelé dans ce contexte.
    _mutable_integer = 3;
}
