pub fn adjacency_list_to_edges<T>(
    adjacency_list: Vec<Vec<(usize, T)>>,
) -> Vec<(usize, usize, T)> {
    adjacency_list
        .into_iter()
        .enumerate()
        .flat_map(|(from, edges)| {
            edges.into_iter().map(move |(to, data)| (from, to, data))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let g = vec![
            vec![(1, 2), (2, 3)],
            vec![(2, -5), (3, 1)],
            vec![(3, 2)],
            vec![],
        ];
        assert_eq!(
            adjacency_list_to_edges(g),
            vec![
                (0, 1, 2),
                (0, 2, 3),
                (1, 2, -5),
                (1, 3, 1),
                (2, 3, 2)
            ],
        );
    }
}
