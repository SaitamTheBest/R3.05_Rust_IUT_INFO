fn average(x: f64, y: f64) -> f64 {
    let result = (x+y)/2.0;
    return result;
}

struct Rectangle{
    length: f64,
    width: f64,
}

fn perimetre(rectangle: Rectangle) -> f64{
    let result = (rectangle.length*2.0+rectangle.width*2.0);
    return result;
}

fn perimetre2(rectangle: &Rectangle) ->f64{
    let result = (rectangle.length*2.0+rectangle.width*2.0);
    return result;
}

fn main() {
    //Exercice 2.1
    let x = 13.0;
    let y = 12.0;
    println!("{}", average(x, y));
    println!("{}", average(x, y));

    //Exercice 2.2.1
    let my_rectangle = Rectangle{
        length : 2.0,
        width : 2.0,
    };
    println!("{}", perimetre2(&my_rectangle)); //Changer pour l'exercice 2.2.2

    //Exercice 2.2.2
    println!("{}", perimetre2(&my_rectangle));
}
