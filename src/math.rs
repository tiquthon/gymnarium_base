use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use serde::{Deserialize, Serialize};

/* --- --- --- Position2D --- --- --- */

/// A position inside the two dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position2D {
    pub x: f64,
    pub y: f64,
}

impl Position2D {
    pub fn with(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0f64, y: 0f64 }
    }

    pub fn one() -> Self {
        Self { x: 1f64, y: 1f64 }
    }

    pub fn vector_to(&self, other: &Position2D) -> Vector2D {
        Vector2D::with(other.x - self.x, other.y - self.y)
    }

    pub fn distance_to(&self, other: &Position2D) -> f64 {
        self.vector_to(other).length()
    }

    pub fn transform(&self, transformations: &Transformations2D) -> Self {
        let transformed = multiply_vector_1x3_and_matrix_3x3(
            [self.x, self.y, 1f64],
            transformations.transformation_matrix(),
        );
        Self {
            x: transformed[0],
            y: transformed[1],
        }
    }
}

impl Add<Vector2D> for Position2D {
    type Output = Self;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector2D> for Position2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2D> for Position2D {
    type Output = Self;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector2D> for Position2D {
    fn sub_assign(&mut self, rhs: Vector2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/* --- --- --- Position3D --- --- --- */

/// A position inside the three dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position3D {
    pub fn with(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }

    pub fn one() -> Self {
        Self {
            x: 1f64,
            y: 1f64,
            z: 1f64,
        }
    }

    pub fn vector_to(&self, other: &Position3D) -> Vector3D {
        Vector3D::with(other.x - self.x, other.y - self.y, other.z - self.z)
    }

    pub fn distance_to(&self, other: &Position3D) -> f64 {
        self.vector_to(other).length()
    }

    /*
    pub fn rotate_x_around_origin(&self, degree: f64) -> Self {
        self.rotate_x_around(&Position3D::zero(), degree)
    }

    pub fn rotate_y_around_origin(&self, degree: f64) -> Self {
        self.rotate_y_around(&Position3D::zero(), degree)
    }

    pub fn rotate_z_around_origin(&self, degree: f64) -> Self {
        self.rotate_z_around(&Position3D::zero(), degree)
    }

    pub fn rotate_x_around(&self, rotate_position: &Position3D, degree: f64) -> Self {
        let radians = degree / 180f64 * std::f64::consts::PI;
        let (radians_sin, radians_cos) = radians.sin_cos();
        Position3D::with(
            self.x,
            self.y * radians_cos - self.z * radians_sin + rotate_position.y * radians_cos
                - rotate_position.z * radians_sin - rotate_position.y,
            self.y * radians_cos + self.z * radians_sin + rotate_position.y * radians_cos
                + rotate_position.z * radians_sin - rotate_position.z,
        )
    }

    pub fn rotate_y_around(&self, rotate_position: &Position3D, degree: f64) -> Self {
        let radians = degree / 180f64 * std::f64::consts::PI;
        let (radians_sin, radians_cos) = radians.sin_cos();
        Position3D::with(
            self.x * radians_cos + self.z * radians_sin + rotate_position.x * radians_cos
                + rotate_position.z * radians_sin - rotate_position.x,
            self.y,
            -self.x * radians_sin + self.z * radians_cos - rotate_position.x * radians_sin
                + rotate_position.z * radians_cos - rotate_position.z
        )
    }

    pub fn rotate_z_around(&self, rotate_position: &Position3D, degree: f64) -> Self {
        let radians = degree / 180f64 * std::f64::consts::PI;
        let (radians_sin, radians_cos) = radians.sin_cos();
        Position3D::with(
            self.x * radians_cos - self.y * radians_sin + rotate_position.x * radians_cos - rotate_position.y * radians_sin - rotate_position.x,
            self.x * radians_sin + self.y * radians_cos + rotate_position.x * radians_sin + rotate_position.y * radians_cos - rotate_position.y,
            self.z
        )
    }*/

    pub fn transform(&self, _transformation: Transformation3D) -> Self {
        todo!()
    }
}

impl Add<Vector3D> for Position3D {
    type Output = Self;

    fn add(self, rhs: Vector3D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector3D> for Position3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vector3D> for Position3D {
    type Output = Self;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vector3D> for Position3D {
    fn sub_assign(&mut self, rhs: Vector3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/* --- --- --- Vector2D --- --- --- */

/// A vector inside the two dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn with(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0f64, y: 0f64 }
    }

    pub fn one() -> Self {
        Self { x: 1f64, y: 1f64 }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalized(self) -> Self {
        self / self.length()
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
        }
    }
}

impl MulAssign<f32> for Vector2D {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs as f64;
        self.y *= rhs as f64;
    }
}

impl Mul<f64> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f64> for Vector2D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vector2D {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs as f64,
            y: self.y / rhs as f64,
        }
    }
}

impl DivAssign<f32> for Vector2D {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs as f64;
        self.y /= rhs as f64;
    }
}

impl Div<f64> for Vector2D {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f64> for Vector2D {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vector2D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

/* --- --- --- Vector3D --- --- --- */

/// A vector inside the three dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn with(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }

    pub fn one() -> Self {
        Self {
            x: 1f64,
            y: 1f64,
            z: 1f64,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalized(self) -> Self {
        self / self.length()
    }
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f32> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
            z: self.z * rhs as f64,
        }
    }
}

impl MulAssign<f32> for Vector3D {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs as f64;
        self.y *= rhs as f64;
        self.z *= rhs as f64;
    }
}

impl Mul<f64> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vector3D {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs as f64,
            y: self.y / rhs as f64,
            z: self.z / rhs as f64,
        }
    }
}

