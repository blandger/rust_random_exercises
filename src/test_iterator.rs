// Написать простой итератор для вектора

trait MyIterator<T> {
    type Output;
    fn next(&mut self) -> Option<Self::Output>;
}

struct MyVec<T> {
    data: Vec<T>,
    pos: usize,
}
impl<T> MyVec<T> {
    fn new(data: Vec<T>) -> Self {
        Self { data, pos: 0 }
    }
}

impl<T> MyIterator<T> for MyVec<T>
where
    T: Clone,
{
    type Output = T;

    fn next(&mut self) -> Option<Self::Output> {
        if self.pos < self.data.len() {
            let item = self.data[self.pos].clone(); // или можно взять через `remove(pos)` если без клонирования
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_my_iterator() {
        let data = vec![10, 20, 30];
        let mut iter = MyVec::new(data);

        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), None);
    }
}
