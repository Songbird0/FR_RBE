// Une structure unitaire. 
struct Nil;

// Un tuple.
struct Pair(i32, f32);

// Une structure avec deux champs.
struct Point {
    x: f32,
    y: f32,
}

// Les structures peuvent faire partie des champs d'une autre structure.
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // On instancie la structure `Point`.
    let point: Point = Point { x: 0.3, y: 0.4 };

    // On accède aux champs du point.
    println!("point coordinates: ({}, {})", point.x, point.y);

    // On décompose les champs de la structure pour les assigner 
    // à de nouvelles variables (i.e. my_x et my_y)
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // L'instanciation de la structure est également une expression.
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // On instancie la structure unitaire, vide.
    let _nil = Nil;

    // On instancie un tuple.
    let pair = Pair(1, 0.1);

    // Accède aux champs du tuple.
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // On décompose un tuple.
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}