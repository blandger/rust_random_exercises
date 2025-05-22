#![allow(dead_code)]

// найти все строки, содержащие подстроку

// Условия:
// items: слайс ссылок на строки (&[&str])
// 
// needle: подстрока, которую мы ищем
// 
// вернуть нужно все строки, содержащие needle
// 
// никаких клонирований — работаем со ссылками
// 
// должна быть безопасна по лайфтаймам (не использовать .to_string() и т.п.)
fn find_all_containing<'a>(items: &'a [&'a str], needle: &str) -> Vec<&'a str>
{
    items.iter()
        .filter(|s| s.contains(needle))
        .map(|s| *s)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_containing() {
        let data = ["rustacean", "crab", "trust", "safe", "fast"];
        let matches = find_all_containing(&data, "st");
        assert_eq!(matches, vec!["rustacean", "trust", "fast"]);
    }
}