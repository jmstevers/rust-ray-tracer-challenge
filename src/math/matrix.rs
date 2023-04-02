use unroll::unroll_for_loops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix4x4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub const fn new(data: [[f32; 4]; 4]) -> Self {
        Self { data }
    }

    pub const fn zero() -> Self {
        Self {
            data: [[0.0; 4]; 4],
        }
    }

    pub const fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        self.mul(Self::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        self.mul(Self::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotate_x(&self, r: f32) -> Self {
        self.mul(Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotate_y(&self, r: f32) -> Self {
        self.mul(Self::new([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotate_z(&self, r: f32) -> Self {
        self.mul(Self::new([
            [r.cos(), -r.sin(), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn shear(&self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        self.mul(Self::new([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    #[unroll_for_loops]
    pub fn transpose(&self) -> Self {
        let mut result = Self::zero();

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
        let mut dest_row = 0;

        for (src_row, src) in self.data.iter().enumerate() {
            if src_row != row {
                let mut dest_col = 0;
                for (src_col, value) in src.iter().enumerate() {
                    if src_col != col {
                        result.data[dest_row][dest_col] = *value;
                        dest_col += 1;
                    }
                }
                dest_row += 1;
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
        Self::new(
            self.data
                .map(|i| i.map(|j| (j * decimal_point).round() / decimal_point)),
        )
    }

    #[unroll_for_loops]
    pub fn inverse(&self) -> Option<Self> {
        let determinant = self.determinant();

        if determinant == 0.0 {
            return None;
        }

        let inv_determinant = 1.0 / determinant;
        let mut result = Self::zero();

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.cofactor(j, i) * inv_determinant;
            }
        }

        Some(result)
    }

    #[unroll_for_loops]
    pub fn mul(&self, rhs: Self) -> Self {
        let mut result = Self::zero();
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
    pub fn new(data: [[f32; 3]; 3]) -> Self {
        Self { data }
    }

    pub fn zero() -> Self {
        Self::new([[0.0; 3]; 3])
    }

    #[unroll_for_loops]
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        let mut result = Matrix2x2::identity();
        let mut dest_row = 0;

        for (src_row, src) in self.data.iter().enumerate() {
            if src_row != row {
                let mut dest_col = 0;
                for (src_col, value) in src.iter().enumerate() {
                    if src_col != col {
                        result.data[dest_row][dest_col] = *value;
                        dest_col += 1;
                    }
                }
                dest_row += 1;
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

    #[unroll_for_loops]
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
    pub fn new(data: [[f32; 2]; 2]) -> Self {
        Self { data }
    }

    pub fn identity() -> Self {
        Self::new([[1.0, 0.0], [0.0, 1.0]])
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
        assert_eq!(sub.determinant(), -272.0);
        assert_eq!(m.minor(1, 0), -272.0);
    }

    #[test]
    fn test_matrix4x4_cofactor() {
        let m = Matrix3x3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.], [6.0, -1.0, 5.0]]);
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_matrix4x4_translate() {
        let m = Matrix4x4::identity().translate(5.0, -3.0, 2.0);
        assert_eq!(
            m,
            Matrix4x4::new([
                [1.0, 0.0, 0.0, 5.0],
                [0.0, 1.0, 0.0, -3.0],
                [0.0, 0.0, 1.0, 2.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_matrix4x4_scale() {
        let m = Matrix4x4::identity().scale(2.0, 3.0, 4.0);
        assert_eq!(
            m,
            Matrix4x4::new([
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 3.0, 0.0, 0.0],
                [0.0, 0.0, 4.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }
}
