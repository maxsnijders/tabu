use core::hash::Hash;
use hashbrown::HashSet;

/// Runs a tabu search *minimization*.
/// # Arguments
/// * `state`: the initial state to start the search from
/// * `descendants`: a function to generate (possible) descendants of a state given a state
/// * `cost`: the cost of a state
/// * `max_iterations`: we stop when we've ran through this many iterations 
/// * `stopping_cost`: if not None, we stop when our cost no longer exceeds this value.
pub fn tabu_search<State, F, D>(
    state: State,
    descendants: F,
    cost: impl Fn(&State) -> f64,
    max_iterations: usize,
    stopping_cost: Option<f64>,
) -> State
where
    D: Iterator<Item = State>,
    F: Fn(&State) -> D,
    State: Hash + Clone + Eq,
{
    let mut tabu_list = HashSet::new();
    let mut best = state.clone();
    let mut best_cost = cost(&best);
    let mut current = best.clone();

    // Loop until we reach the stopping cost or the maximum number of iterations.
    for _ in 0..max_iterations {
        // Keep track of the best descendant we've seen so far.
        let mut best_descendant = None;
        let mut best_descendant_cost = f64::INFINITY;

        // Add the current state to the tabu list
        tabu_list.insert(current.clone());

        // Consider all descendants of the current state.
        for descendant in descendants(&current) {
            // If the descendant is in the tabu list, skip it.
            if tabu_list.contains(&descendant) {
                continue;
            }

            // If the descendant is better than the stopping cost, return it.
            let descendant_cost = cost(&descendant);
            if let Some(sc) = stopping_cost {
                if descendant_cost < sc {
                    return descendant;
                }
            }

            // If the descendant is better than the best descendant we've seen so far, update the best descendant.
            if descendant_cost < best_descendant_cost {
                best_descendant = Some(descendant);
                best_descendant_cost = descendant_cost;
            }
        }

        // If the best descendant is better than the best state we've seen so far, update the best state.
        // Also, update the current state to the best descendant of the current state.
        if let Some(descendant) = best_descendant {
            if best_descendant_cost < best_cost {
                best = descendant.clone();
                best_cost = best_descendant_cost;
            }

            current = descendant;
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_tabu_search_linear() {
        let state = 20;
        let descendants = |state: &i32| (state - 1..=state + 1).filter(|&x| x >= 0);
        let cost = |state: &i32| *state as f64;
        let max_iterations = 100;
        let stopping_cost = None;

        let best = tabu_search(state, descendants, cost, max_iterations, stopping_cost);

        assert_eq!(best, 0);
    }

    #[test]
    fn test_tabu_search_cross_barier() {
        let state = 0;
        let descendants = |state: &i32| (state - 1..=state + 1).filter(|&x| x >= 0 && x <= 10);
        let cost = |&state: &i32| if state < 3 { state - 3 } else { 3 - state } as f64;
        let max_iterations = 100;
        let stopping_cost = None;

        let best = tabu_search(state, descendants, cost, max_iterations, stopping_cost);

        assert_eq!(best, 10);
    }

    #[test]
    fn test_tabu_search_quadratic() {
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
    }
}
