use crate::vec3::*;

pub fn write_color(v: Vec3) {
    println!("{} {} {}", 
        (255.999 * v.x) as u32, 
        (255.999 * v.y) as u32, 
        (255.999 * v.z) as u32);
}