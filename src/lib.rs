//! # Gymnarium Base
//!
//! `gymnarium_base` is a collection of structs, traits and enums to support creating
//! reinforcement environments like the python package `gym`.

extern crate num_traits;

use std::fmt::Debug;

use num_traits::{Float, PrimInt};

/// Dimension values either as discrete value, continuous value or containing more values.
#[derive(Debug, Clone)]
pub enum DimensionValue<D: PrimInt + Debug, C: Float + Debug> {
    DISCRETE(D),
    CONTINUOUS(C),
    MULTIPLE(Vec<DimensionValue<D, C>>),
}

/// Alias for DimensionValue<i32, f32>
pub type DimensionValueI32F32 = DimensionValue<i32, f32>;

/// Alias for DimensionValue<i64, f64>
pub type DimensionValueI64F64 = DimensionValue<i64, f64>;

/// Alias for DimensionValue<u32, f32>
pub type DimensionValueU32F32 = DimensionValue<u32, f32>;

/// Alias for DimensionValue<u64, f64>
pub type DimensionValueU64F64 = DimensionValue<u64, f64>;

impl<D: PrimInt + Debug, C: Float + Debug> DimensionValue<D, C> {
    /// Creates a new discrete DimensionValue.
    ///
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionValue, DimensionValueI64F64};
    /// let value = DimensionValueI64F64::discrete(4);
    /// assert_eq!(
    ///     DimensionValue::DISCRETE(4),
    ///     value
    /// );
    /// ```
    pub fn discrete(value: D) -> Self {
        DimensionValue::DISCRETE(value)
    }

    /// Creates a new continuous DimensionValue.
    ///
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionValue, DimensionValueI64F64};
    /// let value = DimensionValueI64F64::continuous(4.5);
    /// assert_eq!(
    ///     DimensionValue::CONTINUOUS(4.5),
    ///     value
    /// );
    /// ```
    pub fn continuous(value: C) -> Self {
        DimensionValue::CONTINUOUS(value)
    }

    /// Creates a new value containing more values.
    ///
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionValue, DimensionValueI64F64};
    /// let value = DimensionValueI64F64::multiple(vec![
    ///     DimensionValueI64F64::discrete(1),
    ///     DimensionValueI64F64::continuous(3.6),
    ///     DimensionValueI64F64::multiple(vec![
    ///         DimensionValueI64F64::discrete(7),
    ///         DimensionValueI64F64::continuous(3.4)
    ///     ])
    /// ]);
    ///
    /// assert_eq!(
    ///     DimensionValue::MULTIPLE(vec![
    ///         DimensionValue::DISCRETE(1),
    ///         DimensionValue::CONTINUOUS(3.6),
    ///         DimensionValue::MULTIPLE(vec![
    ///             DimensionValue::DISCRETE(7),
    ///             DimensionValue::CONTINUOUS(3.4)
    ///         ])
    ///     ]),
    ///     value
    /// );
    /// ```
    pub fn multiple(values: Vec<DimensionValue<D, C>>) -> Self {
        DimensionValue::MULTIPLE(values)
    }

    /// Checks if this and another DimensionValue have the same structure.
    ///
    /// Does not check if the values match!
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionValue, DimensionValueI64F64};
    /// let shape_a = DimensionValueI64F64::discrete(0);
    /// assert!(shape_a.matches_shape_of(&shape_a));
    ///
    /// let shape_b = DimensionValueI64F64::continuous(22.8);
    /// assert!(shape_b.matches_shape_of(&shape_b));
    /// assert!(!shape_b.matches_shape_of(&shape_a));
    ///
    /// let shape_c = DimensionValueI64F64::multiple(vec![
    ///     DimensionValueI64F64::discrete(10),
    ///     DimensionValueI64F64::multiple(vec![
    ///         DimensionValueI64F64::continuous(12.3),
    ///         DimensionValueI64F64::continuous(-1.8)
    ///     ])
    /// ]);
    /// assert!(shape_c.matches_shape_of(&shape_c));
    /// assert!(!shape_c.matches_shape_of(&shape_a));
    /// assert!(!shape_c.matches_shape_of(&shape_b));
    ///
    /// let shape_d = DimensionValueI64F64::continuous(9.7);
    /// assert!(shape_d.matches_shape_of(&shape_d));
    /// assert!(!shape_d.matches_shape_of(&shape_a));
    /// assert!(shape_d.matches_shape_of(&shape_b));
    /// assert!(!shape_d.matches_shape_of(&shape_c));
    /// ```
    pub fn matches_shape_of(&self, other: &DimensionValue<D, C>) -> bool {
        match self {
            DimensionValue::DISCRETE(_) => match other {
                DimensionValue::DISCRETE(_) => true,
                _ => false,
            },
            DimensionValue::CONTINUOUS(_) => match other {
                DimensionValue::CONTINUOUS(_) => true,
                _ => false,
            },
            DimensionValue::MULTIPLE(values) => match other {
                DimensionValue::MULTIPLE(other_values) => {
                    values.len() == other_values.len()
                        && !values
                            .iter()
                            .zip(other_values.iter())
                            .any(|(v, o)| !v.matches_shape_of(o))
                }
                _ => false,
            },
        }
    }
}

