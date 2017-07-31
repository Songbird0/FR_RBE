struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // Vous pouvez accéder à la ressource par le biais des références 
        // créées et par l'objet original.
        println!("Point has coordinates: ({}, {}, {})",
                 borrowed_point.x, another_borrow.y, point.z);

        // Erreur! Vous ne pouvez pas avoir des accès en écriture à une ressource 
        // qui est déjà empruntée par une référence immuable (i.e. accès en lecture).
        // let mutable_borrow = &mut point;
        // TODO ^ Essayez de décommenter cette ligne.

        // On sort du contexte des références 
        // immuables.
    }

    {
        let mutable_borrow = &mut point;

        // On modifie la ressource par le biais d'une 
        // référence mutable.
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // Erreur! Vous ne pouvez pas accéder à `point` en lecture 
        // alors que la ressource est potentiellement en train d'être modifiée.
        //let y = &point.y;
        // TODO ^ Essayez de décommenter cette ligne.

        // Erreur! On ne peut pas afficher `point.z` en utilisant 
        // la macro `println!` car elle prend en paramètre une référence 
        // immuable.
        // println!("Point Z coordinate is {}", point.z);
        // TODO ^ Essayez de décommenter cette ligne.

        // Ok! Les références mutables peuvent être passées comme 
        // références immuables à `println!`.
        println!("Point has coordinates: ({}, {}, {})",
                 mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

        // On sort du contexte des références 
        // mutables.
    }

    // Les accès en lecture sur `point` sont de nouveau 
    // permis par le borrow checker.
    let borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             borrowed_point.x, borrowed_point.y, borrowed_point.z);
}
