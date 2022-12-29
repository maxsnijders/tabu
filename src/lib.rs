//! Provides local search functionality and related algorithms
//! Currently provided search algorithms:
//! - tabu search
//! 
//! Currently provided derived applications
//! - clustering
//! 
//! for examples, see the included tests.

pub mod search;
pub use search::tabu_search;

pub mod clustering;
pub use clustering::cluster_tabu;
