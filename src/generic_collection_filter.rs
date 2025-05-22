// Фильтрация по условию
// Напиши обобщённую функцию:
// fn filter_with<F, T>(items: &[T], predicate: F) -> Vec<T>
// where
//     F: Fn(&T) -> bool,
//     T: Clone,

// Функция должна возвращать новый Vec<T>, содержащий только те элементы, которые удовлетворяют предикату.

fn filter_with<F, T>(items: &[T], predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
    T: Clone,
{
    /*let mut result: Vec<T> = Vec::new();
    for item in items {
        if predicate(item) {
            result.push(item.clone());
        }
    }
    result*/
    items.iter().filter(|item| predicate(item)).cloned().collect()
}

fn _filter_with_owned<F, T>(items: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    items.into_iter().filter(|x| predicate(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use() {
        let items = vec![1, 2, 3, 4, 5, 6];
        let even = filter_with(&items, |x| x % 2 == 0);
        assert_eq!(even, vec![2, 4, 6]);
    }
}
