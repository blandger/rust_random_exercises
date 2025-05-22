// Найти наибольший элемент в срезе и вернуть его ссылку
// Условие:
// Реализуй функцию max_ref, которая принимает срез элементов и возвращает ссылку на максимальный элемент.

// Ограничения:
// 
// Нельзя использовать .clone() — работаем строго со ссылками
// 
// Тип T должен реализовывать Ord
// 
// Использовать iter() или iter().max() разрешено
#![allow(dead_code)]
fn max_ref<'a, T: Ord>(items: &'a [T]) -> Option<&'a T> {
    items.iter().max()
}

fn max_ref_more_generic<'a, I, T>(items: I) -> Option<&'a T>
where
    I: IntoIterator<Item = &'a T>,
    T: Ord {
    items.into_iter().max()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_ref_with_numbers() {
        let items = vec![10, 20, 5, 40, 15];
        let result = max_ref(&items);
        assert_eq!(result, Some(&40));
        let result = max_ref_more_generic(&items);
        assert_eq!(result, Some(&40));
    }

    #[test]
    fn test_max_ref_with_strings() {
        let items = vec!["alpha", "omega", "delta"];
        let result = max_ref(&items);
        assert_eq!(result, Some(&"omega"));
        let result = max_ref_more_generic(&items);
        assert_eq!(result, Some(&"omega"));
    }

    #[test]
    fn test_max_ref_empty() {
        let items: Vec<i32> = vec![];
        let result = max_ref(&items);
        assert_eq!(result, None);
        let result = max_ref_more_generic(&items);
        assert_eq!(result, None);
    }
}
