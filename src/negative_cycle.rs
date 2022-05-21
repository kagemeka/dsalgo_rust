#[derive(Debug, PartialEq)]
pub struct NegativeCycleError {
    msg: &'static str,
}

impl NegativeCycleError {
    pub fn new() -> Self { Self { msg: "Negative Cycle Found." } }
}

impl std::fmt::Display for NegativeCycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for NegativeCycleError {
    fn description(&self) -> &str { &self.msg }
}