impl<D: PrimInt + Debug, C: Float + Debug> PartialEq for DimensionValue<D, C> {
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionValue, DimensionValueI64F64};
    /// assert_eq!(DimensionValueI64F64::DISCRETE(1), DimensionValueI64F64::DISCRETE(1));
    /// assert_ne!(DimensionValueI64F64::DISCRETE(1), DimensionValueI64F64::DISCRETE(2));
    /// assert_ne!(DimensionValueI64F64::DISCRETE(1), DimensionValueI64F64::CONTINUOUS(1.0));
    /// assert_eq!(DimensionValueI64F64::CONTINUOUS(1.0), DimensionValueI64F64::CONTINUOUS(1.0));
    /// assert_ne!(DimensionValueI64F64::CONTINUOUS(1.0), DimensionValueI64F64::CONTINUOUS(1.1));
    /// assert_ne!(DimensionValueI64F64::MULTIPLE(vec![DimensionValueI64F64::CONTINUOUS(1.1)]), DimensionValueI64F64::CONTINUOUS(1.1));
    /// assert_eq!(
    ///     DimensionValue::MULTIPLE(vec![DimensionValueI64F64::CONTINUOUS(3.8)]),
    ///     DimensionValue::MULTIPLE(vec![DimensionValueI64F64::CONTINUOUS(3.8)])
    /// );
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match self {
            DimensionValue::DISCRETE(i) => match other {
                DimensionValue::DISCRETE(j) => i == j,
                _ => false,
            },
            DimensionValue::CONTINUOUS(f) => match other {
                DimensionValue::CONTINUOUS(g) => f == g,
                _ => false,
            },
            DimensionValue::MULTIPLE(values) => match other {
                DimensionValue::MULTIPLE(other_values) => values == other_values,
                _ => false,
            },
        }
    }
}

/// Inclusive boundaries of dimension values.
#[derive(Debug, Clone)]
pub enum DimensionBoundaries<D: PrimInt + Debug, C: Float + Debug> {
    DISCRETE { minimum: D, maximum: D },
    CONTINUOUS { minimum: C, maximum: C },
    MULTIPLE(Vec<DimensionBoundaries<D, C>>),
}

/// Alias for DimensionBoundaries<i32, f32>
pub type DimensionBoundariesI32F32 = DimensionBoundaries<i32, f32>;

/// Alias for DimensionBoundaries<i64, f64>
pub type DimensionBoundariesI64F64 = DimensionBoundaries<i64, f64>;

/// Alias for DimensionBoundaries<u32, f32>
pub type DimensionBoundariesU32F32 = DimensionBoundaries<u32, f32>;

/// Alias for DimensionBoundaries<u64, f64>
pub type DimensionBoundariesU64F64 = DimensionBoundaries<u64, f64>;

