// https://www.hackerrank.com/challenges/between-two-sets/problem
pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
#[allow(dead_code)]
fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 { 0 } 
    else { (a * b).abs() / gcd(a, b) }
}
#[allow(dead_code)]
#[allow(non_snake_case)]
fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let mut min_lcm = a[0];
    for &val in a.iter().skip(1) {
        min_lcm = lcm(min_lcm, val);
    }
    let mut max_gcd = b[0];
    for &val in b.iter().skip(1) {
        max_gcd = gcd(max_gcd, val);
    }
    let mut count = 0;
    let mut multiple = min_lcm;
    
    while multiple <= max_gcd {
        if max_gcd % multiple == 0 {
            count += 1;
        }
        multiple += min_lcm;
    }

    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(getTotalX(&a, &b), 3);
    }

    #[test]
    fn test1() {
        let a = vec![3, 4];
        let b = vec![24, 48];
        assert_eq!(getTotalX(&a, &b), 2);
    }

    #[test]
    fn test_no_matches() {
        let a = vec![10, 20];
        let b = vec![1, 2];
        assert_eq!(getTotalX(&a, &b), 0);
    }
}