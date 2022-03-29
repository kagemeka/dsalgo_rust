use crate::abstract_structs::Monoid;

/// Monoid<S> is Commutative.
/// map: F x S -> S is homomorphism.
pub fn rerooting<S: Clone, F>(
    g: &Vec<(usize, usize, F)>,
    m: &Monoid<S>,
    map: Box<dyn Fn(&F, &S) -> S>,
) -> Vec<S> {
    fn tree_dp<S, F>(
        g: &Vec<Vec<(usize, &F)>>,
        dp: &mut Vec<S>,
        m: &Monoid<S>,
        map: &Box<dyn Fn(&F, &S) -> S>,
        u: usize,
        parent: usize,
    ) {
        for &(v, x) in g[u].iter() {
            if v == parent {
                continue;
            }
            tree_dp(g, dp, m, map, v, u);
            dp[u] = (m.op)(&dp[u], &map(x, &dp[v]));
        }
    }
    fn reroot<S: Clone, F>(
        g: &Vec<Vec<(usize, &F)>>,
        dp0: &Vec<S>,
        dp1: &mut Vec<S>,
        m: &Monoid<S>,
        map: &Box<dyn Fn(&F, &S) -> S>,
        u: usize,
        parent: usize,
    ) {
        let mut childs = Vec::new();
        for &e in g[u].iter() {
            if e.0 != parent {
                childs.push(e);
            }
        }
        let deg = childs.len();
        let mut dp_l = vec![(m.e)(); deg + 1];
        let mut dp_r = vec![(m.e)(); deg + 1];
        for (i, &(v, x)) in childs.iter().enumerate() {
            dp_l[i + 1] = (m.op)(&dp_l[i], &map(x, &dp0[v]));
        }
        for (i, &(v, x)) in childs.iter().enumerate().rev() {
            dp_r[i] = (m.op)(&dp_r[i + 1], &map(x, &dp0[v]));
        }
        for (i, &(v, x)) in childs.iter().enumerate() {
            dp1[v] = map(x, &(m.op)(&dp1[u], &(m.op)(&dp_l[i], &dp_r[i + 1])));
            reroot(g, dp0, dp1, m, map, v, u);
        }
    }
    assert_eq!(m.commutative, true);
    let n = g.len() + 1;
    let mut t = vec![vec![]; n];
    for (u, v, x) in g.iter() {
        t[*u].push((*v, x));
        t[*v].push((*u, x));
    }
    let mut dp0: Vec<S> = vec![(m.e)(); n];
    let mut dp1: Vec<S> = vec![(m.e)(); n];
    tree_dp(&t, &mut dp0, m, &map, 0, n);
    reroot(&t, &dp0, &mut dp1, m, &map, 0, n);
    let mut dp = vec![(m.e)(); n];
    for i in 0..n {
        dp[i] = (m.op)(&dp0[i], &dp1[i]);
    }
    dp
}
