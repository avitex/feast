pub struct SliceIter<'a, T: Token> {
    cursor: usize,
    items: &'a [T],
}

impl<'a, T: Token> Iterator for SliceIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len() > 0 {
            let next = self.items[self.cursor].clone();
            self.cursor += 1;
            Some(next)
        } else {
            None
        }
    }
}

impl<'a, T: Token> ExactSizeIterator for SliceIter<'a, T> {
    fn len(&self) -> usize {
        self.items.len() - self.cursor
    }
}