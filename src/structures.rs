#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(r: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = r;
    let horizontal_length = (x1 - x2).abs();
    let vertical_length = (y1 - y2).abs();
    horizontal_length * vertical_length
}

fn square(lower_left: &Point, width: f32) -> Rectangle {
    let Point { x, y } = lower_left;
    Rectangle {
        top_left: Point {
            y: y + width,
            ..*lower_left
        },
        bottom_right: Point {
            x: x + width,
            ..*lower_left
        },
    }
}

pub fn structures() {
    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let area = rect_area(&_rectangle);
    println!("Area of {:?} is {}", _rectangle, area);

    let point = Point { x: 0.0, y: 0.0 };
    let _square = square(&point, 5.0);

    println!("A square from {:?} with width 5 is {:?}", point, _square);
}
