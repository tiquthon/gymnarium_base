//! # Gymnarium Base
//!
//! `gymnarium_base` is a collection of structs, traits and enums to support creating
//! reinforcement environments like the python package `gym`.

extern crate rand;

pub mod space;

use std::fmt::Debug;

use space::{Position, Space};

/// Space for the observable environment state.
pub type ObservationSpace = Space;

/// Position in Space as the observable environment state.
pub type EnvironmentState = Position;

/// Space for available environment agent actions.
pub type ActionSpace = Space;

/// Position in Space as the agent action.
pub type AgentAction = Position;

/// Provides conversion from various values into acceptable seed values.
pub struct Seed {
    pub seed_value: Vec<u8>,
}

impl From<String> for Seed {
    /// # Examples
    /// ```
    /// # use gymnarium_base::Seed;
    /// let seed = Seed::from("12345678".to_string());
    /// let result: [u8; 8] = seed.into();
    /// let expected = [49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8];
    /// assert_eq!(expected, result);
    /// ```
    fn from(s: String) -> Self {
        Self::from(s.as_bytes().to_vec())
    }
}

impl From<&str> for Seed {
    /// # Examples
    /// ```
    /// # use gymnarium_base::Seed;
    /// let seed = Seed::from("12345678");
    /// let result: [u8; 8] = seed.into();
    /// let expected = [49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8];
    /// assert_eq!(expected, result);
    /// ```
    fn from(s: &str) -> Self {
        Self::from(s.as_bytes().to_vec())
    }
}

impl From<Vec<u8>> for Seed {
    /// # Examples
    /// ```
    /// # use gymnarium_base::Seed;
    /// let seed = Seed::from(vec!(1, 2, 3, 4, 5, 6, 7, 8));
    /// let result: [u8; 8] = seed.into();
    /// let expected = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
    /// assert_eq!(expected, result);
    /// ```
    fn from(v: Vec<u8>) -> Self {
        Self { seed_value: v }
    }
}

impl From<&[u8]> for Seed {
    /// # Examples
    /// ```
    /// # use gymnarium_base::Seed;
    /// let seed = Seed::from(&vec!(0, 1, 2, 3, 4, 5, 6, 7, 8)[1..9]);
    /// let result: [u8; 8] = seed.into();
    /// let expected = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
    /// assert_eq!(expected, result);
    /// ```
    fn from(v: &[u8]) -> Self {
        Self {
            seed_value: v.to_vec(),
        }
    }
}

impl From<u64> for Seed {
    /// # Examples
    /// ```
    /// # use gymnarium_base::Seed;
    /// let seed = Seed::from(2306414829080219452);
    /// // 2306414829080219452
    /// // = 0010 0000  0000 0010  0000 1000  0001 0001  0011 0000  0000 0101  0000 1111  0011 1100
    /// let expected_seed = [32u8, 2u8, 8u8, 17u8, 48u8, 5u8, 15u8, 60u8];
    /// let seed_array: [u8; 8] = seed.into();
    /// assert_eq!(expected_seed, seed_array);
    /// ```
    fn from(u: u64) -> Self {
        Self {
            seed_value: vec![
                (u >> (8u64 * 7u64)) as u8,
                (u >> (8u64 * 6u64)) as u8,
                (u >> (8u64 * 5u64)) as u8,
                (u >> (8u64 * 4u64)) as u8,
                (u >> (8u64 * 3u64)) as u8,
                (u >> (8u64 * 2u64)) as u8,
                (u >> 8u64) as u8,
                u as u8,
            ],
        }
    }
}

impl Into<[u8; 32]> for Seed {
    /// # Examples
    /// ```
    /// # use gymnarium_base::Seed;
    /// let seed_array: [u8; 32] = Seed::from((0u8..64u8).collect::<Vec<u8>>()).into();
    /// let expected_array = [
    ///     32u8, 34u8, 36u8, 38u8, 40u8, 42u8, 44u8, 46u8, 48u8, 50u8, 52u8, 54u8,
    ///     56u8, 58u8, 60u8, 62u8, 64u8, 66u8, 68u8, 70u8, 72u8, 74u8, 76u8, 78u8,
    ///     80u8, 82u8, 84u8, 86u8, 88u8, 90u8, 92u8, 94u8
    /// ];
    /// assert_eq!(expected_array, seed_array);
    /// ```
    fn into(self) -> [u8; 32] {
        self.seed_value
            .into_iter()
            .fold(([0u8; 32], 0usize), |(mut output, index), v| {
                output[index] = output[index].overflowing_add(v).0;
                (output, (index + 1) % 32)
            })
            .0
    }
}

