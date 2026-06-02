// https://www.hackerrank.com/challenges/sock-merchant/problem
use std::collections::HashSet;

#[allow(non_snake_case)]
pub fn sockMerchant(_n: i32, ar: &[i32]) -> i32 {
    let mut unmatched = HashSet::new();
    let mut pairs = 0;

    for &sock in ar {
        if !unmatched.insert(sock) {
            unmatched.remove(&sock);
            pairs += 1;
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant_logic() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sockMerchant(9, &ar), 3);
    }

    #[test]
    fn test_no_pairs() {
        let ar = vec![1, 2, 3, 4, 5];
        assert_eq!(sockMerchant(5, &ar), 0);
    }
}