fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // Cette expression sera assignée à `y`.
        x_cube + x_squared + x
    };

    let z = {
        // Le point-virgule supprime cette expression et `()` est assigné 
        // à `z`.
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
