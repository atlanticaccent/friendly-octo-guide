//! Cached Pokemon description retrieval and subsequent fun translation of descriptions.
//! 
//! Utilises the Pokeapi and Funtranslations APIs.
//! 
//! Public API served by Warp, API connections made with Hyper. Caching provided
//! by Moka. Utilises async/await where possible on a tokio runtime - runtime 
//! provided by included libraries.

pub mod util;
pub mod models;
pub mod api;
pub mod server;