impl Into<[u8; 16]> for Seed {
    /// # Examples
    /// ```
    /// # use gymnarium_base::Seed;
    /// let seed_array: [u8; 16] = Seed::from((0u8..32u8).collect::<Vec<u8>>()).into();
    /// let expected_array = [
    ///     16u8, 18u8, 20u8, 22u8, 24u8, 26u8, 28u8, 30u8,
    ///     32u8, 34u8, 36u8, 38u8, 40u8, 42u8, 44u8, 46u8
    /// ];
    /// assert_eq!(expected_array, seed_array);
    /// ```
    fn into(self) -> [u8; 16] {
        self.seed_value
            .into_iter()
            .fold(([0u8; 16], 0usize), |(mut output, index), v| {
                output[index] = output[index].overflowing_add(v).0;
                (output, (index + 1) % 16)
            })
            .0
    }
}

impl Into<[u8; 8]> for Seed {
    /// # Examples
    /// ```
    /// # use gymnarium_base::Seed;
    /// let seed_array: [u8; 8] = Seed::from((0u8..16u8).collect::<Vec<u8>>()).into();
    /// let expected_array = [
    ///     8u8, 10u8, 12u8, 14u8, 16u8, 18u8, 20u8, 22u8
    /// ];
    /// assert_eq!(expected_array, seed_array);
    /// ```
    fn into(self) -> [u8; 8] {
        self.seed_value
            .into_iter()
            .fold(([0u8; 8], 0usize), |(mut output, index), v| {
                output[index] = output[index].overflowing_add(v).0;
                (output, (index + 1) % 8)
            })
            .0
    }
}

impl Into<u64> for Seed {
    /// # Examples
    /// ```
    /// # use gymnarium_base::Seed;
    /// let seed_number: u64 = Seed::from(vec!(1, 2, 3, 4)).into();
    /// // 0000 0001  0000 0010  0000 0011  0000 0100  0000 0000  0000 0000  0000 0000  0000 0000
    /// // = 72.623.859.706.101.760
    /// assert_eq!(72623859706101760, seed_number);
    /// ```
    fn into(self) -> u64 {
        let m: [u8; 8] = self.into();
        m.iter()
            .fold((0u64, 7usize), |(mut output, index), input| {
                output |= (*input as u64) << (index as u64 * 8);
                (output, index.overflowing_sub(1).0)
            })
            .0
    }
}

/// Base trait for an environment.
pub trait Environment<E, I>
where
    E: std::error::Error,
    I: Debug,
{
    /// Returns the available boundaries for the actions for this environment.
    fn action_space(&self) -> ActionSpace;

    /// Returns the boundaries for the observable states for this environment.
    fn observation_space(&self) -> ObservationSpace;

    /// Returns the suggested episode step count if the environment provides one.
    fn suggested_episode_steps_count(&self) -> Option<u128>;

    /// Resets a possible internal random number generator with the given seed or by entropy.
    fn reseed(&mut self, random_seed: Option<Seed>) -> Result<(), E>;

    /// Resets the state and initial resources of the environment and returns the initial state.
    ///
    /// Should be called even before the first step is done.
    /// Otherwise there might be no or an invalid state.
    ///
    /// If predictable behaviour is wished, it's recommended to call `seed` in front of `reset`.
    fn reset(&mut self) -> Result<EnvironmentState, E>;

    /// Performs a step within this environment with the given agent action
    fn step(&mut self, action: &AgentAction) -> Result<(EnvironmentState, f64, bool, I), E>;

    /// Cleans up resources of this environment.
    ///
    /// Should be called at the very end of usage.
    fn close(&mut self) -> Result<(), E>;
}

/// Base trait for an agent.
pub trait Agent<E>
where
    E: std::error::Error,
{
    /// Resets a possible internal random number generator with the given seed or by entropy.
    fn reseed(&mut self, random_seed: Option<Seed>) -> Result<(), E>;

    /// Resets the state and initial resources of the agent.
    ///
    /// Should be called even before the first step is done.
    /// Otherwise the agent could be in an invalid state.
    ///
    /// If predictable behaviour is wished, it's recommended to call `seed` in front of `reset`.
    fn reset(&mut self) -> Result<(), E>;

    /// Returns an action based on the environment state given.
    fn choose_action(&mut self, state: &EnvironmentState) -> Result<AgentAction, E>;

    /// Lets this agent process the result of the last step.
    fn process_reward(
        &mut self,
        old_state: &EnvironmentState,
        new_state: &EnvironmentState,
        reward: f64,
        is_done: bool,
    ) -> Result<(), E>;

    /// Cleans up resources of this agent.
    ///
    /// Should be called at the very end of usage.
    fn close(&mut self) -> Result<(), E>;
}
