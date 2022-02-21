use crate::union_find::UnionFind;

pub fn with_dfs(n: usize, g: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut t: Vec<Vec<usize>> = vec![vec![]; n];
    for (u, v) in g.iter() {
        t[*u].push(*v);
        t[*v].push(*u);
    }
    let mut label = vec![n; n];
    let mut l = 0 as usize;
    fn dfs(
        u: usize,
        label: &mut Vec<usize>,
        l: usize,
        t: &Vec<Vec<usize>>,
    ) -> () {
        label[u] = l;
        for v in t[u].iter() {
            if label[*v] == t.len() {
                dfs(*v, label, l, t);
            }
        }
    }
    for i in 0..n {
        if label[i] != n {
            continue;
        }
        dfs(i, &mut label, l, &t);
        l += 1;
    }
    label
}

pub fn with_bfs(n: usize, g: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut t: Vec<Vec<usize>> = vec![vec![]; n];
    for (u, v) in g.iter() {
        t[*u].push(*v);
        t[*v].push(*u);
    }
    let mut label = vec![n; n];
    let mut l = 0 as usize;
    for i in 0..n {
        if label[i] != n {
            continue;
        }
        label[i] = l;
        let mut que = std::collections::VecDeque::new();
        que.push_back(i);
        while let Some(u) = que.pop_front() {
            for v in t[u].iter() {
                if label[*v] != n {
                    continue;
                }
                label[*v] = l;
                que.push_back(*v);
            }
        }
        l += 1;
    }
    label
}

pub fn with_union_find(n: usize, g: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut label = vec![n; n];
    let mut l = 0 as usize;
    let mut uf = UnionFind::new(n);
    for (u, v) in g {
        uf.unite(*u, *v);
    }
    for i in 0..n {
        let root = uf.find(i);
        if label[root] == n {
            label[root] = l;
            l += 1
        }
        label[i] = label[root];
    }
    label
}