impl DivAssign<f32> for Vector3D {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs as f64;
        self.y /= rhs as f64;
        self.z /= rhs as f64;
    }
}

impl Div<f64> for Vector3D {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/* --- --- --- Size2D --- --- --- */

/// A size inside the two dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Size2D {
    pub width: f64,
    pub height: f64,
}

impl Size2D {
    pub fn with(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn zero() -> Self {
        Self {
            width: 0f64,
            height: 0f64,
        }
    }

    pub fn one() -> Self {
        Self {
            width: 1f64,
            height: 1f64,
        }
    }

    pub fn scale(&self, width_factor: f64, height_factor: f64) -> Self {
        Self {
            width: self.width * width_factor,
            height: self.height * height_factor,
        }
    }
}

/* --- --- --- Size3D --- --- --- */

/// A size inside the three dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Size3D {
    pub width: f64,
    pub height: f64,
    pub length: f64,
}

impl Size3D {
    pub fn with(width: f64, height: f64, length: f64) -> Self {
        Self {
            width,
            height,
            length,
        }
    }

    pub fn zero() -> Self {
        Self {
            width: 0f64,
            height: 0f64,
            length: 0f64,
        }
    }

    pub fn one() -> Self {
        Self {
            width: 1f64,
            height: 1f64,
            length: 1f64,
        }
    }

    pub fn scale(&self, width_factor: f64, height_factor: f64, length_factor: f64) -> Self {
        Self {
            width: self.width * width_factor,
            height: self.height * height_factor,
            length: self.length * length_factor,
        }
    }
}

/* --- --- --- Transformation2D --- --- --- */

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Transformation2D {
    Translation {
        direction: Vector2D,
    },
    Identity,
    Rotation {
        angle_in_degree: f64,
    },
    Scale {
        x_factor: f64,
        y_factor: f64,
    },
    IsotropicScale {
        factor: f64,
    },
    ReflectionX,
    ReflectionY,
    ShearX {
        amount: f64,
    },
    ShearY {
        amount: f64,
    },
    ShearXDegree {
        degree: f64,
    },
    ShearYDegree {
        degree: f64,
    },
    Composition {
        name: String,
        transformations: Vec<Transformation2D>,
    },
    Custom {
        name: String,
        transformation: [[f64; 3]; 3],
    },
}

impl Transformation2D {
    pub fn translation(direction: Vector2D) -> Self {
        Self::Translation { direction }
    }

    pub fn identity() -> Self {
        Self::Identity
    }

    pub fn rotation(angle_in_degree: f64) -> Self {
        Self::Rotation { angle_in_degree }
    }

    pub fn scale(x_factor: f64, y_factor: f64) -> Self {
        Self::Scale { x_factor, y_factor }
    }

    pub fn isotropic_scale(factor: f64) -> Self {
        Self::IsotropicScale { factor }
    }

    pub fn reflection_x() -> Self {
        Self::ReflectionX
    }

    pub fn reflection_y() -> Self {
        Self::ReflectionY
    }

    pub fn shear_x(amount: f64) -> Self {
        Self::ShearX { amount }
    }

    pub fn shear_y(amount: f64) -> Self {
        Self::ShearY { amount }
    }

    pub fn shear_x_degree(degree: f64) -> Self {
        Self::ShearXDegree { degree }
    }

    pub fn shear_y_degree(degree: f64) -> Self {
        Self::ShearYDegree { degree }
    }

