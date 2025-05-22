// Найти элемент по условию и вернуть ссылку
// 

fn find_first<'a, T, F>(items: &'a [T], predicate: F) -> Option<&'a T>
where
    F: Fn(&T) -> bool,
{
    items.iter().find(|item| predicate(*item))
}


fn find_first_mut<'a, T, F>(items: &'a mut [T], predicate: F) -> Option<&'a mut T>
where
    F: Fn(&T) -> bool,
{
    items.iter_mut().find(|item| predicate(&**item))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first() {
        let items = vec![1, 2, 3, 4, 5];
        let result = find_first(&items, |x| *x > 3);
        assert_eq!(result, Some(&4));
    }
    #[test]
    fn test_mut() {
        let mut items = vec![1, 2, 3, 4, 5];
        let result = find_first_mut(&mut items, |x| *x > 3);
        assert_eq!(result, Some(&mut 4));
        assert_eq!(*result.unwrap(), 4);
    }
}