// pub trait OperationIdentifier {}

// pub struct OperationId;

// impl OperationIdentifier for OperationId {}

pub trait Mapping<S, F> {
    fn map(operator: &F, element: &S) -> S;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
