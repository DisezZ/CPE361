use std::{io::{self, Write}, cmp::Ordering};
use nalgebra::{self, *};

#[macro_use] extern crate scan_fmt;

fn main() {
    let mut inputs = String::new();

    print!("Enter a starting point: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut inputs)
        .expect("Faile to read line");
    let (x, y) = scan_fmt!(&inputs, "{f} {f}", f64, f64).unwrap();
    let start = Point2::new(x as i32, y as i32);
    
    inputs.clear();

    print!("Enter a radius in each axis x y: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut inputs)
        .expect("Faile to read line");
    let (x, y) = scan_fmt!(&inputs, "{f} {f}", f64, f64).unwrap();
    let radius = Point2::new(x as i32, y as i32);

    midpoint_ellipse(start, radius);
}

fn midpoint_ellipse(center: Point2<i32>, radius: Point2<i32>) {
    let mut vec = Vec::<Point2<i32>>::new();

    let mut x: i32 = 0;
    let mut y = radius.y;
    let mut h = radius.y.pow(2) * 4 + radius.x.pow(2) * (1 - 4 * radius.y);
    let mut d1 = 12 * radius.y.pow(2);
    let mut d2 = -8 * radius.x.pow(2) * (radius.y - 1);

    let mut sn = radius.y.pow(2);
    let mut sd = radius.x.pow(2) * radius.y + radius.x.pow(2) / 2;

    midpoint_ellipse_mirror(&center, &Point2::new(0, radius.y)); vec.push(Point2::new(0, radius.y));
    while sn < sd  {
        println!("d1 = {}, d2 = {}, h = {}", d1, d2, h);
        if h > 0 {
            y -= 1;
            h += d2;
            d2 += 8 * radius.x.pow(2);
            sd -= radius.x.pow(2);
        }
        x += 1;
        h += d1;
        d1 += 8 * radius.y.pow(2);
        sn += radius.y.pow(2);
        println!("sn = {}, sd = {}, slope = {}", sn, sd, sn / sd);
        midpoint_ellipse_mirror(&center, &Point2::new(x, y)); vec.push(Point2::new(x, y));
    }
    
    h = radius.y.pow(2) * (4 * x.pow(2) + 4 * x + 1) + 4 * radius.x.pow(2) * (y - 1).pow(2) - 4 * radius.x.pow(2) * radius.y.pow(2);
    d1 = 8 * radius.y.pow(2) * (x + 1);
    d2 = -4 * radius.x.pow(2) * (2 * y - 3);

    while y > 1 {
        println!("d1 = {}, d2 = {}, h = {}", d1, d2, h);
        if h < 0 {
            x += 1;
            h += d1;
            d1 += 8 * radius.y.pow(2);
        }
        y -= 1;
        h += d2;
        d2 += 8 * radius.x.pow(2);
        midpoint_ellipse_mirror(&center, &Point2::new(x, y)); vec.push(Point2::new(x, y));
    }
    midpoint_ellipse_mirror(&center, &Point2::new(radius.x, 0)); vec.push(Point2::new(x, y));

    println!("\nHere is the points:");
    for point in vec.iter() {
        midpoint_ellipse_mirror(&center, &point);
    }
}

fn midpoint_ellipse_mirror(center: &Point2<i32>, point: &Point2<i32>) {
    print_point(Point2::new(center.x + point.x, center.y + point.y));
    print_point(Point2::new(center.x + point.x, center.y - point.y));
    print_point(Point2::new(center.x - point.x, center.y - point.y));
    print_point(Point2::new(center.x - point.x, center.y + point.y));
}

fn print_point(point: Point2<i32>) {
    println!("({}, {})", point.x, point.y);
}


/*
# How to pseudocode

## first step

midpoint_ellipse(x0, y0, rx, ry)
    while ‘‘slope magnitude’’ less than one:
        increment x;
        if f(x,y-.5)>0
            decrement y
            update f;
        else
            update f;
        EllipsePoints(x0, y0, x, y)

    while y > 0:
        decrement y;
        if f(x+.5,y) > 0
            update f;
        else
            increment x
            update f;
        EllipsePoints(x0, y0, x, y)

plot_ellipse(x0, y0, x, y)
    plot(x0 + x, y0 + y)
    plot(x0 + x, y0 - y)
    plot(x0 - x, y0 - y)
    plot(x0 - x, y0 + y)
 */