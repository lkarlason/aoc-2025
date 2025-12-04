#[derive(Debug)]
pub struct Grid<T>
where
    T: Copy,
{
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(data: Vec<T>, rows: usize) -> Self {
        let cols = data.len() / rows;

        Grid { rows, cols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if row < self.rows && col < self.cols {
            return Some(self.data[row * self.cols + col]);
        }
        None
    }

    pub fn set(&mut self, row: usize, col: usize, elem: T) -> Option<T>  {
      if row < self.rows && col < self.cols {
        self.data[row * self.cols + col] = elem;
        Some(elem)
      } else {
        None
      }
    }

    pub fn get_grid_range(&self, rows: (usize, usize), cols: (usize, usize)) -> Vec<T> {
        if rows.1 < self.rows && rows.0 < rows.1 && cols.1 < self.cols && cols.0 < cols.1 {
            let mut range = Vec::with_capacity((rows.1 - rows.0) + (cols.1 - cols.0));
            for i in rows.0..=rows.1 {
                for j in cols.0..=cols.1 {
                    match self.get(i, j) {
                        Some(elem) => range.push(elem),
                        _ => return vec![],
                    }
                }
            }
            return range;
        }

        vec![]
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}
