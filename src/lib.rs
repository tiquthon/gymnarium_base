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

/// List of dimension boundaries for available environment agent actions.
pub type ActionSpace = Vec<DimensionBoundaries>;

/// List of dimension boundaries for the observable environment state.
pub type ObservationSpace = Vec<DimensionBoundaries>;

/// List of dimension values as the observable environment state.
pub type EnvironmentState = Vec<DimensionValue>;

/// List of dimension values as the agent action.
pub type AgentAction = Vec<DimensionValue>;

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
    fn reset(&mut self, random_seed: Option<u64>) -> Result<EnvironmentState, E>;

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
    fn reset(&mut self) -> Result<(), E>;

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
