/*
 *   Copyright (c) 2020 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

#[derive(Copy, Clone)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
 
impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
        Vec4f {
            x, y, z, w
        }
    }

    pub fn mag_sq(&self) -> f32 {
        self.x.powi(2)+self.y.powi(2)+self.z.powi(2)+self.w.powi(2)
    }

    pub fn mag(&self) -> f32 {
        self.mag_sq().sqrt()
    }

    pub fn set_mag(&mut self, mag: f32) {
        let old_mag = self.mag();
        self.x = self.x / old_mag * mag;
        self.y = self.y / old_mag * mag;
        self.z = self.z / old_mag * mag;
        self.w = self.w / old_mag * mag;
    }
 
    pub fn sum(&self) -> f32 {
        self.x + self.y + self.z + self.w
    } 
}

impl std::ops::Add for Vec4f {
    type Output = Vec4f;

    fn add(self, rhs: Self) -> Self::Output {
        Vec4f::new(
            self.x + rhs.x, 
            self.y + rhs.y,
            self.z + rhs.z, 
            self.w + rhs.w
        )
    }
}

impl std::ops::Sub for Vec4f {
    type Output = Vec4f;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec4f::new(
            self.x - rhs.x, 
            self.y - rhs.y,
            self.z - rhs.z, 
            self.w - rhs.w
        )
    }
}

impl std::ops::Mul for Vec4f {
    type Output = Vec4f;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec4f::new(
            self.x * rhs.x, 
            self.y * rhs.y,
            self.z * rhs.z, 
            self.w * rhs.w
        )
    }
}

impl std::ops::Div for Vec4f {
    type Output = Vec4f;

    fn div(self, rhs: Self) -> Self::Output {
        Vec4f::new(
            self.x / rhs.x, 
            self.y / rhs.y,
            self.z / rhs.z, 
            self.w / rhs.w
        )
    }
}

impl std::ops::AddAssign for Vec4f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z; 
        self.w += rhs.w;
    }
}

impl std::ops::SubAssign for Vec4f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z; 
        self.w -= rhs.w;
    }
}

impl std::ops::MulAssign for Vec4f {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z; 
        self.w *= rhs.w;
    }
}

impl std::ops::DivAssign for Vec4f {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z; 
        self.w /= rhs.w;
    }
}


 #[derive(Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f {
            x, y, z
        }
    }

    pub fn mag_sq(&self) -> f32 {
        self.x.powi(2)+self.y.powi(2)+self.z.powi(2)
    }

    pub fn mag(&self) -> f32 {
        self.mag_sq().sqrt()
    }

    pub fn set_mag(&mut self, mag: f32) {
        let old_mag = self.mag();
        self.x = self.x / old_mag * mag;
        self.y = self.y / old_mag * mag;
        self.z = self.z / old_mag * mag;
    }

    pub fn sum(&self) -> f32 {
        self.x + self.y + self.z
    } 
}

impl std::ops::Add for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3f::new(
            self.x + rhs.x, 
            self.y + rhs.y,
            self.z + rhs.z, 
        )
    }
}

impl std::ops::Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3f::new(
            self.x - rhs.x, 
            self.y - rhs.y,
            self.z - rhs.z, 
        )
    }
}

impl std::ops::Mul for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3f::new(
            self.x * rhs.x, 
            self.y * rhs.y,
            self.z * rhs.z, 
        )
    }
}

impl std::ops::Div for Vec3f {
    type Output = Vec3f;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3f::new(
            self.x / rhs.x, 
            self.y / rhs.y,
            self.z / rhs.z, 
        )
    }
}

impl std::ops::AddAssign for Vec3f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z; 
    }
}

impl std::ops::SubAssign for Vec3f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z; 
    }
}

impl std::ops::MulAssign for Vec3f {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z; 
    }
}

impl std::ops::DivAssign for Vec3f {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z; 
    }
}