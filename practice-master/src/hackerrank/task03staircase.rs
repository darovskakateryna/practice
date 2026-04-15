// https://www.hackerrank.com/challenges/staircase/problem
pub fn staircase(n: i32) {
    let n = n as usize;
    for i in 1..=n {
        let spaces = n - i;
        println!("{:indent$}{:=>count$}", "", "", indent = spaces, count = i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        staircase(3);
    }
}