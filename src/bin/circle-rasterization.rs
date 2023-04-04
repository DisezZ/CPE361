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
    let mut center = Point2::new(x as i32, y as i32);

    inputs.clear();

    print!("Enter a starting point: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut inputs)
        .expect("Faile to read line");
    let r = scan_fmt!(&inputs, "{f}", f64).unwrap() as i32; 

    midpoint_circle(&mut center, r);
}

fn midpoint_circle(center: &mut Point2<i32>, r: i32) {
    let mut vec = Vec::<Point2<i32>>::new();

    let mut h = 1 - r;
    let mut x = 0;
    let mut y = r;
    
    println!("h: {h}");
    println!("Here is the points:");
    midpoint_circle_mirror(center, Point2::new(x, y)); vec.push(Point2::new(x, y));
    while y > x {
        if h < 0 {
            let dU = 2 * x + 3;
            h += dU;
            x += 1;
            print!("dU = {dU}, ");
        } else {
            let dD = 2 * (x - y) + 5;
            h += dD;
            x += 1;
            y -= 1;
            print!("dD = {dD}, ");
        }
        print!("h = {h}\n");
        io::stdout().flush().unwrap();
        midpoint_circle_mirror(center, Point2::new(x, y)); vec.push(Point2::new(x, y));
    }

    println!("\nHere is the points:");
    for point in vec.iter() {
        midpoint_circle_mirror(center, *point);
    }
}

fn midpoint_circle_mirror(center: &mut Point2<i32>, point: Point2<i32>) {
    print_point(Point2::new(center.x + point.x, center.y + point.y));
    print_point(Point2::new(center.x + point.y, center.y + point.x));
    print_point(Point2::new(center.x - point.y, center.y + point.x));
    print_point(Point2::new(center.x - point.x, center.y + point.y));
    print_point(Point2::new(center.x - point.x, center.y - point.y));
    print_point(Point2::new(center.x - point.y, center.y - point.x));
    print_point(Point2::new(center.x + point.y, center.y - point.x));
    print_point(Point2::new(center.x + point.x, center.y - point.y));
}

fn print_point(point: Point2<i32>) {
    println!("({}, {})", point.x, point.y);
}


/*
# How to pseudocode

## first step

midpoint(x0, y0, r)
    h = 1 - r
    x = 0
    y = r
    plot_circle(x0, y0, x, y)
    while y > x
        if h < 0
            dU = 2 * x + 3
            h = h + dU
            x = x + 1
        else
            dD = 2 * (x - y) + 5
            h = h + dD
            x = x + 1
            y = y - 1
        end if
        plot_circle(x0, y0, x, y)

## then

plot_circle(x0, y0, x, y)
    plot(x0 + x, y0 + y)
    plot(x0 + y, y0 + x)
    plot(x0 - y, y0 + x)
    plot(x0 - x, y0 + y)
    plot(x0 - x, y0 - y)
    plot(x0 - y, y0 - x)
    plot(x0 + y, y0 - x)
    plot(x0 + x, y0 - y)
 */