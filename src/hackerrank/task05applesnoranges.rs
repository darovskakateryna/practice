// https://www.hackerrank.com/challenges/apple-and-orange/problem
pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let count = |pos: i32, items: &[i32]| {
        items.iter().filter(|&&d| (s..=t).contains(&(pos + d))).count()
    };

    println!("{}", count(a, apples));
    println!("{}", count(b, oranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apples = [2, 3, -4];
        let oranges = [3, -2, -4];
        count_apples_and_oranges(s, t, a, b, &apples, &oranges);
    }
}
