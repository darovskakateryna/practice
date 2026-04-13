// https://www.hackerrank.com/challenges/kangaroo/problem
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return String::from("NO");
    }

    if (x2 - x1) % (v1 - v2) == 0 {
        String::from("YES")
    } else {
        String::from("NO")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_meets() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_never_meets() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_same_speed_different_pos() {
        assert_eq!(kangaroo(0, 2, 5, 2), "NO");
    }

    #[test]
    fn test_jump_over() {
        assert_eq!(kangaroo(0, 10, 5, 3), "NO");
    }
}