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
    let mut start = Point2::new(x, y);
    
    inputs.clear();

    print!("Enter a ending point: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut inputs)
        .expect("Faile to read line");
    let (x, y) = scan_fmt!(&inputs, "{f} {f}", f64, f64).unwrap();
    let mut end = Point2::new(x, y);

    midpoint_line(&mut start, &mut end);
}

fn midpoint_line(start: &mut Point2<f64>, end: &mut Point2<f64>) {
    if (end.y - start.y).abs() < (end.x - start.x).abs() {
        println!("Slope: 45 > s > -45");
        if start.x > end.x {
            std::mem::swap(start, end);
            println!("Swap: x y");
        }
        let start = Point2::new(start.x as i32, start.y as i32);
        let end = Point2::new(end.x as i32, end.y as i32);
        midpoint_line_low(start, end);
    } else {
        println!("Slope: 45 -> 90 or -45 -> -90");
        if start.y > end.y {
            std::mem::swap(start, end);
            println!("Swap: x y");
        }
        let start = Point2::new(start.x as i32, start.y as i32);
        let end = Point2::new(end.x as i32, end.y as i32);
        midpoint_line_high(start, end);
    }
}

fn midpoint_line_low(start: Point2<i32>, end: Point2<i32>) {
    let dx = end.x - start.x;
    let mut dy = end.y - start.y;
    let yi = match dy.cmp(&0) {
        Ordering::Less => {
            dy = -dy;
            -1
        },
        _ => 1,
    };
    let mut d = 2 * dy - dx;
    let dD = 2 * dy;
    let dU = 2 * (dy - dx);
    let mut y = start.y;

    println!("dx: {}", dx);
    println!("dy: {}", dy);
    println!("yi: {}", yi);
    println!("d: {}", d);
    println!("dD: {}", dD);
    println!("dU: {}", dU);
    println!("Here is the points:");
    for x in start.x..=end.x {
        print_point(Point2::new(x, y));
        if d < 0 {
            d += dD;
        } else {
            d += dU;
            y += yi;
        }
    }
}

fn midpoint_line_high(start: Point2<i32>, end: Point2<i32>) {
    let mut dx = end.x - start.x;
    let dy = end.y - start.y;
    let xi = match dx.cmp(&0) {
        Ordering::Less => {
            dx = -dx;
            -1
        },
        _ => 1,
    };
    let mut d = 2 * dx - dy;
    let dD = 2 * dx;
    let dU = 2 * (dx - dy);
    let mut x = start.x;

    println!("dx: {}", dx);
    println!("dy: {}", dy);
    println!("xi: {}", xi);
    println!("d: {}", d);
    println!("dD: {}", dD);
    println!("dU: {}", dU);
    println!("Here is the points:");
    for y in start.y..=end.y {
        print_point(Point2::new(x, y));
        if d < 0 {
            d += dD;
        } else {
            d += dU;
            x += xi;
        }
    }
}

fn print_point(point: Point2<i32>) {
    println!("({}, {})", point.x, point.y);
}


/*
# How to pseudocode

## first step

plotLine(x0, y0, x1, y1)
    if abs(y1 - y0) < abs(x1 - x0)
        if x0 > x1
            plotLineLow(x1, y1, x0, y0)
        else
            plotLineLow(x0, y0, x1, y1)
        end if
    else
        if y0 > y1
            plotLineHigh(x1, y1, x0, y0)
        else
            plotLineHigh(x0, y0, x1, y1)
        end if
    end if


## then these two

plotLineLow(x0, y0, x1, y1)
    dx = x1 - x0
    dy = y1 - y0
    yi = 1
    if dy < 0
        yi = -1
        dy = -dy
    end if
    D = (2 * dy) - dx
    y = y0

    for x from x0 to x1
        plot(x, y)
        if D > 0
            y = y + yi
            D = D + (2 * (dy - dx))
        else
            D = D + 2*dy
        end if


plotLineHigh(x0, y0, x1, y1)
    dx = x1 - x0
    dy = y1 - y0
    xi = 1
    if dx < 0
        xi = -1
        dx = -dx
    end if
    D = (2 * dx) - dy
    x = x0

    for y from y0 to y1
        plot(x, y)
        if D > 0
            x = x + xi
            D = D + (2 * (dx - dy))
        else
            D = D + 2*dx
        end if
 */