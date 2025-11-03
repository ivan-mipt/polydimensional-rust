pub mod linear_algebra {
    #[derive(Debug, Clone, Copy)]
    pub struct Vector2 {
        x: f64,
        y: f64
    }
    
    impl Vector2 {
        pub fn new(x: f64, y: f64) -> Self {
            Self {x, y}
        }

        pub fn middle(&self) -> f64 {
            (self.x + self.y) / 2.0
        }

        pub fn range(&self) -> f64 {
            (self.x - self.y).abs()
        }

        pub fn dot_product(&self, other: &Vector2) -> f64 {
            self.x * other.x + self.y * other.y
        }

        pub fn magnitude(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Vector3 {
        x: f64,
        y: f64,
        z: f64
    }

    impl Vector3 {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Self {x, y, z}
        }
        pub fn middle(&self) -> f64 {
            (self.x + self.y + self.z) / 3.0
        }
        pub fn range(&self) -> f64 {
             self.x.max(self.y).max(self.z) - self.x.min(self.y).min(self.z)
        }
        pub fn dot_product(&self, other: &Vector3) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }
        pub fn magnitude(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }
        pub fn cross_product(&self, other: &Vector3) -> Vector3 {
            Vector3 {
                x: self.y * other.z - other.y * self.z,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x
            }
        }
        pub fn normalize(&self) -> Option<Self> {
            let mag = self.magnitude();
            if mag == 0.0 {
                None
            } else {
                Some(Self {
                    x: self.x / mag,
                    y: self.y / mag,
                    z: self.z / mag
                })
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Matrix3 {
        pub data: [[f64; 3]; 3]
    }

    impl Matrix3 {
        pub fn new(
            m11: f64, m12: f64, m13: f64,
            m21: f64, m22: f64, m23: f64, 
            m31: f64, m32: f64, m33: f64
        ) -> Self {
            Self {
                data: [
                    [m11, m12, m13],
                    [m21, m22, m23], 
                    [m31, m32, m33]
                ]
            }
        }
    

        pub fn get(&self, row: usize, col: usize) -> f64 {
            self.data[row][col]
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Vector4 {
        pub x: f64,  
        pub y: f64,
        pub z: f64, 
        pub w: f64,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Matrix4 {
        pub data: [[f64; 4]; 4]
    }
    impl Matrix4 {
        pub fn new(
            m11: f64, m12: f64, m13: f64, m14: f64,  
            m21: f64, m22: f64, m23: f64, m24: f64,   
            m31: f64, m32: f64, m33: f64, m34: f64,  
            m41: f64, m42: f64, m43: f64, m44: f64   
        ) -> Self {
            Self {
                data: [
                    [m11, m12, m13, m14],
                    [m21, m22, m23, m24],
                    [m31, m32, m33, m34],
                    [m41, m42, m43, m44]
                ]
            }
        }
    }
    impl Vector4 {
        pub fn dot_product(&self, other: &Vector4) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
        }
        pub fn magnitude(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
        }
        pub fn range(&self) -> f64 {
            self.x.max(self.y).max(self.z).max(self.w) - self.x.min(self.y).min(self.z).min(self.w)
        }
        pub fn middle(&self) -> f64 {
            (self.x + self.y + self.z + self.w) / 4.0
        }
    }
}