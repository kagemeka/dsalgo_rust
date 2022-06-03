#[derive(Debug, PartialEq)]
pub struct DisconnectedError {
    msg: &'static str,
}

impl DisconnectedError {
    pub fn new() -> Self {
        Self {
            msg: "given graph is not connected.",
        }
    }
}

impl std::fmt::Display for DisconnectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for DisconnectedError {
    fn description(&self) -> &str { &self.msg }
}
