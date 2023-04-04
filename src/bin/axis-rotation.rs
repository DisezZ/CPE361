use nalgebra::{self, *};

fn main() {
    let degree: f64 = 60.0;
    let rad: f64 = degree.to_radians();
    let start = Point3::new(2.0, 2.0, 2.0);
    let end = Point3::new(7.0, 8.0, 9.0);
    let first_vertex = Point3::new(5.0, 10.0, 15.0);
    let vector = end - start;
    let mut hexagon: [Point3<f64>; 6] = [Point3::origin(); 6];
    hexagon[0] = first_vertex;

    let t = Translation3::from(-start);
    let ti = Translation3::from(-t.vector);

    let u = vector.normalize();
    let d = u.remove_row(0).norm();

    let mut rx = Matrix4::<f64>::identity();
    rx[(1, 1)] = u.z / d; rx[(1, 2)] = -u.y / d;
    rx[(2, 1)] = u.y / d; rx[(2, 2)] = u.z / d;
    let rxi = rx.try_inverse().unwrap_or(Matrix4::identity());
    
    let mut ry = Matrix4::<f64>::identity();
    ry[(0, 0)] = d; ry[(0, 2)] = -u.x;
    ry[(2, 0)] = u.x; ry[(2, 2)] = d;
    let ryi = ry.try_inverse().unwrap_or(Matrix4::identity());

    let mut rz = Matrix4::<f64>::identity();
    rz[(0, 0)] = rad.cos(); rz[(0, 1)] = -rad.sin();
    rz[(1, 0)] = rad.sin(); rz[(1, 1)] = rad.cos();

    let r= ti.to_homogeneous() * rxi * ryi * rz * ry * rx * t.to_homogeneous();

    let mut prev = first_vertex;
    for vertex in hexagon.iter_mut().skip(1) {
        let curr = if let Some(val) = Point3::from_homogeneous(r * prev.to_homogeneous()) {
            val
        } else {
            Point3::origin()
        };
        prev = curr;
        *vertex = curr;
    }

    println!("d: {}", d);
    println!("u: {}", u);
    println!("T: {}", t.to_homogeneous());
    println!("Rx: {}", rx);
    println!("Ry: {}", ry);
    println!("Rz: {}", rz);
    println!("Ryi: {}", ryi);
    println!("Rxi: {}", rxi);
    println!("Ti: {}", ti.to_homogeneous());
    println!("R: {}", r);
    for (idx, vertex) in hexagon.iter().enumerate() {
        println!("Point {}: {}", idx + 1, vertex);
    }
}
