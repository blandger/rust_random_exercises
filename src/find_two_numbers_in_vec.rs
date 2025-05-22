#![allow(dead_code)]
use std::collections::HashMap;

// Дано: вектор целых чисел nums и целое число target.
// Нужно вернуть индексы двух чисел, сумма которых равна target.
// Гарантируется, что ровно одно такое решение существует.
// Верни результат как кортеж из двух usize.

const INPUT_ARRAY: [i32; 7] = [2, 3, 5, 7, 11, 15, 20];

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen_element_map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen_element_map.get(&complement) {
            return Some((j, i));
        }
        seen_element_map.insert(num, i);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum_1() {
        let target = 12;
        let result = two_sum(&INPUT_ARRAY, target);
        println!("{result:?} = for '{target}'");
        assert_eq!(Some((2, 3)), result);
    }
    #[test]
    fn test_two_sum_2() {
        let target = 18;
        let result = two_sum(&INPUT_ARRAY, target);
        println!("{result:?} = for '{target}'");
        assert_eq!(Some((3, 4)), result);
    }
    #[test]
    fn test_two_sum_3() {
        let target = 22;
        let result = two_sum(&INPUT_ARRAY, target);
        println!("{result:?} = for '{target}'");
        assert_eq!(Some((3, 5)), result);
    }
    #[test]
    fn edge_case_1() {
        let nums = vec![3, 3];
        let target = 6;
        // ожидается: Some((0, 1)) или (1, 0)
        let result = two_sum(&nums, target);
        println!("{result:?} = for '{target}'");
        assert_eq!(Some((0, 1)), result);
    }
    #[test]
    fn edge_case_2() {
        let nums = vec![-2, -3, 7, 5];
        let target = 2;
        // ожидается: Some((1, 3))
        let result = two_sum(&nums, target);
        println!("{result:?} = for '{target}'");
        assert_eq!(Some((1, 3)), result);
    }
    #[test]
    fn edge_case_3() {
        let nums = vec![0, 4, 0];
        let target = 0;
        // ожидается: Some((0, 2))
        let result = two_sum(&nums, target);
        println!("{result:?} = for '{target}'");
        assert_eq!(Some((0, 2)), result);
    }
    #[test]
    fn edge_case_4() {
        let nums = vec![5];
        let target = 10;
        // ожидается: None
        let result = two_sum(&nums, target);
        println!("{result:?} = for '{target}'");
        assert_eq!(None, result);
    }
    #[test]
    fn edge_case_5() {
        let nums = vec![1, 1, 1, 1, 1];
        let target = 2;
        // ожидается: Some((0, 1)) или любые разные индексы
        let result = two_sum(&nums, target);
        println!("{result:?} = for '{target}'");
        assert_eq!(Some((0, 1)), result);
    }

}

/*pub fn main() {
    let target = 9;
    let result = two_sum(&INPUT_ARRAY, target);
    println!("{result:?} = for '{target}'");
}*/
