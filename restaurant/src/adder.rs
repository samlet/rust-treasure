/// 将给定的数字加一
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    // $ cargo test add
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    // $ cargo test hundred
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // 测试私有函数
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}