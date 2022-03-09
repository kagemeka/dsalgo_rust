pub trait DynamicShape {
    fn shape(&self) -> crate::matrix::Shape;
}

#[derive(Clone, PartialEq)]
pub struct DynamicShapedMatrix<T> {
    data: Vec<Vec<T>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for DynamicShapedMatrix<T> {
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

impl<T> From<Vec<Vec<T>>> for DynamicShapedMatrix<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        if height > 0 {
            let width = data[0].len();
            for row in 1..height {
                assert_eq!(data[row].len(), width);
            }
        }
        Self { data }
    }
}

impl<T> DynamicShapedMatrix<T> {
    pub fn new(height: usize, width: usize) -> DynamicShapedMatrix<T>
    where
        T: Default + Clone,
    {
        DynamicShapedMatrix {
            data: vec![vec![T::default(); width]; height],
        }
    }

    pub fn shape(&self) -> crate::matrix::Shape {
        let (height, width) = if self.data.len() == 0 {
            (0, 0)
        } else {
            (self.data.len(), self.data[0].len())
        };
        crate::matrix::Shape { height, width }
    }

    pub fn transpose(&self) -> Self
    where
        T: Default + Clone,
    {
        let original_shape = self.shape();
        let mut result = DynamicShapedMatrix::new(original_shape.width, original_shape.height);
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

impl<T> std::ops::Index<(usize, usize)> for DynamicShapedMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output { &self.data[index.0][index.1] }
}

impl<T> std::ops::IndexMut<(usize, usize)> for DynamicShapedMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (height, width) = (3, 4);
        let mut matrix = super::DynamicShapedMatrix::<usize>::new(height, width);
        assert_eq!(
            matrix,
            super::DynamicShapedMatrix::<usize>::from(vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ])
        );
        println!("{:?}", matrix);
        matrix[(1, 1)] += 1;
        assert_eq!(
            matrix,
            super::DynamicShapedMatrix::<usize>::from(vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 0, 0],
            ])
        );
        assert_eq!(
            matrix.transpose(),
            super::DynamicShapedMatrix::<usize>::from(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
            ])
        );

        for row in 0..height {
            for col in 0..width {
                matrix[(row, col)] = row * width + col;
            }
        }
        assert_eq!(
            matrix,
            super::DynamicShapedMatrix::<usize>::from(vec![
                vec![0, 1, 2, 3],
                vec![4, 5, 6, 7],
                vec![8, 9, 10, 11],
            ])
        );
        assert_eq!(
            matrix.reverse(),
            super::DynamicShapedMatrix::<usize>::from(vec![
                vec![8, 9, 10, 11],
                vec![4, 5, 6, 7],
                vec![0, 1, 2, 3],
            ])
        );
        assert_eq!(
            matrix.rotate_counterclockwise(),
            super::DynamicShapedMatrix::<usize>::from(vec![
                vec![3, 7, 11],
                vec![2, 6, 10],
                vec![1, 5, 9],
                vec![0, 4, 8],
            ])
        );
        assert_eq!(
            matrix.rotate_clockwise(),
            super::DynamicShapedMatrix::<usize>::from(vec![
                vec![8, 4, 0],
                vec![9, 5, 1],
                vec![10, 6, 2],
                vec![11, 7, 3],
            ])
        );
    }
}
