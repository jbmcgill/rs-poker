/// Module that can generate possible cards for a starting hand.
mod starting_hand;
/// Export `StartingHand`
pub use self::starting_hand::{Suitedness, StartingHand};

/// Module for `MonteCarloGame` that will hold the current state of the game for simulation.
mod monte_carlo_game;
/// Export `MonteCarloGame`
pub use self::monte_carlo_game::MonteCarloGame;

/// Module with all the starting hand parsing code.
mod parse;
/// Export `RangeParser`
pub use self::parse::RangeParser;