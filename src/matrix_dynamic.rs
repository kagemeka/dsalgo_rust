pub struct Shape {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format_str = self
            .data
            .iter()
            .map(|row| format!("{:?}", row))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", format_str)
    }
}

impl<T> Matrix<T> {
    pub fn new(height: usize, width: usize) -> Matrix<T>
    where
        T: Default + Clone,
    {
        Matrix {
            data: vec![vec![T::default(); width]; height],
        }
    }

    pub fn shape(&self) -> Shape {
        let (height, width) = if self.data.len() == 0 {
            (0, 0)
        } else {
            (self.data.len(), self.data[0].len())
        };
        Shape { height, width }
    }

    pub fn transpose(&self) -> Self
    where
        T: Default + Clone,
    {
        let original_shape = self.shape();
        let mut result = Matrix::new(original_shape.width, original_shape.height);
        for i in 0..original_shape.height {
            for j in 0..original_shape.width {
                result.data[j][i] = self.data[i][j].clone();
            }
        }
        result
    }

    pub fn reverse(&self) -> Self
    where
        T: Clone,
    {
        let mut result = Self {
            data: self.data.clone(),
        };
        let height = self.shape().height;
        for i in 0..height / 2 {
            result.data.swap(i, height - i - 1);
        }
        result
    }

    pub fn rotate_counterclockwise(&self) -> Self
    where
        T: Default + Clone,
    {
        self.transpose().reverse()
    }

    pub fn rotate_clockwise(&self) -> Self
    where
        T: Default + Clone,
    {
        self.reverse().transpose()
    }
}

impl<T> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output { &self.data[index.0][index.1] }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (height, width) = (3, 4);
        let mut matrix = super::Matrix::<usize>::new(height, width);
        println!("{:?}", matrix);
        matrix[(1, 1)] += 1;
        println!();
        println!("{:?}", matrix);
        println!();
        println!("{:?}", matrix.transpose());
        println!();
        println!("{:?}", matrix);

        for row in 0..height {
            for col in 0..width {
                matrix[(row, col)] = row * width + col;
            }
        }
        println!();
        println!("{:?}", matrix);
        println!();
        println!("{:?}", matrix.reverse());
        println!();
        println!("{:?}", matrix.rotate_counterclockwise());
        println!();
        println!("{:?}", matrix.rotate_clockwise());
    }
}
