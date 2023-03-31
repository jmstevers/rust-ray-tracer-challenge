use unroll::unroll_for_loops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix4x4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub const fn new(data: [[f32; 4]; 4]) -> Self {
        Matrix4x4 { data }
    }

    pub const fn zero() -> Self {
        Matrix4x4 {
            data: [[0.0; 4]; 4],
        }
    }

    pub const fn identity() -> Self {
        Matrix4x4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        self.mul(Matrix4x4::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        self.mul(Matrix4x4::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotate_x(&self, r: f32) -> Self {
        self.mul(Matrix4x4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotate_y(&self, r: f32) -> Self {
        self.mul(Matrix4x4::new([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotate_z(&self, r: f32) -> Self {
        self.mul(Matrix4x4::new([
            [r.cos(), -r.sin(), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn shear(&self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        self.mul(Matrix4x4::new([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    #[unroll_for_loops]
    pub fn transpose(&self) -> Self {
        let mut result = Matrix4x4::zero();

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[j][i]
            }
        }

        result
    }

    #[unroll_for_loops]
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        let mut result = Matrix3x3::zero();
        let mut row_offset = 0;
        for i in 0..4 {
            if i != row {
                let mut col_offset = 0;
                for j in 0..4 {
                    if j != col {
                        result.data[i - row_offset][j - col_offset] = self.data[i][j];
                    } else {
                        col_offset = 1;
                    }
                }
            } else {
                row_offset = 1;
            }
        }
        result
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);

        match (row + col) % 2 {
            1 => return -minor,
            0 => return minor,
            _ => panic!(),
        }
    }

    #[unroll_for_loops]
    pub fn determinant(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..4 {
            sum += self.data[i][0] * self.cofactor(i, 0);
        }
        sum
    }

    pub fn round(&self, decimal_point: f32) -> Self {
        Matrix4x4::new(
            self.data
                .map(|i| i.map(|j| (j * decimal_point).round() / decimal_point)),
        )
    }

    #[unroll_for_loops]
    pub fn inverse(&self) -> Self {
        let determinant = self.determinant();
        let mut result = Matrix4x4::zero();

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.cofactor(j, i) / determinant;
            }
        }

        result
    }

    #[unroll_for_loops]
    pub fn mul(&self, rhs: Matrix4x4) -> Self {
        let mut result = Matrix4x4::zero();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        result
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix3x3 {
    pub data: [[f32; 3]; 3],
}

impl Matrix3x3 {
    pub fn new(data: [[f32; 3]; 3]) -> Matrix3x3 {
        Matrix3x3 { data }
    }

    pub fn zero() -> Matrix3x3 {
        Matrix3x3::new([[0.0; 3]; 3])
    }
    #[unroll_for_loops]
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        let mut result = Matrix2x2::zero();
        let mut row_offset = 0;
        for i in 0..3 {
            if i != row {
                let mut col_offset = 0;
                for j in 0..3 {
                    if j != col {
                        result.data[i - row_offset][j - col_offset] = self.data[i][j];
                    } else {
                        col_offset = 1;
                    }
                }
            } else {
                row_offset = 1;
            }
        }
        result
    }

    pub fn minor(self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);

        match (row + col) % 2 {
            1 => return -minor,
            0 => return minor,
            _ => panic!(),
        }
    }

    pub fn determinant(self) -> f32 {
        let mut sum = 0.0;
        for i in 0..3 {
            sum += self.data[i][0] * self.cofactor(i, 0)
        }
        sum
    }
}

#[derive(PartialEq, Debug)]
pub struct Matrix2x2 {
    pub data: [[f32; 2]; 2],
}

impl Matrix2x2 {
    pub fn new(data: [[f32; 2]; 2]) -> Matrix2x2 {
        Matrix2x2 { data }
    }

    pub fn zero() -> Matrix2x2 {
        Matrix2x2::new([[0.0; 2]; 2])
    }

    pub fn determinant(self) -> f32 {
        self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix4x4_determinant() {
        let m = Matrix4x4::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn test_matrix4x4_submatrix() {
        let m = Matrix4x4::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let sub = m.submatrix(2, 1);
        assert_eq!(
            sub,
            Matrix3x3::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]])
        );
    }

    #[test]
    fn test_matrix4x4_minor() {
        let m = Matrix4x4::new([
            [3.0, 5.0, 0.0, 3.0],
            [2.0, -1.0, -7.0, 4.0],
            [6.0, -1.0, 5.0, 6.0],
            [-6.0, 0.0, 9.0, 1.0],
        ]);
        let sub = m.submatrix(1, 0);
        assert_eq!(sub.determinant(), 25.0);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn test_matrix4x4_cofactor() {
        let m = Matrix3x3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.], [6.0, -1.0, 5.0]]);
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }
}
