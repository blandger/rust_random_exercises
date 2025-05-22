// Проверка сбалансированных скобок
// Реализуй функцию is_valid_parentheses, которая принимает строку, содержащую только символы '(', ')', '{', '}', '[', ']' и проверяет, является ли скобочная последовательность корректной.

// Правила:
// Каждая открывающая скобка должна быть закрыта тем же типом скобки.
//
// Скобки должны быть закрыты в правильном порядке.
//
// Каждая закрывающая скобка должна иметь соответствующую открывающую.

fn is_valid_parentheses(s: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in s.chars().filter(|c| !c.is_whitespace()) {
        // println!("{c}");
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => unreachable!(), // safeguard
        }
    }
    // println!("{stack:?}");
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(is_valid_parentheses("()".to_string()), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_valid_parentheses("()[]{}".to_string()), true);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_valid_parentheses("(]".to_string()), false);
    }
    #[test]
    fn test_4() {
        assert_eq!(is_valid_parentheses("([)]".to_string()), false);
    }
    #[test]
    fn test_5() {
        assert_eq!(is_valid_parentheses("{()[]}{}".to_string()), true);
    }
    #[test]
    fn test_6() {
        assert_eq!(is_valid_parentheses("{()[]}({})".to_string()), true);
    }
}
