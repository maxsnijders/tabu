use hashbrown::HashSet;
use core::hash::Hash;

/// Runs a tabu search *minimization*.
pub fn tabu_search<State, F, D>(
    state: State,
    descendants: F,
    cost: impl Fn(&State) -> f64,
    max_iterations: usize,
    stopping_cost: f64,
) -> State
where
    D: Iterator<Item = State>,
    F: Fn(&State) -> D,
    State: Hash + Clone + Eq,
{
    let mut tabu_list = HashSet::new();
    let mut best = state.clone();
    let mut best_cost = cost(&best);

    for _ in 0..max_iterations {
        let mut best_descendant = None;
        let mut best_descendant_cost = f64::INFINITY;

        for descendant in descendants(&best) {
            if tabu_list.contains(&descendant) {
                continue;
            }

            let descendant_cost = cost(&descendant);

            if descendant_cost < stopping_cost {
                return descendant;
            }

            if descendant_cost < best_descendant_cost {
                best_descendant = Some(descendant);
                best_descendant_cost = descendant_cost;
            }
        }

        if let Some(descendant) = best_descendant {
            if best_descendant_cost < best_cost {
                best = descendant.clone();
                best_cost = best_descendant_cost;
            }

            tabu_list.insert(descendant);
        }

    }

    
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabu_search() {
        let state = 20;
        let descendants = |state: &i32| (state - 1..=state + 1).filter(|&x| x >= 0);
        let cost = |state: &i32| state.pow(2) as f64;
        let max_iterations = 100;
        let stopping_cost = 0.0;

        let best = tabu_search(state, descendants, cost, max_iterations, stopping_cost);

        assert_eq!(best, 0);
    }
}