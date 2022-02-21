pub struct Scanner<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    /// let stdin = std::io::stdin();
    /// let mut sc = Scanner::new(stdin.lock());
    pub fn new(reader: R) -> Self { Self { reader: reader } }

    pub fn scan<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        self.reader.by_ref().bytes().map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>().parse::<T>().ok().unwrap()
    }
}


pub struct StdinScanner<'a> {
    reader: std::io::StdinLock<'a>,
}

impl<'a> StdinScanner<'a> {
    /// let mut sc = StdinScanner::new();
    pub fn new() -> Self {
        let stdin = Box::leak(Box::new(std::io::stdin()));
        Self { reader: stdin.lock() }
    }

    /// let a = sc.scan::<i32>();
    /// let b: i32 = sc.scan();
    pub fn scan<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        self.reader.by_ref().bytes().map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>().parse::<T>().ok().unwrap()
    }
}


pub fn scan<T: std::str::FromStr>() -> T {
    use std::io::Read;
    std::io::stdin().lock().bytes().map(|c| c.unwrap() as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect::<String>().parse::<T>().ok().unwrap()
}


pub fn scan<T: std::str::FromStr>() -> T {
    use std::io::Read;
    std::io::stdin().lock().bytes().map(|c|c.unwrap()as char)
    .skip_while(|c|c.is_whitespace())
    .take_while(|c|!c.is_whitespace())
    .collect::<String>().parse::<T>().ok().unwrap()
}



use std::io::Write;
/// let stdout = std::io::stdout();
/// let out = &mut std::io::BufWriter::new(stdout.lock());
/// writeln!(out, "{}", 1).unwrap()


pub struct Writer<W: std::io::Write> {
    writer: W,
}

impl<W: std::io::Write> Writer<W> {
    pub fn new(writer: W) -> Self { Self { writer: writer } }

}


/// use std::io::Write;
/// let stdin = std::io::stdin();
/// let mut sc = Scanner::new(std::io::BufReader::new(stdin.lock()));
/// let stdout = std::io::stdout();
/// let out = &mut std::io::BufWriter::new(stdout.lock());