impl<D: PrimInt + Debug, C: Float + Debug> DimensionBoundaries<D, C> {
    /// Creates new discrete DimensionBoundaries with &#91;0;maximum&#93;.
    ///
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionBoundariesI64F64, DimensionBoundaries};
    /// let boundaries = DimensionBoundariesI64F64::discrete_zero(4);
    /// assert_eq!(
    ///     DimensionBoundaries::DISCRETE { minimum: 0, maximum: 4 },
    ///     boundaries
    /// );
    /// ```
    pub fn discrete_zero(maximum: D) -> Self {
        DimensionBoundaries::DISCRETE {
            minimum: D::zero(),
            maximum,
        }
    }

    /// Creates new discrete DimensionBoundaries with &#91;minimum;maximum&#93;.
    ///
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionBoundaries, DimensionBoundariesI64F64};
    /// let boundaries = DimensionBoundariesI64F64::discrete(-5, 4);
    /// assert_eq!(
    ///     DimensionBoundaries::DISCRETE { minimum: -5, maximum: 4 },
    ///     boundaries
    /// );
    /// ```
    pub fn discrete(minimum: D, maximum: D) -> Self {
        DimensionBoundaries::DISCRETE { minimum, maximum }
    }

    /// Creates new continuous DimensionBoundaries with &#91;0;maximum&#93;.
    ///
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionBoundaries, DimensionBoundariesI64F64};
    /// let boundaries = DimensionBoundariesI64F64::continuous_zero(12.4);
    /// assert_eq!(
    ///     DimensionBoundaries::CONTINUOUS { minimum: 0.0, maximum: 12.4 },
    ///     boundaries
    /// );
    /// ```
    pub fn continuous_zero(maximum: C) -> Self {
        DimensionBoundaries::CONTINUOUS {
            minimum: C::zero(),
            maximum,
        }
    }

    /// Creates new continuous DimensionBoundaries with &#91;minimum;maximum&#93;.
    ///
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionBoundaries, DimensionBoundariesI64F64};
    /// let boundaries = DimensionBoundariesI64F64::continuous(-32.67, 12.4);
    /// assert_eq!(
    ///     DimensionBoundaries::CONTINUOUS { minimum: -32.67, maximum: 12.4 },
    ///     boundaries
    /// );
    /// ```
    pub fn continuous(minimum: C, maximum: C) -> Self {
        DimensionBoundaries::CONTINUOUS { minimum, maximum }
    }

    /// Creates new DimensionBoundaries containing more boundaries.
    ///
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionBoundaries, DimensionBoundariesI64F64};
    /// let boundaries = DimensionBoundariesI64F64::multiple(vec![
    ///     DimensionBoundariesI64F64::continuous_zero(3.8),
    ///     DimensionBoundariesI64F64::discrete_zero(3),
    ///     DimensionBoundariesI64F64::multiple(vec![
    ///         DimensionBoundariesI64F64::discrete(-1, -5),
    ///         DimensionBoundariesI64F64::continuous(-3.2, -5.3)
    ///     ])
    /// ]);
    /// assert_eq!(
    ///     DimensionBoundaries::MULTIPLE(vec![
    ///         DimensionBoundaries::CONTINUOUS { minimum: 0.0, maximum: 3.8 },
    ///         DimensionBoundaries::DISCRETE { minimum: 0, maximum: 3 },
    ///         DimensionBoundaries::MULTIPLE(vec![
    ///             DimensionBoundaries::DISCRETE { minimum: -1, maximum: -5 },
    ///             DimensionBoundaries::CONTINUOUS { minimum: -3.2, maximum: -5.3 }
    ///         ])
    ///     ]),
    ///     boundaries
    /// );
    /// ```
    pub fn multiple(values: Vec<DimensionBoundaries<D, C>>) -> Self {
        DimensionBoundaries::MULTIPLE(values)
    }

    /// Checks if a given DimensionValue is inclusively inside these minimum and maximum values.
    ///
    /// It is also able to compare DISCRETE values with CONTINUOUS boundaries.
    /// But NOT vice versa, because a CONTINUOUS 1.2f64 is not within DISCRETE boundaries of 1 to 3, which results in values 1, 2 and 3.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gymnarium_base::{DimensionBoundariesI64F64, DimensionValueI64F64};
    /// let continuous_boundaries = DimensionBoundariesI64F64::CONTINUOUS {
    ///     minimum: 1.0f64,
    ///     maximum: 1.5f64
    /// };
    /// assert_eq!(true, continuous_boundaries.contains(&DimensionValueI64F64::CONTINUOUS(1.2f64)));
    /// assert_eq!(true, continuous_boundaries.contains(&DimensionValueI64F64::DISCRETE(1)));
    /// assert_eq!(false, continuous_boundaries.contains(&DimensionValueI64F64::DISCRETE(3)));
    ///
    /// let discrete_boundaries = DimensionBoundariesI64F64::DISCRETE {
    ///     minimum: 1,
    ///     maximum: 2
    /// };
    /// assert_eq!(false, discrete_boundaries.contains(&DimensionValueI64F64::CONTINUOUS(1.2f64)));
    /// assert_eq!(true, discrete_boundaries.contains(&DimensionValueI64F64::DISCRETE(1)));
    /// assert_eq!(false, discrete_boundaries.contains(&DimensionValueI64F64::DISCRETE(3)));
    ///
    /// let multiple_boundaries = DimensionBoundariesI64F64::MULTIPLE(vec![
    ///     DimensionBoundariesI64F64::DISCRETE { minimum: 1, maximum: 5 },
    ///     DimensionBoundariesI64F64::CONTINUOUS { minimum: -1.3, maximum: 6.7 }
    /// ]);
    /// assert_eq!(false, multiple_boundaries.contains(&DimensionValueI64F64::MULTIPLE(vec![
    ///     DimensionValueI64F64::DISCRETE(7)
    /// ])));
    /// assert_eq!(true, multiple_boundaries.contains(&DimensionValueI64F64::MULTIPLE(vec![
    ///     DimensionValueI64F64::DISCRETE(4),
    ///     DimensionValueI64F64::CONTINUOUS(0.6)
    /// ])));
    /// assert_eq!(false, multiple_boundaries.contains(&DimensionValueI64F64::MULTIPLE(vec![
    ///     DimensionValueI64F64::DISCRETE(4),
    ///     DimensionValueI64F64::DISCRETE(-2)
    /// ])));
    /// ```
    pub fn contains(&self, value: &DimensionValue<D, C>) -> bool {
        match self {
            DimensionBoundaries::DISCRETE { minimum, maximum } => match value {
                DimensionValue::DISCRETE(val) => minimum <= val && val <= maximum,
                DimensionValue::CONTINUOUS(val) => {
                    if C::abs(val.floor().sub(*val)) < C::epsilon() {
                        minimum.to_i64().unwrap() <= val.floor().to_i64().unwrap()
                            && val.ceil().to_i64().unwrap() <= maximum.to_i64().unwrap()
                    } else {
                        false
                    }
                }
                _ => false,
            },
            DimensionBoundaries::CONTINUOUS { minimum, maximum } => match value {
                DimensionValue::CONTINUOUS(val) => minimum <= val && val <= maximum,
                DimensionValue::DISCRETE(val) => {
                    minimum.to_f64().unwrap() <= val.to_f64().unwrap()
                        && val.to_f64().unwrap() <= maximum.to_f64().unwrap()
                }
                _ => false,
            },
            DimensionBoundaries::MULTIPLE(values) => match value {
                DimensionValue::MULTIPLE(other_values) => {
                    values.len() == other_values.len()
                        && values
                            .iter()
                            .zip(other_values.iter())
                            .find(|(bound, val)| !bound.contains(val))
                            .is_none()
                }
                _ => false,
            },
        }
    }

    /// Checks if this boundaries shape matches the shape of the other DimensionValues.
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionBoundariesI64F64, DimensionValueI64F64};
    /// let boundaries_shape_a = DimensionBoundariesI64F64::discrete_zero(10);
    /// let boundaries_shape_b = DimensionBoundariesI64F64::continuous_zero(3.7);
    /// let boundaries_shape_c = DimensionBoundariesI64F64::multiple(vec![
    ///     DimensionBoundariesI64F64::continuous(3.3, 3.4),
    ///     DimensionBoundariesI64F64::discrete_zero(33)
    /// ]);
    /// let boundaries_shape_d = DimensionBoundariesI64F64::continuous(-3.5, 4.9);
    ///
    /// let value_shape_a = DimensionValueI64F64::discrete(22);
    /// assert!(boundaries_shape_a.matches_shape_for(&value_shape_a));
    /// assert!(!boundaries_shape_b.matches_shape_for(&value_shape_a));
    /// assert!(!boundaries_shape_c.matches_shape_for(&value_shape_a));
    /// assert!(!boundaries_shape_d.matches_shape_for(&value_shape_a));
    ///
    /// let value_shape_b = DimensionValueI64F64::continuous(100.4);
    /// assert!(!boundaries_shape_a.matches_shape_for(&value_shape_b));
    /// assert!(boundaries_shape_b.matches_shape_for(&value_shape_b));
    /// assert!(!boundaries_shape_c.matches_shape_for(&value_shape_b));
    /// assert!(boundaries_shape_d.matches_shape_for(&value_shape_b));
    ///
    /// let value_shape_c = DimensionValueI64F64::multiple(vec![
    ///     DimensionValueI64F64::continuous(-5.0),
    ///     DimensionValueI64F64::discrete(40)
    /// ]);
    /// assert!(!boundaries_shape_a.matches_shape_for(&value_shape_c));
    /// assert!(!boundaries_shape_b.matches_shape_for(&value_shape_c));
    /// assert!(boundaries_shape_c.matches_shape_for(&value_shape_c));
    /// assert!(!boundaries_shape_d.matches_shape_for(&value_shape_c));
    ///
    /// let value_shape_d = DimensionValueI64F64::multiple(vec![
    ///     DimensionValueI64F64::continuous(-5.0),
    ///     DimensionValueI64F64::multiple(vec![
    ///         DimensionValueI64F64::discrete(22),
    ///         DimensionValueI64F64::continuous(12.3)
    ///     ])
    /// ]);
    /// assert!(!boundaries_shape_a.matches_shape_for(&value_shape_d));
    /// assert!(!boundaries_shape_b.matches_shape_for(&value_shape_d));
    /// assert!(!boundaries_shape_c.matches_shape_for(&value_shape_d));
    /// assert!(!boundaries_shape_d.matches_shape_for(&value_shape_d));
    /// ```
    pub fn matches_shape_for(&self, other: &DimensionValue<D, C>) -> bool {
        match self {
            DimensionBoundaries::DISCRETE { .. } => match other {
                DimensionValue::DISCRETE(_) => true,
                _ => false,
            },
            DimensionBoundaries::CONTINUOUS { .. } => match other {
                DimensionValue::CONTINUOUS(_) => true,
                _ => false,
            },
            DimensionBoundaries::MULTIPLE(boundaries) => match other {
                DimensionValue::MULTIPLE(values) => {
                    boundaries.len() == values.len()
                        && !boundaries
                            .iter()
                            .zip(values.iter())
                            .any(|(b, c)| !b.matches_shape_for(c))
                }
                _ => false,
            },
        }
    }

    /// Checks if this boundaries shape matches the other boundaries shape.
    /// # Examples
    /// ```
    /// # use gymnarium_base::DimensionBoundariesI64F64;
    /// let shape_a = DimensionBoundariesI64F64::discrete_zero(10);
    /// assert!(shape_a.matches_shape_of(&shape_a));
    ///
    /// let shape_b = DimensionBoundariesI64F64::continuous_zero(3.7);
    /// assert!(shape_b.matches_shape_of(&shape_b));
    /// assert!(!shape_b.matches_shape_of(&shape_a));
    ///
    /// let shape_c = DimensionBoundariesI64F64::multiple(vec![
    ///     DimensionBoundariesI64F64::continuous(3.3, 3.4),
    ///     DimensionBoundariesI64F64::discrete_zero(33)
    /// ]);
    /// assert!(shape_c.matches_shape_of(&shape_c));
    /// assert!(!shape_c.matches_shape_of(&shape_a));
    /// assert!(!shape_c.matches_shape_of(&shape_b));
    ///
    /// let shape_d = DimensionBoundariesI64F64::continuous(-3.5, 4.9);
    /// assert!(shape_d.matches_shape_of(&shape_d));
    /// assert!(!shape_d.matches_shape_of(&shape_a));
    /// assert!(shape_d.matches_shape_of(&shape_b));
    /// assert!(!shape_d.matches_shape_of(&shape_c));
    /// ```
    pub fn matches_shape_of(&self, other: &DimensionBoundaries<D, C>) -> bool {
        match self {
            DimensionBoundaries::DISCRETE { .. } => match other {
                DimensionBoundaries::DISCRETE { .. } => true,
                _ => false,
            },
            DimensionBoundaries::CONTINUOUS { .. } => match other {
                DimensionBoundaries::CONTINUOUS { .. } => true,
                _ => false,
            },
            DimensionBoundaries::MULTIPLE(boundaries) => match other {
                DimensionBoundaries::MULTIPLE(other_boundaries) => {
                    boundaries.len() == other_boundaries.len()
                        && !boundaries
                            .iter()
                            .zip(other_boundaries.iter())
                            .any(|(b, o)| !b.matches_shape_of(o))
                }
                _ => false,
            },
        }
    }

    /// Returns a sample value within these boundaries.
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionBoundaries, DimensionBoundariesI64F64, DimensionValue, DimensionValueI64F64};
    /// let discrete_boundaries = DimensionBoundariesI64F64::discrete_zero(10);
    /// let sampled_value = discrete_boundaries.sample(&|min, max| 5, &|min, max| 0f64);
    /// assert_eq!(DimensionValueI64F64::DISCRETE(5i64), sampled_value);
    ///
    /// let continuous_boundaries = DimensionBoundariesI64F64::continuous_zero(2.5);
    /// let sampled_value = continuous_boundaries.sample(&|min, max| 0, &|min, max| 1.7f64);
    /// assert_eq!(DimensionValueI64F64::CONTINUOUS(1.7f64), sampled_value);
    ///
    /// let continuous_boundaries = DimensionBoundariesI64F64::multiple(vec![
    ///     DimensionBoundariesI64F64::discrete_zero(20),
    ///     DimensionBoundariesI64F64::continuous_zero(4.8f64),
    ///     DimensionBoundariesI64F64::multiple(vec![
    ///         DimensionBoundariesI64F64::discrete(5, 7),
    ///         DimensionBoundariesI64F64::continuous(-1.2, -0.2)
    ///     ]),
    /// ]);
    /// let sampled_value = continuous_boundaries.sample(&|min, max| ((max-min)/2)+min, &|min, max| ((max-min)/2f64)+min);
    /// assert_eq!(DimensionValueI64F64::MULTIPLE(vec![
    ///     DimensionValueI64F64::discrete(10),
    ///     DimensionValueI64F64::continuous(2.4f64),
    ///     DimensionValueI64F64::multiple(vec![
    ///         DimensionValueI64F64::discrete(6),
    ///         DimensionValueI64F64::continuous(-0.7)
    ///     ])
    /// ]), sampled_value);
    /// ```
    pub fn sample<FD: Fn(&D, &D) -> D, FC: Fn(&C, &C) -> C>(
        &self,
        discrete_random_provider: &FD,
        continuous_random_provider: &FC,
    ) -> DimensionValue<D, C> {
        match self {
            DimensionBoundaries::DISCRETE { minimum, maximum } => {
                DimensionValue::DISCRETE(discrete_random_provider(minimum, maximum))
            }
            DimensionBoundaries::CONTINUOUS { minimum, maximum } => {
                DimensionValue::CONTINUOUS(continuous_random_provider(minimum, maximum))
            }
            DimensionBoundaries::MULTIPLE(boundaries) => DimensionValue::MULTIPLE(
                boundaries
                    .iter()
                    .map(|b| b.sample(discrete_random_provider, continuous_random_provider))
                    .collect::<Vec<DimensionValue<D, C>>>(),
            ),
        }
    }
}

