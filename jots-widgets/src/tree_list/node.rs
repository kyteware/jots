pub struct Node<T> {
    pub label: T,
    pub folder: bool,
    pub depth: usize,
}