    pub fn composition(name: String, transformations: Vec<Transformation2D>) -> Self {
        Self::Composition {
            name,
            transformations,
        }
    }

    pub fn custom(name: String, transformation: [[f64; 3]; 3]) -> Self {
        Self::Custom {
            name,
            transformation,
        }
    }
}

impl Transformation2D {
    pub fn rotation_around_position(rotation_position: &Position2D, angle: f64) -> Self {
        Self::composition(
            "RotationAroundPosition".to_string(),
            vec![
                Self::translation(rotation_position.vector_to(&Position2D::zero())),
                Self::rotation(angle),
                Self::translation(Position2D::zero().vector_to(&rotation_position)),
            ],
        )
    }
}

impl Transformation2D {
    pub fn transformation_matrix(&self) -> [[f64; 3]; 3] {
        match self {
            Self::Translation { direction } => [
                [1f64, 0f64, direction.x],
                [0f64, 1f64, direction.y],
                [0f64, 0f64, 1f64],
            ],
            Self::Identity => [[1f64, 0f64, 0f64], [0f64, 1f64, 0f64], [0f64, 0f64, 1f64]],
            Self::Rotation { angle_in_degree } => [
                [
                    degrees_to_radians(*angle_in_degree).cos(),
                    -(degrees_to_radians(*angle_in_degree).sin()),
                    0f64,
                ],
                [
                    degrees_to_radians(*angle_in_degree).sin(),
                    degrees_to_radians(*angle_in_degree).cos(),
                    0f64,
                ],
                [0f64, 0f64, 1f64],
            ],
            Self::Scale { x_factor, y_factor } => [
                [*x_factor, 0f64, 0f64],
                [0f64, *y_factor, 0f64],
                [0f64, 0f64, 1f64],
            ],
            Self::IsotropicScale { factor } => [
                [*factor, 0f64, 0f64],
                [0f64, *factor, 0f64],
                [0f64, 0f64, 1f64],
            ],
            Self::ReflectionX => [[-1f64, 0f64, 0f64], [0f64, 1f64, 0f64], [0f64, 0f64, 1f64]],
            Self::ReflectionY => [[1f64, 0f64, 0f64], [0f64, -1f64, 0f64], [0f64, 0f64, 1f64]],
            Self::ShearX { amount } => [
                [1f64, *amount, 0f64],
                [0f64, 1f64, 0f64],
                [0f64, 0f64, 1f64],
            ],
            Self::ShearY { amount } => [
                [1f64, 0f64, 0f64],
                [*amount, 1f64, 0f64],
                [0f64, 0f64, 1f64],
            ],
            Self::ShearXDegree { degree } => [
                [1f64, degrees_to_radians(*degree).tan(), 0f64],
                [0f64, 1f64, 0f64],
                [0f64, 0f64, 1f64],
            ],
            Self::ShearYDegree { degree } => [
                [1f64, 0f64, 0f64],
                [degrees_to_radians(*degree).tan(), 1f64, 0f64],
                [0f64, 0f64, 1f64],
            ],
            Self::Composition {
                transformations, ..
            } => transformations
                .iter()
                .map(|transformation| transformation.transformation_matrix())
                .reduce(multiply_matrices_3x3)
                .unwrap_or_else(|| Self::identity().transformation_matrix()),
            Self::Custom { transformation, .. } => *transformation,
        }
    }

    pub fn transformation_matrix_as_3x2(&self) -> [[f64; 3]; 2] {
        matrix_3x3_as_matrix_3x2(self.transformation_matrix())
    }

    pub fn reverse(self) -> Self {
        match self {
            Self::Composition {
                name,
                transformations,
            } => Self::Composition {
                name: format!("Reverse-{:?}", name),
                transformations: transformations
                    .into_iter()
                    .map(|transformation| transformation.reverse())
                    .collect(),
            },
            t => Self::Custom {
                name: format!("Reverse-{:?}", t),
                transformation: inverse_of_matrix_3x3(t.transformation_matrix()),
            },
        }
    }
}

/* --- --- --- Matrix, Vector Things --- --- --- */

pub fn radians_to_degrees(radians: f64) -> f64 {
    (radians * 180f64) / std::f64::consts::PI
}

pub fn degrees_to_radians(degree: f64) -> f64 {
    (degree * std::f64::consts::PI) / 180f64
}