impl<D: PrimInt + Debug, C: Float + Debug> PartialEq for DimensionBoundaries<D, C> {
    /// ```
    /// # use gymnarium_base::{DimensionBoundaries, DimensionBoundariesI64F64};
    /// assert_eq!(
    ///     DimensionBoundariesI64F64::DISCRETE {minimum: 1, maximum: 2},
    ///     DimensionBoundariesI64F64::DISCRETE {minimum: 1, maximum: 2}
    /// );
    /// assert_eq!(
    ///     DimensionBoundariesI64F64::CONTINUOUS {minimum: 1.5, maximum: 3.3},
    ///     DimensionBoundariesI64F64::CONTINUOUS {minimum: 1.5, maximum: 3.3}
    /// );
    /// assert_eq!(
    ///     DimensionBoundariesI64F64::MULTIPLE(vec![
    ///         DimensionBoundariesI64F64::DISCRETE { minimum: 3, maximum: 6 }
    ///     ]),
    ///     DimensionBoundariesI64F64::MULTIPLE(vec![
    ///         DimensionBoundariesI64F64::DISCRETE { minimum: 3, maximum: 6 }
    ///     ])
    /// );
    /// assert_ne!(
    ///     DimensionBoundariesI64F64::DISCRETE {minimum: 1, maximum: 2},
    ///     DimensionBoundariesI64F64::CONTINUOUS {minimum: 1.5, maximum: 3.3}
    /// );
    /// assert_ne!(
    ///     DimensionBoundariesI64F64::DISCRETE {minimum: 1, maximum: 2},
    ///     DimensionBoundariesI64F64::DISCRETE {minimum: 2, maximum: 3}
    /// );
    /// assert_ne!(
    ///     DimensionBoundariesI64F64::MULTIPLE(vec![
    ///         DimensionBoundariesI64F64::DISCRETE { minimum: 3, maximum: 6 }
    ///     ]),
    ///     DimensionBoundariesI64F64::MULTIPLE(vec![
    ///         DimensionBoundariesI64F64::CONTINUOUS { minimum: 3.0, maximum: 6.6 }
    ///     ])
    /// );
    /// ```
    fn eq(&self, other: &Self) -> bool {
        match self {
            DimensionBoundaries::DISCRETE {
                minimum: self_minimum,
                maximum: self_maximum,
            } => match other {
                DimensionBoundaries::DISCRETE {
                    minimum: other_minimum,
                    maximum: other_maximum,
                } => self_minimum == other_minimum && self_maximum == other_maximum,
                _ => false,
            },
            DimensionBoundaries::CONTINUOUS {
                minimum: self_minimum,
                maximum: self_maximum,
            } => match other {
                DimensionBoundaries::CONTINUOUS {
                    minimum: other_minimum,
                    maximum: other_maximum,
                } => self_minimum == other_minimum && self_maximum == other_maximum,
                _ => false,
            },
            DimensionBoundaries::MULTIPLE(values) => match other {
                DimensionBoundaries::MULTIPLE(other_values) => values == other_values,
                _ => false,
            },
        }
    }
}

