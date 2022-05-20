// TODO: this code is stub yet;
pub trait PriorityQueue<T>
where
    T: PartialOrd,
{
    fn push(&mut self, x: T);
    fn pop(&mut self) -> Option<T>;
}
