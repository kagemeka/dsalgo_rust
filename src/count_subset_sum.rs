pub fn subset_sum_counting_table<T>(values: &[u64], size: usize) -> Vec<T>
where
    T: std::ops::Add<Output = T> + From<u64> + Clone,
{
    let mut count: Vec<T> = vec![0.into(); size];
    count[0] = 1.into();
    for &v in values {
        let v = v as usize;
        for j in (v..size).rev() {
            count[j] = count[j].clone() + count[j - v].clone();
        }
    }
    count
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
