enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn area(shape: Shape) {
    match shape {
        Shape::Circle(radius) => {
            println!("Circle Area: {:.2}", std::f64::consts::PI * radius * radius);
        }
        Shape::Rectangle(width, height) => {
            println!("Rectangle Area: {:.2}", width * height);
        }
    }
}

fn main() {
    let c = Shape::Circle(3.0);
    let r = Shape::Rectangle(4.0, 45.0);

    area(c);
    area(r);
}
