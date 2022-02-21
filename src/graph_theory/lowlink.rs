pub fn lowlink(g: &Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>) {
    let n = g.len();
    let mut order = vec![n; n];
    let mut low = vec![n; n];
    fn dfs(
        g: &Vec<Vec<usize>>,
        u: usize,
        parent: usize,
        ord: &mut usize,
        order: &mut Vec<usize>,
        low: &mut Vec<usize>,
    ) {
        let n = g.len();
        order[u] = *ord;
        *ord += 1;
        low[u] = order[u];
        for v in g[u].iter().map(|x| *x) {
            if v == parent {
                continue;
            }
            if order[v] != n {
                if order[v] < low[u] {
                    low[u] = order[v];
                }
                continue;
            }
            dfs(g, v, u, ord, order, low);
            if low[v] < low[u] {
                low[u] = low[v];
            }
        }
    }
    let mut ord = 0;
    for i in 0..n {
        if order[i] != n {
            continue;
        }
        dfs(g, i, n, &mut ord, &mut order, &mut low);
    }
    (order, low)
}

pub fn bridges(n: usize, g: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut t = vec![vec![]; n];
    for (u, v) in g {
        t[*u].push(*v);
        t[*v].push(*u);
    }
    let (order, low) = lowlink(&t);
    let mut bridges = Vec::new();
    for (u, v) in g {
        let mut u = *u;
        let mut v = *v;
        if order[u] > order[v] {
            std::mem::swap(&mut u, &mut v);
        }
        if low[v] <= order[u] {
            continue;
        }
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        bridges.push((u, v));
    }
    bridges.sort();
    bridges
}

pub fn articulation_points(g: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = g.len();
    let mut articulation_points = Vec::new();
    let (order, low) = lowlink(g);
    let mut visited = vec![false; n];
    fn dfs(
        g: &Vec<Vec<usize>>,
        u: usize,
        parent: usize,
        visited: &mut Vec<bool>,
        order: &Vec<usize>,
        low: &Vec<usize>,
        points: &mut Vec<usize>,
    ) {
        let n = g.len();
        visited[u] = true;
        let mut cnt = 0u32;
        let mut is_articulation = false;
        for v in g[u].iter().map(|x| *x) {
            if visited[v] {
                continue;
            }
            cnt += 1;
            dfs(g, v, u, visited, order, low, points);
            if parent == n || low[v] < order[u] {
                continue;
            }
            is_articulation = true;
        }
        if parent == n && cnt >= 2 {
            is_articulation = true;
        }
        if is_articulation {
            points.push(u);
        }
    }
    for i in 0..n {
        if visited[i] {
            continue;
        }
        dfs(g, i, n, &mut visited, &order, &low, &mut articulation_points);
    }
    articulation_points.sort();
    articulation_points
}
