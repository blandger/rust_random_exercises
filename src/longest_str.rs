// Вернуть строку с максимальной длиной
// Напиши функцию longest_str<'a>(a: &'a str, b: &'a str) -> &'a str,
// которая возвращает ссылку на самую длинную из двух строк.

//Требования:
// Функция должна возвращать &str, ссылающийся на один из аргументов (в зависимости от длины).
//
// Нельзя делать .to_string() или клонировать — только работа со ссылками.
//
// Функция должна скомпилироваться и пройти тест.

fn longest_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
struct StrPair<'a> {
    a: &'a str,
    b: &'a str,
}
impl<'a> StrPair<'a> {
    pub fn new(a: &'a str, b: &'a str) -> StrPair<'a> {
        StrPair { a, b }
    }
    pub fn longest(&self) -> &str {
        if self.a.len() > self.b.len() {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest() {
        let s1 = String::from("short");
        let s2 = String::from("longer string");
        let result = longest_str(&s1, &s2);
        assert_eq!(result, "longer string");
    }
    #[test]
    fn test_longest_struct() {
        let longest = StrPair::new("short", "longer string");
        let result = longest.longest();
        assert_eq!(result, "longer string");
    }
}
