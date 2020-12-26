mod vec3;
use vec3::*;
mod ray;
use ray::*;
mod color;
use color::write_color;

#[derive(Debug)]
struct Image {
    width: f64,
    height: f64,
}

fn ray_color(r: &Ray) {
    let direction = r.dir.unit();
    let t = 0.5 * (direction.y + 1.0);
    (1.0 - t) * Vec3::one() + t * color(0.5, 0.7, 1.0);
}

fn main() {
    let image = Image {
        width: 256.0,
        height: 256.0,
    };
    
    println!("P3");

    println!("{} {}", image.width, image.height);

    println!("255");

    for j in (0..image.height as u32).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image.width as u32 {
            
            write_color(
                color(
                    i as f64 / (image.width - 1.0),
                    j as f64 / (image.height - 1.0),
                    0.25
                )
            );
        }
    }

    eprintln!("\nDone!");
}

