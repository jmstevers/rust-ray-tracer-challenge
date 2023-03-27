use std::ops::Mul;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix4x4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn new(data: [[f32; 4]; 4]) -> Matrix4x4 {
        Matrix4x4 { data }
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4x4 {
        Matrix4x4::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4x4 {
        Matrix4x4::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn x_rotation(r: f32) -> Matrix4x4 {
        Matrix4x4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn zero() -> Matrix4x4 {
        Matrix4x4::new([[0.0; 4]; 4])
    }

    pub fn transpose(self) -> Matrix4x4 {
        let mut result = Matrix4x4::zero();

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[j][i]
            }
        }

        result
    }

    pub fn submatrix(self, row: usize, col: usize) -> Matrix3x3 {
        let mut nums: Vec<f32> = vec![];
        for i in 0..4 {
            if i == row {
                continue;
            }
            for j in 0..4 {
                if j == col {
                    continue;
                }
                nums.push(self.data[i][j])
            }
        }

        let mut result = Matrix3x3::zero();
        let mut count = 0;
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = nums[i + j + count];
            }
            count += 2;
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
        let mut result = 0.0;

        for i in 0..4 {
            result += self.data[0][i] * self.cofactor(0, i);
        }

        result
    }

    pub fn round(self, decimal_point: f32) -> Matrix4x4 {
        Matrix4x4::new(
            self.data
                .map(|i| i.map(|j| (j * decimal_point).round() / decimal_point)),
        )
    }

    pub fn inverse(self) -> Result<Matrix4x4, &'static str> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            return Err("Determinant of matrix is 0.");
        }

        let mut result = Matrix4x4::zero();

        for i in 0..4 {
            for j in 0..4 {
                let cofactor = self.cofactor(i, j);

                result.data[j][i] = cofactor / determinant
            }
        }

        Ok(result)
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut result = Matrix4x4::new([[0.0; 4]; 4]);

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][0] * rhs.data[0][j]
                    + self.data[i][1] * rhs.data[1][j]
                    + self.data[i][2] * rhs.data[2][j]
                    + self.data[i][3] * rhs.data[3][j];
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

    pub fn submatrix(self, row: usize, col: usize) -> Matrix2x2 {
        let mut nums: Vec<f32> = vec![];
        for i in 0..3 {
            if i == row {
                continue;
            }
            for j in 0..3 {
                if j == col {
                    continue;
                }
                nums.push(self.data[i][j])
            }
        }

        let mut result = Matrix2x2::zero();
        let mut count = 0;
        for i in 0..2 {
            for j in 0..2 {
                result.data[i][j] = nums[i + j + count];
            }
            count += 1;
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
        let mut result = 0.0;

        for i in 0..3 {
            result += self.data[0][i] * self.cofactor(0, i);
        }

        result
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
