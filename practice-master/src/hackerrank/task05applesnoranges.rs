// https://www.hackerrank.com/challenges/apple-and-orange/problem
pub fn countapplesandoranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter()
        .map(|&d| a + d) 
        .filter(|&pos| pos >= s && pos <= t)
        .count(); 
    let orange_count = oranges.iter()
        .map(|&d| b + d) 
        .filter(|&pos| pos >= s && pos <= t) 
        .count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() { 
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];
        countapplesandoranges(s, t, a, b, &apples, &oranges);
    }
}