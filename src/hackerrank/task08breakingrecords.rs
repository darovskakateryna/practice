// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() { return vec![0, 0]; }

    let init = (scores[0], scores[0], 0, 0); 
    let (_, _, max_breaks, min_breaks) = scores.iter().skip(1)
        .fold(init, |(max, min, max_c, min_c), &s| {
            if s > max { (s, min, max_c + 1, min_c) }
            else if s < min { (max, s, max_c, min_c + 1) }
            else { (max, min, max_c, min_c) }
        });

    vec![max_breaks, min_breaks]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records_standard() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&scores), vec![2, 4]);
    }

    #[test]
    fn test_breaking_records_another_case() {
        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records(&scores), vec![4, 0]);
    }

    #[test]
    fn test_empty_scores() {
        let scores: Vec<i32> = vec![];
        assert_eq!(breaking_records(&scores), vec![0, 0]);
    }

    #[test]
    fn test_single_score() {
        let scores = vec![10];
        assert_eq!(breaking_records(&scores), vec![0, 0]);
    }
}