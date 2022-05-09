fn read<R, T>(reader: &mut R) -> Result<T, <T as std::str::FromStr>::Err>
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

pub struct ReadWrapper<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> ReadWrapper<R> {
    /// let stdin = std::io::stdin();
    /// let mut reader = ReadWrapper::new(std::io::BufReader::new(stdin.lock()));
    /// let x = reader.read::<usize>();
    pub fn new(reader: R) -> Self { Self { reader: reader } }

    pub fn read<T: std::str::FromStr>(&mut self) -> Result<T, <T as std::str::FromStr>::Err> {
        read(&mut self.reader)
    }
}

pub fn locked_buf_reader() -> ReadWrapper<std::io::BufReader<std::io::StdinLock<'static>>> {
    let stdin = Box::leak(Box::new(std::io::stdin()));
    ReadWrapper::new(std::io::BufReader::new(stdin.lock()))
}

/// Example
/// ```
/// use std::io::Write;
/// let writer = &mut locked_buf_writer();
/// writeln!(writer, "Hello, world!");
/// writer.flush().unwrap();
/// ```
pub fn locked_buf_writer() -> std::io::BufWriter<std::io::StdoutLock<'static>> {
    let stdout = Box::leak(Box::new(std::io::stdout()));
    std::io::BufWriter::new(stdout.lock())
}

pub fn fast_io() -> (
    ReadWrapper<std::io::BufReader<std::io::StdinLock<'static>>>,
    std::io::BufWriter<std::io::StdoutLock<'static>>,
) {
    (locked_buf_reader(), locked_buf_writer())
}
