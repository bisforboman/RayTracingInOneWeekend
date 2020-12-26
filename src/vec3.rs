use std::ops;

// Mostly stolen från adamse

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn color(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn point(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn vec(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

fn clamp(i: f64, min: f64, max: f64) -> f64 {
    i.max(min).min(max)
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        dot(self, self)
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn color_fmt(&self, samples_per_pixel: u32) -> String {
        let scale = 1.0 / samples_per_pixel as f64;
        let color = |i: f64| -> u32 { (256.0 * clamp((i * scale).sqrt(), 0.0, 0.999)) as u32 };
        format!("{} {} {}", color(self.x), color(self.y), color(self.z))
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, o: Self) -> Self {
        Vec3 {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, o: Self) {
        self.x += o.x;
        self.y += o.y;
        self.z += o.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, o: Self) -> Self {
        Vec3 { 
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, o: Vec3) -> Self {
        Vec3 {        
            x: self.x * o.x,
            y: self.y * o.y,
            z: self.z * o.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, o: f64) -> Self {
        Vec3 {
            x: self.x * o,
            y: self.y * o,
            z: self.z * o,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, o: Vec3) -> Vec3 {
        o * self
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, o: f64) {
        self.x *= o;
        self.y *= o;
        self.z *= o;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, o: f64) -> Self {
        Vec3 {
            x: self.x / o,
            y: self.y / o,
            z: self.z / o,
        }
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, o: Vec3) -> Vec3 {
        o / self
    }
}