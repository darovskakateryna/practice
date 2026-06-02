// https://www.hackerrank.com/challenges/grading/problem
pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&g| match g {
        g if g >= 38 && g % 5 >= 3 => g + (5 - g % 5),
        _ => g,
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }
}
