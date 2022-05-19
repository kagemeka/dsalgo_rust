pub fn ford_fulkerson(
    size_a: usize,
    size_b: usize,
    g: &[(usize, usize)],
) -> Vec<Option<usize>> {
    fn dfs(
        g: &[Vec<usize>],
        pair: &mut [Option<usize>],
        visited: &mut [bool],
        u: usize,
    ) -> bool {
        visited[u] = true;
        for &v in g[u].iter() {
            if !pair[v].map_or(true, |v| {
                !visited[v] && dfs(g, pair, visited, v)
            }) {
                continue;
            }
            pair[v] = Some(u);
            pair[u] = Some(v);
            return true;
        }
        false
    }
    let n = size_a + size_b;
    let mut t = vec![vec![]; n];
    for &(u, v) in g.iter() {
        t[u].push(v + size_a);
        t[v + size_a].push(u);
    }
    let mut pair = vec![None; n];
    for i in 0..size_a {
        if pair[i].is_some() {
            continue;
        }
        dfs(
            &t,
            &mut pair,
            &mut vec![false; n],
            i,
        );
    }
    pair.into_iter().take(size_a).collect()
}

pub fn hopcroft_karp(
    size_a: usize,
    size_b: usize,
    g: &[(usize, usize)],
) -> Vec<Option<usize>> {
    let bfs = |g: &[Vec<usize>],
               matched: &[bool],
               pair_a: &[Option<usize>]|
     -> Vec<usize> {
        let mut que = std::collections::VecDeque::new();
        let mut level = vec![std::usize::MAX; size_b];
        for u in 0..size_b {
            if matched[u] {
                continue;
            }
            level[u] = 0;
            que.push_back(u);
        }
        while let Some(u) = que.pop_front() {
            for &v in g[u].iter() {
                if let Some(u2) = pair_a[v] {
                    if level[u2] <= level[u] + 1 {
                        continue;
                    }
                    level[u2] = level[u] + 1;
                    que.push_back(u2);
                }
            }
        }
        level
    };

    fn dfs(
        g: &[Vec<usize>],
        level: &[usize],
        it: &mut [usize],
        pair_a: &mut [Option<usize>],
        matched: &mut [bool],
        u: usize,
    ) -> bool {
        for (i, &v) in g[u].iter().enumerate().skip(it[u]) {
            it[u] = i + 1;
            if !pair_a[v].map_or(true, |u2| {
                level[u2] == level[u] + 1
                    && dfs(
                        g, level, it, pair_a, matched, u2,
                    )
            }) {
                continue;
            }
            pair_a[v] = Some(u);
            matched[u] = true;
            return true;
        }
        false
    }

    let mut t = vec![vec![]; size_b];
    for &(v, u) in g.iter() {
        t[u].push(v);
    } // v \in A, u \in B.
    let mut matched = vec![false; size_b];
    let mut pair_a = vec![None; size_a];

    loop {
        let level = bfs(&t, &matched, &pair_a);
        let mut it = vec![0; size_b];
        let mut updated = false;
        for u in 0..size_b {
            if !matched[u] {
                updated |= dfs(
                    &t,
                    &level,
                    &mut it,
                    &mut pair_a,
                    &mut matched,
                    u,
                );
            }
        }
        if !updated {
            break;
        }
    }
    pair_a
}

pub fn blossom() {}
