//! # Gymnarium Base
//!
//! `gymnarium_base` is a collection of traits and enums to support creating
//! reinforcement environments like the python package `gym`.

/// Dimension values either as discrete value or continuous value.
#[derive(Debug, Clone)]
pub enum DimensionValue {
    DISCRETE(i64),
    CONTINUOUS(f64),
}

impl PartialEq for DimensionValue {
    /// ```
    /// # use gymnarium_base::DimensionValue;
    /// assert_eq!(DimensionValue::DISCRETE(1), DimensionValue::DISCRETE(1));
    /// assert_ne!(DimensionValue::DISCRETE(1), DimensionValue::DISCRETE(2));
    /// assert_ne!(DimensionValue::DISCRETE(1), DimensionValue::CONTINUOUS(1.0));
    /// assert_eq!(DimensionValue::CONTINUOUS(1.0), DimensionValue::CONTINUOUS(1.0));
    /// assert_ne!(DimensionValue::CONTINUOUS(1.0), DimensionValue::CONTINUOUS(1.1));
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match *self {
            DimensionValue::DISCRETE(i) => match *other {
                DimensionValue::DISCRETE(j) => i == j,
                _ => false,
            },
            DimensionValue::CONTINUOUS(f) => match *other {
                DimensionValue::CONTINUOUS(g) => f == g,
                _ => false,
            },
        }
    }
}

/// Inclusive boundaries of dimension values.
#[derive(Debug, Clone)]
pub enum DimensionBoundaries {
    DISCRETE { minimum: i64, maximum: i64 },
    CONTINUOUS { minimum: f64, maximum: f64 },
}

impl DimensionBoundaries {
    /// Checks if a given DimensionValue is inclusively inside these minimum and maximum values.
    ///
    /// It is also able to compare DISCRETE values with CONTINUOUS boundaries.
    /// But NOT vice versa, because a CONTINUOUS 1.2f64 is not within DISCRETE boundaries of 1 to 3, which results in values 1, 2 and 3.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gymnarium_base::{DimensionBoundaries, DimensionValue};
    /// let continuous_boundaries = DimensionBoundaries::CONTINUOUS {
    ///     minimum: 1.0f64,
    ///     maximum: 1.5f64
    /// };
    /// assert_eq!(true, continuous_boundaries.contains(&DimensionValue::CONTINUOUS(1.2f64)));
    /// assert_eq!(true, continuous_boundaries.contains(&DimensionValue::DISCRETE(1)));
    /// assert_eq!(false, continuous_boundaries.contains(&DimensionValue::DISCRETE(3)));
    ///
    /// let discrete_boundaries = DimensionBoundaries::DISCRETE {
    ///     minimum: 1,
    ///     maximum: 2
    /// };
    /// assert_eq!(false, discrete_boundaries.contains(&DimensionValue::CONTINUOUS(1.2f64)));
    /// assert_eq!(true, discrete_boundaries.contains(&DimensionValue::DISCRETE(1)));
    /// assert_eq!(false, discrete_boundaries.contains(&DimensionValue::DISCRETE(3)));
    /// ```
    pub fn contains(&self, value: &DimensionValue) -> bool {
        match self {
            Self::DISCRETE { minimum, maximum } => match value {
                DimensionValue::DISCRETE(val) => minimum <= val && val <= maximum,
                DimensionValue::CONTINUOUS(val) => {
                    if (val.floor() - *val).abs() < std::f64::EPSILON {
                        *minimum <= val.floor() as i64 && val.ceil() as i64 <= *maximum
                    } else {
                        false
                    }
                }
            },
            Self::CONTINUOUS { minimum, maximum } => match value {
                DimensionValue::CONTINUOUS(val) => minimum <= val && val <= maximum,
                DimensionValue::DISCRETE(val) => *minimum <= *val as f64 && *val as f64 <= *maximum,
            },
        }
    }
}

impl PartialEq for DimensionBoundaries {
    /// ```
    /// # use gymnarium_base::DimensionBoundaries;
    /// assert_eq!(
    ///     DimensionBoundaries::DISCRETE {minimum: 1, maximum: 2},
    ///     DimensionBoundaries::DISCRETE {minimum: 1, maximum: 2}
    /// );
    /// assert_eq!(
    ///     DimensionBoundaries::CONTINUOUS {minimum: 1.5, maximum: 3.3},
    ///     DimensionBoundaries::CONTINUOUS {minimum: 1.5, maximum: 3.3}
    /// );
    /// assert_ne!(
    ///     DimensionBoundaries::DISCRETE {minimum: 1, maximum: 2},
    ///     DimensionBoundaries::CONTINUOUS {minimum: 1.5, maximum: 3.3}
    /// );
    /// assert_ne!(
    ///     DimensionBoundaries::DISCRETE {minimum: 1, maximum: 2},
    ///     DimensionBoundaries::DISCRETE {minimum: 2, maximum: 3}
    /// );
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match *self {
            DimensionBoundaries::DISCRETE {
                minimum: self_minimum,
                maximum: self_maximum,
            } => match *other {
                DimensionBoundaries::DISCRETE {
                    minimum: other_minimum,
                    maximum: other_maximum,
                } => self_minimum == other_minimum && self_maximum == other_maximum,
                _ => false,
            },
            DimensionBoundaries::CONTINUOUS {
                minimum: self_minimum,
                maximum: self_maximum,
            } => match *other {
                DimensionBoundaries::CONTINUOUS {
                    minimum: other_minimum,
                    maximum: other_maximum,
                } => self_minimum == other_minimum && self_maximum == other_maximum,
                _ => false,
            },
        }
    }
}

/// List of dimension boundaries for available environment agent actions.
pub type ActionSpace = Vec<DimensionBoundaries>;

/// List of dimension boundaries for the observable environment state.
pub type ObservationSpace = Vec<DimensionBoundaries>;

/// List of dimension values as the observable environment state.
pub type EnvironmentState = Vec<DimensionValue>;

/// List of dimension values as the agent action.
pub type AgentAction = Vec<DimensionValue>;

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
pub trait Environment<E: std::error::Error> {
    /// Returns the available boundaries for the actions for this environment.
    fn action_space(&self) -> ActionSpace;

    /// Returns the boundaries for the observable states for this environment.
    fn observation_space(&self) -> ObservationSpace;

    /// Returns the suggested episode step count if the environment provides one.
    fn suggested_episode_steps_count(&self) -> Option<u128>;

    /// Resets the state and initial resources of the environment and returns the initial state.
    ///
    /// Should be called even before the first step is done.
    /// Otherwise there might be no or an invalid state.
    ///
    /// Optionally a seed can be given to initialise the internal random number generator.
    fn reset(&mut self, random_seed: Option<Seed>) -> Result<EnvironmentState, E>;

    /// Performs a step within this environment with the given agent action
    fn step(&mut self, action: &AgentAction) -> Result<(EnvironmentState, f64, bool), E>;

    /// Cleans up resources of this environment.
    ///
    /// Should be called at the very end of usage.
    fn close(&mut self) -> Result<(), E>;
}

/// Base trait for an agent.
pub trait Agent<E: std::error::Error> {
    /// Resets the state and initial resources of the agent.
    ///
    /// Should be called even before the first step is done.
    /// Otherwise the agent could be in an invalid state.
    ///
    /// Optionally a seed can be given to initialise the internal random number generator.
    fn reset(&mut self, random_seed: Option<Seed>) -> Result<(), E>;

    /// Returns an action based on the environment state given.
    fn choose_action(&self, state: &EnvironmentState) -> Result<AgentAction, E>;

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
