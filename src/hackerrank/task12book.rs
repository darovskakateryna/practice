//www.hackerrank.com/challenges/drawing-book/problem
#[allow(non_snake_case)]
pub fn pageCount(n: i32, p: i32) -> i32 {
    let front = p / 2;
    
    let back = (n / 2) - front;
    
    if front < back { front } else { back }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case_1() {
        assert_eq!(pageCount(6, 2), 1);
    }

    #[test]
    fn test_example_case_2() {
        assert_eq!(pageCount(5, 4), 0);
    }

    #[test]
    fn test_exact_match_at_end() {
        assert_eq!(pageCount(6, 6), 0);
    }

    #[test]
    fn test_first_page() {
        assert_eq!(pageCount(10, 1), 0);
    }
}