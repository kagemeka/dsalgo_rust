use crate::vector::compress_array;


/// O(N|log^2{N})
pub fn sa_doubling(a: &Vec<usize>) -> Vec<usize> {
    let n = a.len();
    let (mut rank, _) = compress_array(&a);
    let mut k = 1usize;
    let mut key = vec![0; n];
    let mut sa = vec![0; n];
    loop {
        for i in 0..n { 
            key[i] = rank[i] << 30;
            if i + k < n { key[i] |= 1 + rank[i + k]; }
            sa[i] = i;
        }
        sa.sort_by_key(|&x| key[x]);
        rank[sa[0]] = 0;
        for i in 0..n - 1 { 
            rank[sa[i + 1]] = rank[sa[i]];
            if key[sa[i + 1]] > key[sa[i]] { rank[sa[i + 1]] += 1; }
        }        
        k <<= 1;
        if k >= n { break; }
    }
    sa
}
  

/// O(N\log{N})
pub fn sa_doubling_with_countsort(a: &Vec<usize>) -> Vec<usize> {
    let n = a.len();
    let counting_sort_key = |a: &Vec<usize>| -> Vec<usize> {
        let mut cnt = vec![0; n + 2];
        for &x in a.iter() { cnt[x + 1] += 1; }
        let mut key = vec![0; n];
        for i in 0..n { cnt[i + 1] += cnt[i]; }
        for i in 0..n { 
            key[cnt[a[i]]] = i;
            cnt[a[i]] += 1;
        }
        key
    };
    let (mut rank, _) = compress_array(&a);
    let mut k = 1usize;
    let mut key = vec![0; n];
    let mut first = vec![0; n];
    let mut second = vec![0; n];
    let mut sa = vec![0; n];
    loop {
        for i in 0..n { second[i] = if i + k < n { 1 + rank[i + k] } else { 0 }; }
        let rank_second = counting_sort_key(&second);
        for i in 0..n { first[i] = rank[rank_second[i]]; }
        let rank_first = counting_sort_key(&first);
        for i in 0..n { sa[i] = rank_second[rank_first[i]]; }
        for i in 0..n { key[i] = first[rank_first[i]] << 30 | second[sa[i]]; }
        rank[sa[0]] = 0;
        for i in 0..n - 1 {
            rank[sa[i + 1]] = rank[sa[i]];
            if key[i + 1] > key[i] { rank[sa[i + 1]] += 1; }
        }

        k <<= 1;
        if k >= n { break; }
    }
    sa
}



/// suffix array with induced sort.
/// O(N)
/// references
/// - https://mametter.hatenablog.com/entry/20180130/p1
pub fn sa_is(a: &Vec<usize>) -> Vec<usize> {
    assert!(a.len() > 0);
    let mn = *a.iter().min().unwrap();
    let mut a = a.iter().map(|x| x - mn + 1).collect::<Vec<usize>>();
    a.push(0);
    let n = a.len();
    let m = a.iter().max().unwrap() + 1;
    let mut is_s = vec![true; n];
    let mut is_lms = vec![false; n];
    let mut lms = Vec::with_capacity(n);
    for i in (1..n).rev() {
        is_s[i - 1] = if a[i - 1] == a[i] { is_s[i] } else { a[i - 1] < a[i] };
        is_lms[i] = !is_s[i - 1] && is_s[i];
        if is_lms[i] { lms.push(i); }
    }
    lms.reverse();
    let mut bucket = vec![0usize; m];
    for &x in a.iter() { bucket[x] += 1; }

    let induce = |lms: &Vec<usize>| -> Vec<usize> {
        let mut sa = vec![n; n];
        let mut sa_idx = bucket.clone();
        
        for i in 0..m - 1 { sa_idx[i + 1] += sa_idx[i]; }
        for &i in lms.iter().rev() { 
            sa_idx[a[i]] -= 1;
            sa[sa_idx[a[i]]] = i;
        }

        sa_idx = bucket.clone();
        let mut s = 0usize;
        for i in 0..m { sa_idx[i] += s; std::mem::swap(&mut s, &mut sa_idx[i]); }
        for i in 0..n {
            if sa[i] == n || sa[i] == 0 { continue; } 
            let i = sa[i] - 1;
            if !is_s[i] { sa[sa_idx[a[i]]] = i; sa_idx[a[i]] += 1; }
        }
        
        sa_idx = bucket.clone();
        for i in 0..m - 1 { sa_idx[i + 1] += sa_idx[i]; }
        for i in (0..n).rev() {
            if sa[i] == n || sa[i] == 0 { continue; }
            let i = sa[i] - 1;
            if is_s[i] { sa_idx[a[i]] -= 1; sa[sa_idx[a[i]]] = i; }
        }
        sa
    };

    let sa = induce(&lms);
    let mut lms_idx = Vec::with_capacity(n);
    let mut rank = vec![n; n];
    for &i in sa.iter() { if is_lms[i] { lms_idx.push(i); }; }
    let l = lms_idx.len();
    let mut r = 0usize;
    rank[n - 1] = r;
    for i in 0..l - 1 {
        let j = lms_idx[i];
        let k = lms_idx[i + 1];
        for d in 0..n {
            let j_is_lms = is_lms[j + d];
            let k_is_lms = is_lms[k + d];
            if a[j + d] != a[k + d] || j_is_lms ^ k_is_lms { r += 1; break; }
            if d > 0 && j_is_lms | k_is_lms { break; }
        } 
        rank[k] = r;
    }
    rank = rank.into_iter().filter(|&x| x != n).collect();
    let mut lms_order: Vec<usize> = Vec::new();
    if r == l - 1 { 
        lms_order.resize(l, n);
        for i in 0..l { lms_order[rank[i]] = i; }
    } else {
        lms_order = sa_is(&rank);
    }
    lms = lms_order.iter().map(|&i| lms[i]).collect();
    let sa = induce(&lms);
    sa[1..].to_vec()
} 



#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_sa_is() {
        let s = vec![1, 1, 0, 0, 3, 3, 0, 0, 3, 3, 0, 0, 2, 2, 0, 0];
        let sa = sa_is(&s);    
        assert_eq!(
            sa,
            vec![15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4],
        );
    }

    #[test]
    fn test_sa_doubling_with_countsort() {
        let s = vec![1, 1, 0, 0, 3, 3, 0, 0, 3, 3, 0, 0, 2, 2, 0, 0];
        let sa = sa_doubling_with_countsort(&s);    
        assert_eq!(
            sa,
            vec![15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4],
        );
    }

    #[test]
    fn test_sa_doubling() {
        let s = vec![1, 1, 0, 0, 3, 3, 0, 0, 3, 3, 0, 0, 2, 2, 0, 0];
        let sa = sa_doubling(&s);    
        assert_eq!(
            sa,
            vec![15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4],
        );
    }
}