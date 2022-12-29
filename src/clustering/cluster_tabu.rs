use crate::search::tabu_search;
use core::hash::Hash;

/// Runs a tabu-search based clustering
/// # Arguments
/// * `items`: the items to cluster
/// * `cost`: a function that returns the cost of a possible clustering
/// * `n_clusters`: the number of clusters to identify
/// * `max_iterations`: the number of iterations (at most) to search for
/// * `stopping_cost`: if `Some(x)`, we stop the search when the cost of the current state no longer exceeds `x`.
pub fn cluster_tabu<Item>(
    items: Vec<Item>,
    cost: impl Fn(&Vec<Vec<Item>>) -> f64,
    n_clusters: usize,
    max_iterations: usize,
    stopping_cost: Option<f64>,
) -> Vec<Vec<Item>>
where
    Item: Clone + Eq + Hash + PartialEq,
{
    if n_clusters == 1 {
        return vec![items];
    }

    #[derive(Clone, Eq, Hash, PartialEq)]
    struct State<Item> {
        clusters: Vec<Vec<Item>>,
    }

    fn descendants<Item>(state: &State<Item>) -> impl Iterator<Item = State<Item>>
    where
        Item: Clone + PartialEq,
    {
        // All descendants of a state are obtained by moving one item from one cluster to another.
        // This is done by iterating over all pairs of clusters and all items in the first cluster.
        // For each pair, we create a new state where the item is moved from the first cluster to the second.

        // We collect the descendants into a vector because we need to return an iterator.
        let mut descendants = Vec::new();

        // Look at each cluster...
        for (i, cluster) in state.clusters.iter().enumerate() {
            // and each item in the cluster...
            for item in cluster {
                // then look at each *other* cluster, to see if we can move the item to that cluster.
                for j in 0..state.clusters.len() {
                    // If the clusters are the same, skip this pair.
                    if i == j {
                        continue;
                    }

                    // Execute the move of item from cluster i to cluster j.
                    let mut new_clusters = state.clusters.clone();
                    new_clusters[i].retain(|x| x != item);
                    new_clusters[j].push(item.clone());

                    // Store the new state as a descendant.
                    descendants.push(State {
                        clusters: new_clusters,
                    });
                }
            }
        }

        // Return the iterator over the descendants.
        descendants.into_iter()
    }

    // Create an initial state, in which all items are in the first cluster.
    let initial_state = State {
        clusters: vec![items]
            .into_iter()
            .chain(vec![vec![]; n_clusters - 1])
            .collect(),
    };

    let best_state = tabu_search(
        initial_state,
        descendants,
        |state| cost(&state.clusters),
        max_iterations,
        stopping_cost,
    );

    best_state.clusters
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clustering::diameter;

    #[test]
    fn test_clustering_with_diameter_cost() {
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let n_clusters = 2;
        let max_iterations = 100;
        let stopping_cost = None;

        let mut clusters: Vec<Vec<i32>> = cluster_tabu(
            items,
            |clusters: &Vec<Vec<i32>>| {
                clusters
                    .iter()
                    .map(|cluster| {
                        diameter(cluster, |x, y| (x - y).abs() as f64).unwrap_or(f64::NEG_INFINITY)
                    })
                    .fold(f64::NEG_INFINITY, f64::max)
            },
            n_clusters,
            max_iterations,
            stopping_cost,
        )
        .into_iter()
        .map(|mut cluster| {
            cluster.sort();
            cluster
        })
        .collect();
        clusters.sort();

        assert_eq!(clusters.len(), n_clusters);
        assert_eq!(clusters, vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]]);
    }
}