pub fn multiply_matrices_3x3(matrix_a: [[f64; 3]; 3], matrix_b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    [
        [
            matrix_a[0][0] * matrix_b[0][0]
                + matrix_a[1][0] * matrix_b[0][1]
                + matrix_a[2][0] * matrix_b[0][2],
            matrix_a[0][1] * matrix_b[0][0]
                + matrix_a[1][1] * matrix_b[0][1]
                + matrix_a[2][1] * matrix_b[0][2],
            matrix_a[0][2] * matrix_b[0][0]
                + matrix_a[1][2] * matrix_b[0][1]
                + matrix_a[2][2] * matrix_b[0][2],
        ],
        [
            matrix_a[0][0] * matrix_b[1][0]
                + matrix_a[1][0] * matrix_b[1][1]
                + matrix_a[2][0] * matrix_b[1][2],
            matrix_a[0][1] * matrix_b[1][0]
                + matrix_a[1][1] * matrix_b[1][1]
                + matrix_a[2][1] * matrix_b[1][2],
            matrix_a[0][2] * matrix_b[1][0]
                + matrix_a[1][2] * matrix_b[1][1]
                + matrix_a[2][2] * matrix_b[1][2],
        ],
        [
            matrix_a[0][0] * matrix_b[2][0]
                + matrix_a[1][0] * matrix_b[2][1]
                + matrix_a[2][0] * matrix_b[2][2],
            matrix_a[0][1] * matrix_b[2][0]
                + matrix_a[1][1] * matrix_b[2][1]
                + matrix_a[2][1] * matrix_b[2][2],
            matrix_a[0][2] * matrix_b[2][0]
                + matrix_a[1][2] * matrix_b[2][1]
                + matrix_a[2][2] * matrix_b[2][2],
        ],
    ]
}

pub fn multiply_vector_1x3_and_matrix_3x3(vector: [f64; 3], matrix: [[f64; 3]; 3]) -> [f64; 3] {
    [
        vector[0] * matrix[0][0] + vector[1] * matrix[0][1] + vector[2] * matrix[0][2],
        vector[0] * matrix[1][0] + vector[1] * matrix[1][1] + vector[2] * matrix[1][2],
        vector[0] * matrix[2][0] + vector[1] * matrix[2][1] + vector[2] * matrix[2][2],
    ]
}

pub fn matrix_3x3_as_matrix_3x2(matrix: [[f64; 3]; 3]) -> [[f64; 3]; 2] {
    [
        [matrix[0][0], matrix[0][1], matrix[0][2]],
        [matrix[1][0], matrix[1][1], matrix[1][2]],
    ]
}

pub fn matrix_3x2_as_homogeneous_matrix_3x3(matrix: [[f64; 3]; 2]) -> [[f64; 3]; 3] {
    [
        [matrix[0][0], matrix[0][1], matrix[0][2]],
        [matrix[1][0], matrix[1][1], matrix[1][2]],
        [0f64, 0f64, 1f64],
    ]
}

pub fn vector_1x3_as_vector_1x2(vector: [f64; 3]) -> [f64; 2] {
    [vector[0], vector[1]]
}

pub fn vector_1x2_as_homogeneous_vector_1x3(vector: [f64; 2]) -> [f64; 3] {
    [vector[0], vector[1], 1f64]
}

pub fn determinant_of_matrix_3x3(matrix: [[f64; 3]; 3]) -> f64 {
    matrix[0][0] * matrix[1][1] * matrix[2][2]
        + matrix[1][0] * matrix[2][1] * matrix[0][2]
        + matrix[2][0] * matrix[0][1] * matrix[1][2]
        - matrix[2][0] * matrix[1][1] * matrix[2][2]
        - matrix[1][0] * matrix[0][1] * matrix[2][2]
        - matrix[0][0] * matrix[2][1] * matrix[1][2]
}

