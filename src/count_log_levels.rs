// У тебя есть вектор строк — лог-файлы, которые выглядят вот так:

// "INFO User logged in: user_id=42"
// "ERROR Disk is full"
// "INFO User logged out: user_id=42"
// "WARNING High memory usage"

// Написать функцию
// fn count_log_levels(logs: &[String]) -> HashMap<String, usize>

// Функция должна вернуть HashMap, в котором ключ — это уровень логирования ("INFO", "ERROR", "WARNING"),
// а значение — количество строк в логе с этим уровнем.Функция должна вернуть HashMap, в котором ключ — это уровень логирования ("INFO", "ERROR", "WARNING"),
// а значение — количество строк в логе с этим уровнем.

use std::collections::HashMap;

fn count_log_levels(logs: &[String]) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    for log in logs {
        let log_level = log.split_whitespace().next().unwrap();
        let count = result.entry(log_level.to_string()).or_insert(0);
        *count += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        let logs = vec![
            "INFO User logged in: user_id=42".to_string(),
            "ERROR Disk is full".to_string(),
            "INFO User logged out: user_id=42".to_string(),
            "WARNING High memory usage".to_string(),
        ];
        let result = count_log_levels(&logs);

        // Должно получиться:
        assert_eq!(result.get("INFO"), Some(&2));
        assert_eq!(result.get("ERROR"), Some(&1));
        assert_eq!(result.get("WARNING"), Some(&1));
    }
}
