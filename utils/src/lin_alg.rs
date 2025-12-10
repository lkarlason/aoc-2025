pub struct Matrix {
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(vec: Vec<Vec<f64>>) -> Self {
        Self { data: vec }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data.get(0).map(|row| row.len()).unwrap_or(0)
    }

    pub fn gauss_elim(&self, b: &Vec<f64>) -> Option<Vec<f64>> {
        let N = self.rows();
        let M = self.cols();

        if N != b.len() {
            return None;
        }

        let mut data = self.data();
        let mut bc = vec![0.0f64; M];
        for i in 0..N {
            bc[i] = b[i];
        }
        let eps = 1e-9;

        let mut pivot_col = vec![None; N];
        let mut row = 0usize;

        for col in 0..M {
            let mut pivot = row;
            let mut max_val = self.data[row][col].abs();
            for r in (row + 1)..N {
                let v = self.data[r][col].abs();
                if v > max_val {
                    max_val = v;
                    pivot = r;
                }
            }

            if max_val < eps {
                continue;
            }

            data.swap(row, pivot);
            bc.swap(row, pivot);

            let pivot_val = data[row][col];
            for c in col..M {
                data[row][c] /= pivot_val;
            }
            bc[row] /= pivot_val;

            for r in 0..N {
                if r == row {
                    continue;
                }

                let factor = data[r][col];
                if factor.abs() < eps {
                    continue;
                }

                for c in col..M {
                    data[r][c] -= factor * data[row][c];
                }
                bc[r] -= factor * bc[row];
            }

            pivot_col[row] = Some(col);
            row += 1;

            if row == N {
                break;
            }
        }

        let mut res = vec![0.0f64; M];
        for j in 0..M {
            for i in 0..N {
                if let Some(pc) = pivot_col[i]
                    && pc == j
                {
                    res[j] = b[i];
                    break;
                }
            }
        }

        Some(res)
    }

    pub fn reduced_echelon(&self) -> Vec<Vec<f64>> {
        let n = self.rows();
        let m = self.cols();
        let mut data = self.data();
        let eps = 1e-9;
        let mut row = 0usize;

        for col in 0..m {
            let mut pivot = row;
            let mut max_val = data[row][col].abs();
            for r in (row + 1)..n {
                let v = data[r][col].abs();
                if v > max_val {
                    max_val = v;
                    pivot = r;
                }
            }

            if max_val < eps {
                continue;
            }

            data.swap(row, pivot);

            let pivot_val = data[row][col];
            for c in col..m {
                data[row][c] /= pivot_val;
            }

            for r in 0..n {
                if r == row {
                    continue;
                }

                let factor = data[r][col];
                if factor.abs() < eps {
                    continue;
                }

                for c in col..m {
                    data[r][c] -= factor * data[row][c];
                }
            }

            row += 1;

            if row == n {
                break;
            }
        }

        data
    }

    pub fn vec_mult(&self, b: &Vec<f64>) -> Option<Vec<f64>> {
      let n = self.rows();
      let m = self.cols();

      if m != b.len() {
        return None;
      }

      let mut res = vec![0.0f64; n];

      for i in 0..n {
        res[i] = 0.0;
        for j in 0..m {
          res[i] += self.data[i][j] * b[j];
        }
      }

      Some(res)
    }

    pub fn rank(&self) -> usize {
        let echelon = self.reduced_echelon();
        let eps = 1e-9;

        let mut rank = 0;
        for row in echelon {
            for col in row {
                if col > eps {
                    rank += 1;
                    break;
                }
            }
        }

        rank
    }

    fn data(&self) -> Vec<Vec<f64>> {
        let N = self.rows();
        let M = self.cols();
        let mut data = vec![vec![0.0f64; M]; N];

        for i in 0..N {
            for j in 0..M {
                data[i][j] = self.data[i][j];
            }
        }

        data
    }
}