/// Defines a position within a Space. Used for AgentAction and EnvironmentState.
/// # Examples
/// ```
/// # use gymnarium_base::{Space, SpacePosition, DimensionValueI64F64, DimensionValue, DimensionBoundaries, DimensionBoundariesI64F64};
/// let action_space = Space::from(vec![
///     DimensionBoundariesI64F64::discrete_zero(25),
///     DimensionBoundariesI64F64::continuous_zero(3.0),
/// ]);
/// let agent_action = SpacePosition::from(vec![
///     DimensionValueI64F64::discrete(20),
///     DimensionValueI64F64::continuous(2.3)
/// ]);
/// ```
#[derive(Debug, Default)]
pub struct SpacePosition<D: PrimInt + Debug, C: Float + Debug> {
    data: Vec<DimensionValue<D, C>>,
}

impl<D: PrimInt + Debug, C: Float + Debug> SpacePosition<D, C> {
    /// Checks if the shape of this position matches with the other one.
    /// # Examples
    /// ```
    /// # use gymnarium_base::{SpacePosition, DimensionValueI64F64};
    /// let position_a = SpacePosition::from(DimensionValueI64F64::discrete(20));
    /// assert!(position_a.matches_shape_of(&position_a));
    ///
    /// let position_b = SpacePosition::from(DimensionValueI64F64::continuous(1.98));
    /// assert!(position_b.matches_shape_of(&position_b));
    /// assert!(!position_b.matches_shape_of(&position_a));
    ///
    /// let position_c = SpacePosition::from(DimensionValueI64F64::multiple(vec![
    ///     DimensionValueI64F64::discrete(6),
    ///     DimensionValueI64F64::continuous(4.3),
    /// ]));
    /// assert!(position_c.matches_shape_of(&position_c));
    /// assert!(!position_c.matches_shape_of(&position_a));
    /// assert!(!position_c.matches_shape_of(&position_b));
    ///
    /// let position_d = SpacePosition::from(DimensionValueI64F64::discrete(1));
    /// assert!(position_d.matches_shape_of(&position_d));
    /// assert!(position_d.matches_shape_of(&position_a));
    /// assert!(!position_d.matches_shape_of(&position_b));
    /// assert!(!position_d.matches_shape_of(&position_c));
    /// ```
    pub fn matches_shape_of(&self, other: &SpacePosition<D, C>) -> bool {
        if self.data.len() != other.data.len() {
            false
        } else {
            !self
                .data
                .iter()
                .zip(other.data.iter())
                .any(|(s, o)| !s.matches_shape_of(o))
        }
    }
}

