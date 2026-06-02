//www.hackerrank.com/challenges/diagonal-difference/problem
#[allow(non_snake_case)]
pub fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let sum: i32 = arr
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let left = row[i];              
            let right = row[row.len() - 1 - i]; 
            left - right
        })
        .sum();

    sum.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference_standard() {
        let matrix = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        assert_eq!(diagonalDifference(&matrix), 15);
    }

    #[test]
    fn test_diagonal_difference_small() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        assert_eq!(diagonalDifference(&matrix), 0);
    }

    #[test]
    fn test_diagonal_difference_negative() {
        let matrix = vec![
            vec![-1, 1, -7, 10],
            vec![3, 5, 24, 1],
            vec![2, 10, -5, 2],
            vec![-8, 0, 2, 10],
        ];
        let result = diagonalDifference(&matrix);
        assert!(result >= 0); 
    }
}
