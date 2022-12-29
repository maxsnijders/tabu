/// Computes the diameter of a vector of items, defined as the largest pairwise distance between elements in the vector
/// # Arguments
/// * `items`: the items to compute the diameter for
/// * `distance`: a callable computing the distance between any two items
pub fn diameter<Item, T>(items: &Vec<Item>, distance: impl Fn(&Item, &Item) -> T) -> Option<T>
where
    T: PartialOrd + Copy,
{
    let mut diameter: Option<T> = None;

    for i in 0..items.len() {
        for j in i + 1..items.len() {
            let distance = distance(&items[i], &items[j]);
            if let Some(current_diameter) = diameter {
                if distance > current_diameter {
                    diameter = Some(distance);
                }
            } else {
                diameter = Some(distance);
            }
        }
    }

    diameter
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_diameter_of_integer_vector_with_abs_cost() {
        let test_vec: Vec<i32> = vec![1, 2, 3, 4];

        assert_eq!(diameter(&test_vec, |x, y| (x - y).abs()).unwrap(), 3);
    }

    #[test]
    fn test_diameter_of_float_vector_with_quadratic_cost() {
        let test_vec: Vec<OrderedFloat<f64>> = vec![1, 2, 3, 4]
            .into_iter()
            .map(|x| OrderedFloat(x as f64))
            .collect();

        assert_eq!(
            diameter(&test_vec, |&x, &y| (x - y) * (x - y)).unwrap(),
            OrderedFloat(9.0)
        );
    }
}