impl<D: PrimInt + Debug, C: Float + Debug> From<DimensionValue<D, C>> for SpacePosition<D, C> {
    fn from(values: DimensionValue<D, C>) -> Self {
        Self { data: vec![values] }
    }
}

impl<D: PrimInt + Debug, C: Float + Debug> From<Vec<DimensionValue<D, C>>> for SpacePosition<D, C> {
    fn from(values: Vec<DimensionValue<D, C>>) -> Self {
        Self { data: values }
    }
}

impl<D: PrimInt + Debug, C: Float + Debug> Into<Vec<DimensionValue<D, C>>> for SpacePosition<D, C> {
    fn into(self) -> Vec<DimensionValue<D, C>> {
        self.data
    }
}

impl<D: PrimInt + Debug, C: Float + Debug> PartialEq for SpacePosition<D, C> {
    /// # Examples
    /// ```
    /// use gymnarium_base::{SpacePosition, DimensionValueI64F64};
    /// let space_position_a = SpacePosition::from(DimensionValueI64F64::discrete(10));
    /// assert_eq!(space_position_a, space_position_a);
    ///
    /// let space_position_b = SpacePosition::from(DimensionValueI64F64::continuous(3.5));
    /// assert_eq!(space_position_b, space_position_b);
    /// assert_ne!(space_position_b, space_position_a);
    ///
    /// let space_position_c = SpacePosition::from(vec![
    ///     DimensionValueI64F64::multiple(vec![
    ///         DimensionValueI64F64::discrete(5),
    ///         DimensionValueI64F64::continuous(3.4),
    ///     ]),
    ///     DimensionValueI64F64::discrete(7)
    /// ]);
    /// assert_eq!(space_position_c, space_position_c);
    /// assert_ne!(space_position_c, space_position_a);
    /// assert_ne!(space_position_c, space_position_b);
    ///
    /// let space_position_d = SpacePosition::from(DimensionValueI64F64::continuous(3.5));
    /// assert_eq!(space_position_d, space_position_d);
    /// assert_ne!(space_position_d, space_position_a);
    /// assert_eq!(space_position_d, space_position_b);
    /// assert_ne!(space_position_d, space_position_c);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            false
        } else {
            !self.data.iter().zip(other.data.iter()).any(|(s, o)| s != o)
        }
    }
}

/// Contains the ActionSpace and EnvironmentSpace of environments.
///
/// ## Examples
/// ### Single value
/// ```
/// # use gymnarium_base::{Space, DimensionBoundaries, DimensionBoundariesI64F64};
/// let single_discrete_value = Space::from(DimensionBoundariesI64F64::discrete_zero(2));
/// let single_continuous_value = Space::from(DimensionBoundariesI64F64::continuous(1.5, 3.7));
/// ```
/// ### Multiple values
/// ```
/// # use gymnarium_base::{Space, DimensionBoundaries, DimensionBoundariesI64F64};
/// let multiple_discrete_values = Space::from(vec![
///     DimensionBoundariesI64F64::discrete_zero(2),
///     DimensionBoundariesI64F64::discrete_zero(5)
/// ]);
/// let multiple_continuous_values = Space::from(vec![
///     DimensionBoundariesI64F64::continuous(1.5, 3.7),
///     DimensionBoundariesI64F64::continuous(1.5, 3.7)
/// ]);
/// let multiple_different_values = Space::from(vec![
///     DimensionBoundariesI64F64::continuous(1.5, 3.7),
///     DimensionBoundariesI64F64::discrete_zero(8),
///     DimensionBoundariesI64F64::continuous_zero(3.7)
/// ]);
/// ```
#[derive(Debug)]
pub struct Space<D: PrimInt + Debug, C: Float + Debug> {
    data: Vec<DimensionBoundaries<D, C>>,
}

impl<D: PrimInt + Debug, C: Float + Debug> Space<D, C> {
    /// Checks if this space shape matches the SpacePosition shape.
    /// # Examples
    /// ```
    /// # use gymnarium_base::{Space, DimensionBoundariesI64F64, SpacePosition, DimensionValueI64F64};
    /// let space_a = Space::from(DimensionBoundariesI64F64::discrete_zero(20));
    /// let space_b = Space::from(DimensionBoundariesI64F64::continuous(1.2, 1.8));
    /// let space_c = Space::from(vec![
    ///     DimensionBoundariesI64F64::multiple(vec![
    ///         DimensionBoundariesI64F64::continuous_zero(34.3),
    ///         DimensionBoundariesI64F64::continuous(1.0, 1.1)
    ///     ]),
    ///     DimensionBoundariesI64F64::discrete_zero(4)
    /// ]);
    /// let space_d = Space::from(DimensionBoundariesI64F64::discrete(0, 1));
    ///
    /// let position_a = SpacePosition::from(DimensionValueI64F64::discrete(300));
    /// assert!(space_a.matches_shape_for(&position_a));
    /// assert!(!space_b.matches_shape_for(&position_a));
    /// assert!(!space_c.matches_shape_for(&position_a));
    /// assert!(space_d.matches_shape_for(&position_a));
    ///
    /// let position_b = SpacePosition::from(DimensionValueI64F64::continuous(-20.5));
    /// assert!(!space_a.matches_shape_for(&position_b));
    /// assert!(space_b.matches_shape_for(&position_b));
    /// assert!(!space_c.matches_shape_for(&position_b));
    /// assert!(!space_d.matches_shape_for(&position_b));
    ///
    /// let position_c = SpacePosition::from(vec![
    ///     DimensionValueI64F64::multiple(vec![
    ///         DimensionValueI64F64::continuous(99.9),
    ///         DimensionValueI64F64::continuous(-5.3)
    ///     ]),
    ///     DimensionValueI64F64::discrete(7)
    /// ]);
    /// assert!(!space_a.matches_shape_for(&position_c));
    /// assert!(!space_b.matches_shape_for(&position_c));
    /// assert!(space_c.matches_shape_for(&position_c));
    /// assert!(!space_d.matches_shape_for(&position_c));
    /// ```
    pub fn matches_shape_for(&self, position: &SpacePosition<D, C>) -> bool {
        if self.data.len() != position.data.len() {
            false
        } else {
            !self
                .data
                .iter()
                .zip(position.data.iter())
                .any(|(s, o)| !s.matches_shape_for(o))
        }
    }

