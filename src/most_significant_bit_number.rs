/// O(\log\log{N})
pub fn msb_number_by_sieve(mut n: usize) -> usize {
    if n & 0xffffffff00000000 > 0 {
        n &= 0xffffffff00000000;
    }
    if n & 0xffff0000ffff0000 > 0 {
        n &= 0xffff0000ffff0000;
    }
    if n & 0xff00ff00ff00ff00 > 0 {
        n &= 0xff00ff00ff00ff00;
    }
    if n & 0xf0f0f0f0f0f0f0f0 > 0 {
        n &= 0xf0f0f0f0f0f0f0f0;
    }
    if n & 0xcccccccccccccccc > 0 {
        n &= 0xcccccccccccccccc;
    }
    if n & 0xaaaaaaaaaaaaaaaa > 0 {
        n &= 0xaaaaaaaaaaaaaaaa;
    }
    n
}
