// TODO: refactor

#[derive(Debug)]
pub struct LowlinkResult {
    pub orders: Vec<usize>,
    pub low_orders: Vec<usize>,
}

impl LowlinkResult {
    pub fn get_lowlinks(data: &LowlinkResult) -> Vec<usize> {
        let n = data.orders.len();
        let mut vertices = vec![0; n];
        for i in 0..n {
            vertices[data.orders[i]] = i;
        }
        data.low_orders.iter().map(|&i| vertices[i]).collect()
    }
}
