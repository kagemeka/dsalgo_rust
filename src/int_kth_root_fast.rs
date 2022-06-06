use crate::{
    int_kth_root_binary_search::int_kth_root_binary_search,
    int_kth_root_linear::int_kth_root_linear,
};

pub fn int_kth_root_fast(n: u64, k: u8) -> u64 {
    if k >= 10 {
        int_kth_root_linear(n, k)
    } else {
        int_kth_root_binary_search(n, k)
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