    /// Checks if this space shape matches the other one.
    /// # Examples
    /// ```
    /// # use gymnarium_base::{Space, DimensionBoundariesI64F64};
    /// let space_a = Space::from(DimensionBoundariesI64F64::discrete_zero(20));
    /// assert!(space_a.matches_shape_of(&space_a));
    ///
    /// let space_b = Space::from(DimensionBoundariesI64F64::continuous(1.2, 1.8));
    /// assert!(space_b.matches_shape_of(&space_b));
    /// assert!(!space_b.matches_shape_of(&space_a));
    ///
    /// let space_c = Space::from(vec![
    ///     DimensionBoundariesI64F64::multiple(vec![
    ///         DimensionBoundariesI64F64::continuous_zero(34.3),
    ///         DimensionBoundariesI64F64::continuous(1.0, 1.1)
    ///     ]),
    ///     DimensionBoundariesI64F64::discrete_zero(4)
    /// ]);
    /// assert!(space_c.matches_shape_of(&space_c));
    /// assert!(!space_c.matches_shape_of(&space_a));
    /// assert!(!space_c.matches_shape_of(&space_b));
    ///
    /// let space_d = Space::from(DimensionBoundariesI64F64::discrete(0, 1));
    /// assert!(space_d.matches_shape_of(&space_d));
    /// assert!(space_d.matches_shape_of(&space_a));
    /// assert!(!space_d.matches_shape_of(&space_b));
    /// assert!(!space_d.matches_shape_of(&space_c));
    /// ```
    pub fn matches_shape_of(&self, other: &Space<D, C>) -> bool {
        if self.data.len() != other.data.len() {
            false
        } else {
            !self
                .data
                .iter()
                .zip(other.data.iter())
                .any(|(s, o)| !s.matches_shape_of(o))
        }
    }

    /// Checks if the position is contained within this space.
    /// # Examples
    /// ```
    /// # use gymnarium_base::{Space, DimensionBoundariesI64F64, SpacePosition, DimensionValueI64F64};
    /// let space_a = Space::from(DimensionBoundariesI64F64::discrete_zero(10));
    ///
    /// let position_a = SpacePosition::from(DimensionValueI64F64::discrete(8));
    /// assert!(space_a.contains(&position_a));
    ///
    /// let position_b = SpacePosition::from(DimensionValueI64F64::discrete(12));
    /// assert!(!space_a.contains(&position_b));
    ///
    /// let position_c = SpacePosition::from(DimensionValueI64F64::continuous(8.5));
    /// assert!(!space_a.contains(&position_c));
    ///
    ///
    /// let space_b = Space::from(vec![
    ///     DimensionBoundariesI64F64::discrete_zero(20),
    ///     DimensionBoundariesI64F64::continuous_zero(90.0),
    ///     DimensionBoundariesI64F64::multiple(vec![
    ///         DimensionBoundariesI64F64::discrete(5, 10),
    ///         DimensionBoundariesI64F64::continuous(5.5, 6.5)
    ///     ])
    /// ]);
    /// assert!(!space_b.contains(&position_a));
    /// assert!(!space_b.contains(&position_b));
    /// assert!(!space_b.contains(&position_c));
    ///
    /// let position_d = SpacePosition::from(vec![
    ///     DimensionValueI64F64::discrete(8),
    ///     DimensionValueI64F64::continuous(8.9)
    /// ]);
    /// assert!(!space_b.contains(&position_d));
    ///
    /// let position_e = SpacePosition::from(vec![
    ///     DimensionValueI64F64::discrete(8),
    ///     DimensionValueI64F64::continuous(8.9),
    ///     DimensionValueI64F64::multiple(vec![
    ///         DimensionValueI64F64::discrete(7),
    ///         DimensionValueI64F64::continuous(6.0),
    ///     ])
    /// ]);
    /// assert!(space_b.contains(&position_e));
    /// ```
    pub fn contains(&self, position: &SpacePosition<D, C>) -> bool {
        if self.data.len() != position.data.len() {
            false
        } else {
            !self
                .data
                .iter()
                .zip(position.data.iter())
                .any(|(s, p)| !s.contains(p))
        }
    }

    /// Returns a sample position within this space.
    /// # Examples
    /// ```
    /// # use gymnarium_base::{DimensionBoundariesI64F64, Space, SpacePosition, DimensionValueI64F64};
    /// let discrete_space = Space::from(DimensionBoundariesI64F64::discrete_zero(10));
    /// let sampled_value = discrete_space.sample(&|min, max| 5, &|min, max| 0f64);
    /// assert_eq!(SpacePosition::from(DimensionValueI64F64::discrete(5)), sampled_value);
    ///
    /// let continuous_space = Space::from(DimensionBoundariesI64F64::continuous_zero(2.5));
    /// let sampled_value = continuous_space.sample(&|min, max| 0, &|min, max| 1.7f64);
    /// assert_eq!(SpacePosition::from(DimensionValueI64F64::continuous(1.7)), sampled_value);
    ///
    /// let multiple_space = Space::from(DimensionBoundariesI64F64::multiple(vec![
    ///     DimensionBoundariesI64F64::discrete_zero(20),
    ///     DimensionBoundariesI64F64::continuous_zero(4.8f64),
    ///     DimensionBoundariesI64F64::multiple(vec![
    ///         DimensionBoundariesI64F64::discrete(5, 7),
    ///         DimensionBoundariesI64F64::continuous(-1.2, -0.2)
    ///     ]),
    /// ]));
    /// let sampled_value = multiple_space.sample(&|min, max| ((max-min)/2)+min, &|min, max| ((max-min)/2f64)+min);
    /// assert_eq!(SpacePosition::from(DimensionValueI64F64::MULTIPLE(vec![
    ///     DimensionValueI64F64::discrete(10),
    ///     DimensionValueI64F64::continuous(2.4f64),
    ///     DimensionValueI64F64::multiple(vec![
    ///         DimensionValueI64F64::discrete(6),
    ///         DimensionValueI64F64::continuous(-0.7)
    ///     ])
    /// ])), sampled_value);
    /// ```
    pub fn sample<FD: Fn(&D, &D) -> D, FC: Fn(&C, &C) -> C>(
        &self,
        discrete_random_provider: &FD,
        continuous_random_provider: &FC,
    ) -> SpacePosition<D, C> {
        SpacePosition::from(
            self.data
                .iter()
                .map(|b| b.sample(discrete_random_provider, continuous_random_provider))
                .collect::<Vec<DimensionValue<D, C>>>(),
        )
    }
}

