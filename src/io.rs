fn read_token<R, T>(reader: &mut R) -> Result<T, <T as std::str::FromStr>::Err>
where
    R: std::io::Read,
    T: std::str::FromStr,
{
    use std::io::Read;
    reader
        .by_ref()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<T>()
}

pub fn read_stdin<T>() -> Result<T, <T as std::str::FromStr>::Err>
where
    T: std::str::FromStr,
{
    read_token(&mut std::io::stdin().lock())
}

pub struct ReadWrapper<R: std::io::BufRead> {
    reader: R,
    tokens: Vec<String>,
}

/// Example
/// ```
/// use dsalgo::io::ReadWrapper;
/// let stdin = std::io::stdin();
/// let mut reader = ReadWrapper::new(stdin.lock());
/// // let x = reader.read::<usize>();
/// ```
impl<R: std::io::BufRead> ReadWrapper<R> {
    pub fn new(reader: R) -> Self { Self { reader, tokens: vec![] } }

    pub fn read<T: std::str::FromStr>(&mut self) -> Result<T, <T as std::str::FromStr>::Err> {
        while self.tokens.is_empty() {
            let mut buf = String::new();
            self.reader.read_line(&mut buf).unwrap();
            self.tokens = buf.split_whitespace().map(str::to_string).rev().collect();
        }
        self.tokens.pop().unwrap().parse::<T>()
    }
}

pub fn locked_stdin_reader() -> ReadWrapper<std::io::StdinLock<'static>> {
    let stdin = Box::leak(Box::new(std::io::stdin()));
    ReadWrapper::new(stdin.lock())
}

/// Example
/// ```
/// use std::io::Write;
///
/// use dsalgo::io::locked_stdin_buf_writer;
/// let mut writer = locked_stdin_buf_writer();
/// writeln!(writer, "Hello, world!");
/// writer.flush().unwrap();
/// ```
pub fn locked_stdin_buf_writer() -> std::io::BufWriter<std::io::StdoutLock<'static>> {
    let stdout = Box::leak(Box::new(std::io::stdout()));
    std::io::BufWriter::new(stdout.lock())
}
