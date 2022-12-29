# Tabu

Provides local search functionality and related algorithms
Currently provided search algorithms:
- tabu search

Currently provided derived applications:
- clustering

for detailed examples, see the included tests. 

## Running a tabu search
The following code runs a tabu search on a quadratic optimization problem from a list of initial state.

```rs
    mod tabu;
    use tabu::tabu_search;
    use itertools::Itertools; 

    for initial_state in vec![(10, 10), (0, 0), (7, 0), (6, 5)] {
        let step = 1;

        let descendants = |&(x, y): &(i32, i32)| {
            (-1..2)
                .cartesian_product(-1..2)
                .map(move |(dx, dy)| (x + dx * step, y + dy * step))
        };
        let cost = |&(x, y): &(i32, i32)| ((x - 5).pow(2) + (y - 5).pow(2)) as f64;
        let max_iterations = 100;
        let stopping_cost = 0.0;

        let best = tabu_search(
            initial_state,
            descendants,
            cost,
            max_iterations,
            Some(stopping_cost),
        );

        assert_eq!(best, (5, 5));
    }
```

## Running a clustering job

The following code clusters a set of integers into two clusters, with the cost being the diameter of the largest cluster. 

```rs
    mod tabu;
    use tabu::{cluster_tabu, diameter};

    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let n_clusters = 2;
    let max_iterations = 100;
    let stopping_cost = None; // Stopping cost of None means that we'll only stop when we've exhausted all our options or we've hit the iteration limit

    let mut clusters: Vec<Vec<i32>> = cluster_tabu(
        items,
        // The cost function - note that it's easy to write different cost functions here.
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
```