impl<D: PrimInt + Debug, C: Float + Debug> From<DimensionBoundaries<D, C>> for Space<D, C> {
    fn from(data: DimensionBoundaries<D, C>) -> Self {
        Self { data: vec![data] }
    }
}

impl<D: PrimInt + Debug, C: Float + Debug> From<Vec<DimensionBoundaries<D, C>>> for Space<D, C> {
    fn from(data: Vec<DimensionBoundaries<D, C>>) -> Self {
        Self { data }
    }
}

impl<D: PrimInt + Debug, C: Float + Debug> Into<Vec<DimensionBoundaries<D, C>>> for Space<D, C> {
    fn into(self) -> Vec<DimensionBoundaries<D, C>> {
        self.data
    }
}

impl<D: PrimInt + Debug, C: Float + Debug> PartialEq for Space<D, C> {
    /// # Examples
    /// ```
    /// use gymnarium_base::{Space, DimensionBoundariesI64F64};
    /// let space_position_a = Space::from(DimensionBoundariesI64F64::discrete_zero(10));
    /// assert_eq!(space_position_a, space_position_a);
    ///
    /// let space_position_b = Space::from(DimensionBoundariesI64F64::continuous_zero(3.5));
    /// assert_eq!(space_position_b, space_position_b);
    /// assert_ne!(space_position_b, space_position_a);
    ///
    /// let space_position_c = Space::from(vec![
    ///     DimensionBoundariesI64F64::multiple(vec![
    ///         DimensionBoundariesI64F64::discrete_zero(5),
    ///         DimensionBoundariesI64F64::continuous_zero(3.4),
    ///     ]),
    ///     DimensionBoundariesI64F64::discrete_zero(7)
    /// ]);
    /// assert_eq!(space_position_c, space_position_c);
    /// assert_ne!(space_position_c, space_position_a);
    /// assert_ne!(space_position_c, space_position_b);
    ///
    /// let space_position_d = Space::from(DimensionBoundariesI64F64::continuous_zero(3.5));
    /// assert_eq!(space_position_d, space_position_d);
    /// assert_ne!(space_position_d, space_position_a);
    /// assert_eq!(space_position_d, space_position_b);
    /// assert_ne!(space_position_d, space_position_c);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            false
        } else {
            !self.data.iter().zip(other.data.iter()).any(|(s, o)| s != o)
        }
    }
}

/// List of dimension boundaries for the observable environment state.
pub type ObservationSpace<D, C> = Space<D, C>;

/// List of dimension values as the observable environment state.
pub type EnvironmentState<D, C> = SpacePosition<D, C>;

/// List of dimension values as the agent action.
pub type AgentAction<D, C> = SpacePosition<D, C>;

/// List of dimension boundaries for available environment agent actions.
pub type ActionSpace<D, C> = Space<D, C>;

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
pub trait Environment<E: std::error::Error, I: Debug, D: PrimInt + Debug, C: Float + Debug> {
    /// Returns the available boundaries for the actions for this environment.
    fn action_space(&self) -> ActionSpace<D, C>;

    /// Returns the boundaries for the observable states for this environment.
    fn observation_space(&self) -> ObservationSpace<D, C>;

    /// Returns the suggested episode step count if the environment provides one.
    fn suggested_episode_steps_count(&self) -> Option<u128>;

    /// Resets the state and initial resources of the environment and returns the initial state.
    ///
    /// Should be called even before the first step is done.
    /// Otherwise there might be no or an invalid state.
    ///
    /// Optionally a seed can be given to initialise the internal random number generator.
    fn reset(&mut self, random_seed: Option<Seed>) -> Result<EnvironmentState<D, C>, E>;

    /// Performs a step within this environment with the given agent action
    fn step(
        &mut self,
        action: &AgentAction<D, C>,
    ) -> Result<(EnvironmentState<D, C>, f64, bool, I), E>;

    /// Cleans up resources of this environment.
    ///
    /// Should be called at the very end of usage.
    fn close(&mut self) -> Result<(), E>;
}

/// Base trait for an agent.
pub trait Agent<E: std::error::Error, D: PrimInt + Debug, C: Float + Debug> {
    /// Resets the state and initial resources of the agent.
    ///
    /// Should be called even before the first step is done.
    /// Otherwise the agent could be in an invalid state.
    ///
    /// Optionally a seed can be given to initialise the internal random number generator.
    fn reset(&mut self, random_seed: Option<Seed>) -> Result<(), E>;

    /// Returns an action based on the environment state given.
    fn choose_action(&mut self, state: &EnvironmentState<D, C>) -> Result<AgentAction<D, C>, E>;

    /// Lets this agent process the result of the last step.
    fn process_reward(
        &mut self,
        old_state: &EnvironmentState<D, C>,
        new_state: &EnvironmentState<D, C>,
        reward: f64,
        is_done: bool,
    ) -> Result<(), E>;

    /// Cleans up resources of this agent.
    ///
    /// Should be called at the very end of usage.
    fn close(&mut self) -> Result<(), E>;
}