pub fn inverse_of_matrix_3x3(matrix: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let determinant = determinant_of_matrix_3x3(matrix);
    [
        [
            (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1]) / determinant,
            (matrix[0][2] * matrix[2][1] - matrix[0][1] * matrix[2][2]) / determinant,
            (matrix[0][1] * matrix[1][2] - matrix[0][2] * matrix[1][1]) / determinant,
        ],
        [
            (matrix[1][2] * matrix[2][0] - matrix[1][0] * matrix[2][2]) / determinant,
            (matrix[0][0] * matrix[2][2] - matrix[0][2] * matrix[2][0]) / determinant,
            (matrix[0][2] * matrix[1][0] - matrix[0][0] * matrix[1][2]) / determinant,
        ],
        [
            (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]) / determinant,
            (matrix[0][1] * matrix[2][0] - matrix[0][0] * matrix[2][1]) / determinant,
            (matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]) / determinant,
        ],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radians_to_degrees() {
        assert_eq!(180f64, radians_to_degrees(std::f64::consts::PI));
    }

    #[test]
    fn test_degrees_to_radians() {
        assert_eq!(std::f64::consts::PI / 5f64, degrees_to_radians(36f64));
    }

    #[test]
    fn multiply_matrices_3x3_works() {
        let matrix_a = [[1f64, 2f64, 3f64], [4f64, 5f64, 6f64], [7f64, 8f64, 9f64]];
        let matrix_b = [
            [10f64, 11f64, 12f64],
            [13f64, 14f64, 15f64],
            [16f64, 17f64, 18f64],
        ];
        assert_eq!(
            [
                [138f64, 171f64, 204f64],
                [174f64, 216f64, 258f64],
                [210f64, 261f64, 312f64]
            ],
            multiply_matrices_3x3(matrix_a, matrix_b)
        );
    }

    #[test]
    fn multiply_vector_1x3_and_matrix_3x3_works() {
        let vector = [1f64, 4f64, 7f64];
        let matrix = [
            [10f64, 11f64, 12f64],
            [13f64, 14f64, 15f64],
            [16f64, 17f64, 18f64],
        ];
        assert_eq!(
            [138f64, 174f64, 210f64],
            multiply_vector_1x3_and_matrix_3x3(vector, matrix)
        );
    }

    #[test]
    fn matrix_3x3_as_matrix_3x2_works() {
        let matrix = [[1f64, 2f64, 3f64], [4f64, 5f64, 6f64], [7f64, 8f64, 9f64]];

        assert_eq!(
            [[1f64, 2f64, 3f64], [4f64, 5f64, 6f64]],
            matrix_3x3_as_matrix_3x2(matrix)
        );
    }

    #[test]
    fn matrix_3x2_as_homogeneous_matrix_3x3_works() {
        let matrix = [[1f64, 2f64, 3f64], [4f64, 5f64, 6f64]];

        assert_eq!(
            [[1f64, 2f64, 3f64], [4f64, 5f64, 6f64], [0f64, 0f64, 1f64]],
            matrix_3x2_as_homogeneous_matrix_3x3(matrix)
        );
    }

    #[test]
    fn vector_1x3_as_vector_1x2_works() {
        let vector = [1f64, 2f64, 3f64];

        assert_eq!([1f64, 2f64], vector_1x3_as_vector_1x2(vector));
    }

    #[test]
    fn vector_1x2_as_homogeneous_vector_1x3_works() {
        let vector = [1f64, 2f64];

        assert_eq!(
            [1f64, 2f64, 1f64],
            vector_1x2_as_homogeneous_vector_1x3(vector)
        );
    }

    #[test]
    fn determinant_of_matrix_3x3_works() {
        let matrix = [
            [2f64, -1f64, 0f64],
            [-1f64, 2f64, -1f64],
            [0f64, -1f64, 2f64],
        ];

        assert_eq!(4f64, determinant_of_matrix_3x3(matrix));
    }

    #[test]
    fn inverse_of_matrix_3x3_works() {
        let matrix = [
            [2f64, -1f64, 0f64],
            [-1f64, 2f64, -1f64],
            [0f64, -1f64, 2f64],
        ];

        assert_eq!(
            [
                [3f64 / 4f64, 2f64 / 4f64, 1f64 / 4f64],
                [2f64 / 4f64, 4f64 / 4f64, 2f64 / 4f64],
                [1f64 / 4f64, 2f64 / 4f64, 3f64 / 4f64]
            ],
            inverse_of_matrix_3x3(matrix)
        );
    }
}

/* --- --- --- Transformation3D --- --- --- */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Transformation3D {
    // TODO:
}

/* --- --- --- Transformations2D --- --- --- */

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transformations2D {
    pub transformations: Vec<Transformation2D>,
}

impl Transformations2D {
    pub fn transformation_matrix(&self) -> [[f64; 3]; 3] {
        self.transformations
            .iter()
            .map(|transformation| transformation.transformation_matrix())
            .reduce(multiply_matrices_3x3)
            .unwrap_or_else(|| Transformation2D::identity().transformation_matrix())
    }

    pub fn reverse(mut self) -> Self {
        self.transformations.reverse();
        Self {
            transformations: self
                .transformations
                .into_iter()
                .map(|transformation| transformation.reverse())
                .collect(),
        }
    }
}

/* --- --- --- Transformations3D --- --- --- */

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Transformations3D {
    pub transformations: Vec<Transformation3D>,
}

/* --- --- --- --- --- --- --- --- --- --- --- */
