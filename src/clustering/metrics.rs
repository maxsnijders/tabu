/// Computes the diameter of a vector of items, defined as the largest pairwise distance between elements in the vector
pub fn diameter<Item>(items: &Vec<Item>, distance: impl Fn(&Item, &Item) -> f64) -> f64 {
    let mut diameter = 0.0;

    for i in 0..items.len() {
        for j in i + 1..items.len() {
            let distance = distance(&items[i], &items[j]);
            if distance > diameter {
                diameter = distance;
            }
        }
    }

    diameter
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_diameter_of_integer_vector() {
        let test_vec = vec![1, 2, 3, 4];
        
        assert_eq!(diameter(test_vec, |x, y| (x - y).abs()), 3);
    }
}
