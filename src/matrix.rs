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

use crate::vector::*;

pub union Mat3x3f {
    pub m: [[f32; 3]; 3],
    pub values: [f32; 3*3],
    pub rows: [Vec3f; 3],
}

impl Mat3x3f {
    pub fn new() -> Mat3x3f {
        Mat3x3f {
            m: [[0.0; 3]; 3],
        }
    }

    pub fn identity() -> Mat3x3f {
        unsafe {
            let mut identity = Mat3x3f::new();
            identity.m[0][0] = 1.0;
            identity.m[1][1] = 1.0;
            identity.m[2][2] = 1.0;
            identity
        }
    }

    pub fn translation(x: f32, y: f32) -> Mat3x3f {
        unsafe {
            let mut translation = Mat3x3f::identity();
            translation.m[0][2] = x;
            translation.m[1][2] = y;
            translation
        }
    }

    pub fn scale(x_scale: f32, y_scale: f32) -> Mat3x3f {
        unsafe {
            let mut scale = Mat3x3f::new();
            scale.m[0][0] = x_scale;
            scale.m[1][1] = y_scale;
            scale.m[2][2] = 1.0;
            scale
        }
    }

    pub fn rotation(angle: f32) -> Mat3x3f {
        unsafe {
            let mut rotation = Mat3x3f::identity();
            rotation.m[0][0] =  angle.cos();
            rotation.m[1][0] =  angle.sin();
            rotation.m[0][1] = -angle.sin();
            rotation.m[1][1] =  angle.cos();
            rotation
        }
    }

    pub fn mult(lhs: &Mat3x3f, rhs: &Mat3x3f) -> Mat3x3f {
        unsafe {
            let mut new_matrix = Mat3x3f::new();

            for r in 0..3 {
                for c in 0..3 {
                    new_matrix.m[r][c] = lhs.m[r][0] * rhs.m[0][c] + lhs.m[r][1] * rhs.m[1][c] + lhs.m[r][2] * rhs.m[2][c];
                }
            }

            new_matrix
        }
    }

    pub fn mult_vec(lhs: &Mat3x3f, rhs: &Vec3f) -> Vec3f {
        unsafe {
            let mut res = Vec3f::new(0.0, 0.0, 0.0);
            res.x = (Vec3f::mult(&lhs.rows[0], rhs)).sum();
            res.y = (Vec3f::mult(&lhs.rows[1], rhs)).sum();
            res.z = (Vec3f::mult(&lhs.rows[2], rhs)).sum();
            res
        }
    }
}

pub union Mat4x4f {
    pub m: [[f32; 4]; 4],
    pub values: [f32; 4*4],
    pub rows: [Vec4f; 4],
}

impl Mat4x4f {
    pub fn new() -> Mat4x4f {
        Mat4x4f {
            m: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> Mat4x4f {
        unsafe {
            let mut identity = Mat4x4f::new();
            identity.m[0][0] = 1.0;
            identity.m[1][1] = 1.0;
            identity.m[2][2] = 1.0;
            identity.m[3][3] = 1.0;
            identity
        }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Mat4x4f {
        unsafe {
            let mut translation = Mat4x4f::identity();
            translation.m[0][3] = x;
            translation.m[1][3] = y;
            translation.m[2][3] = z;
            translation
        }
    }

    pub fn scale(x_scale: f32, y_scale: f32, z_scale: f32) -> Mat4x4f {
        unsafe {
            let mut scale = Mat4x4f::new();
            scale.m[0][0] = x_scale;
            scale.m[1][1] = y_scale;
            scale.m[2][2] = z_scale;
            scale.m[3][3] = 1.0;
            scale
        }
    }

    pub fn rotation_x(angle: f32) -> Mat4x4f {
        unsafe {
            let mut rotation = Mat4x4f::identity();
            rotation.m[1][1] =  angle.cos();
            rotation.m[2][1] =  angle.sin();
            rotation.m[1][2] = -angle.sin();
            rotation.m[2][2] =  angle.cos();
            rotation
        }
    }

    pub fn rotation_y(angle: f32) -> Mat4x4f {
        unsafe {
            let mut rotation = Mat4x4f::identity();
            rotation.m[0][0] =  angle.cos();
            rotation.m[0][2] =  angle.sin();
            rotation.m[2][0] = -angle.sin();
            rotation.m[2][2] =  angle.cos();
            rotation
        }
    }

    pub fn rotation_z(angle: f32) -> Mat4x4f {
        unsafe {
            let mut rotation = Mat4x4f::identity();
            rotation.m[0][0] =  angle.cos();
            rotation.m[1][0] =  angle.sin();
            rotation.m[0][1] = -angle.sin();
            rotation.m[1][1] =  angle.cos();
            rotation
        }
    }

    pub fn mult(lhs: &Mat4x4f, rhs: &Mat4x4f) -> Mat4x4f {
        unsafe {
            let mut new_matrix = Mat4x4f::new();

            for r in 0..4 {
                for c in 0..4 {
                    new_matrix.m[r][c] = lhs.m[r][0] * rhs.m[0][c] + lhs.m[r][1] * rhs.m[1][c] + lhs.m[r][2] * rhs.m[2][c] + lhs.m[r][3] * rhs.m[3][c];
                }
            }

            new_matrix
        }
    }

    pub fn mult_vec(lhs: &Mat4x4f, rhs: &Vec4f) -> Vec4f {
        unsafe {
            let mut res = Vec4f::new(0.0, 0.0, 0.0, 0.0);
            res.x = (Vec4f::mult(&lhs.rows[0], rhs)).sum();
            res.y = (Vec4f::mult(&lhs.rows[1], rhs)).sum();
            res.z = (Vec4f::mult(&lhs.rows[2], rhs)).sum();
            res.w = (Vec4f::mult(&lhs.rows[3], rhs)).sum();
            res
        }
    }

    pub fn projection(aspect_ratio: f32, fov: f32, far: f32, near: f32) -> Mat4x4f {
        unsafe {
            let mut projection = Mat4x4f::new();
            projection.m[0][0] = aspect_ratio * fov;
            projection.m[1][1] = fov;
            projection.m[2][2] = far / (far - near);
            projection.m[2][3] = (-far * near) / (far - near);
            projection.m[3][2] = 1.0;
            projection.m[3][3] = 0.0;
            projection
        }
    }
}