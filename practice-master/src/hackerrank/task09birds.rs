// https://www.hackerrank.com/challenges/migratory-birds/problem
pub fn migratory_birds(arr: &[i32]) -> i32 {
let mut counts = [0; 6]; 
   
    for &bird in arr {
        counts[bird as usize] += 1;
    }

    (1..6).fold(1, |max_id, id| {
        if counts[id] > counts[max_id] { id } else { max_id }
    }) as i32
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_basic() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_migratory_birds_tie_break() {
        let arr = vec![1, 2, 1, 2, 3];
        assert_eq!(migratory_birds(&arr), 1);
    }

    #[test]
    fn test_migratory_birds_all_same() {
        let arr = vec![5, 5, 5, 5];
        assert_eq!(migratory_birds(&arr), 5);
    }
}