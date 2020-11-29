//! Contains structures for defining dimensions needed for gymnarium.
//!
//! Primarily `Space` and `Position` are relevant.
//! These are the working bits for `Environment` and `Agent`.
//!
//! ## Example: Nintendo GameBoy
//!
//! **For example** the input space for the first Nintendo GameBoy can be defined as:
//! `(Discrete(0 to 4), Discrete(0 or 1), Discrete(0 or 1))`.
//! Where the first `Discrete` are the values for the **D-pad** and the second and third `Discrete`
//! are the pressed state for the **A** and **B** button.
//!
//! This can be constructed as follows:
//! ```
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//! use gymnarium_base::Seed;
//! use gymnarium_base::space::{Space, Position, DimensionBoundaries, DimensionValue};
//!
//! let gameboy_input_space = Space::simple(vec![
//!     DimensionBoundaries::from(4),
//!     DimensionBoundaries::from(1),
//!     DimensionBoundaries::from(1)
//! ]);
//!
//! let mut rng = StdRng::from_seed(Seed::from("gymnarium").into());
//! let gameboy_input = gameboy_input_space.sample_with(&mut rng);
//!
//! assert_eq!(
//!     Position::simple(vec![
//!         DimensionValue::from(2),
//!         DimensionValue::from(1),
//!         DimensionValue::from(1),
//!     ]),
//!     gameboy_input
//! );
//! ```
//!
//! ## Example: RAM
//! **Another example** is the content of RAM which can be defined as:
//! `[Discrete(0 to 255)]`.
//! This is like RAM a one dimensional list of n cells with 8 bit of data which can go from 0 to 255.
//!
//! This can be constructed as follows:
//! ```
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//! use gymnarium_base::Seed;
//! use gymnarium_base::space::{Space, Position, DimensionBoundaries, DimensionValue};
//!
//! let ram_space = Space::simple_all(DimensionBoundaries::from(255), 4);
//!
//! let mut rng = StdRng::from_seed(Seed::from("gymnarium").into());
//! let ram_output = ram_space.sample_with(&mut rng);
//!
//! assert_eq!(
//!     Position::simple(vec![
//!         DimensionValue::from(151),
//!         DimensionValue::from(180),
//!         DimensionValue::from(231),
//!         DimensionValue::from(105)
//!     ]),
//!     ram_output
//! );
//! ```
//!
//! ## Example: Image / Positional Data
//!
//! **The next example** is the dealing with image data or positional data, which can be defined as three dimensions but with same type in each dimension.
//! Like 8 bit of image data per pixel for grayscale or 24 bit / 32 bit for rgb or rgba pixels.
//! 24 bit or 32 bit can be represented as single u32 or as Vec<u8>.
//!
//! A image with rgb and 2 pixels by 2 pixels can be described like:
//! ```
//! use gymnarium_base::space::{Space, Position, DimensionBoundaries, DimensionValue};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//! use gymnarium_base::Seed;
//! use std::iter::repeat;
//!
//! let image_space = Space::all(DimensionBoundaries::from(255), vec![2, 2, 3]);
//!
//! let mut rng = StdRng::from_seed(Seed::from("gymnarium").into());
//! let image_output = image_space.sample_with(&mut rng);
//!
//! assert_eq!(
//!   Position::new(
//!     vec![
//!       DimensionValue::from(151),DimensionValue::from(180),DimensionValue::from(231),
//!       DimensionValue::from(105),DimensionValue::from(23),DimensionValue::from(197),
//!       DimensionValue::from(210),DimensionValue::from(119),DimensionValue::from(111),
//!       DimensionValue::from(179),DimensionValue::from(245),DimensionValue::from(40)
//!     ],
//!     vec![2, 2, 3]
//!   ).unwrap(),
//!   image_output
//! );
//! ```
//!
//! ## Example: Irregular Structure
//!
//! Until now every structure has been either one-dimensional or three-dimensional.
//! But what if following has to be defined:
//!
//! ```json
//! {
//!   'sensors': {
//!     'position': Space::all(DimensionBoundaries::from(-100f32..=100f32), vec![3]),
//!     'velocity': Space::all(DimensionBoundaries::from(-1f32..=1f32), vec![3]),
//!     'front_cam': (
//!       Space::all(DimensionBoundaries::from(1f32), vec![10, 10, 3]),
//!       Space::all(DimensionBoundaries::from(1f32), vec![10, 10, 3])
//!     ),
//!     'rear_cam': Space::all(DimensionBoundaries::from(1f32), vec![10, 10, 3])
//!   },
//!   'ext_controller': Space::simple(vec![
//!     DimensionBoundaries::from(4), DimensionBoundaries::from(1), DimensionBoundaries::from(1)
//!   ]),
//!   'inner_state': {
//!     'charge': Space::simple(vec![DimensionBoundaries::from(99)]),
//!     'system_checks': Space::simple_all(DimensionBoundaries::from(1), vec![10]),
//!     'job_status: {
//!       'task': Space::simple(vec![DimensionBoundaries::from(4)]),
//!       'progress': Space::simple(vec![DimensionBoundaries::from(100f32)]),
//!     },
//!   },
//! }
//! ```
//!
//! *(Source: <https://github.com/openai/gym/blob/master/gym/spaces/dict.py>)*
//!
//! This structure can be defined with a format like:
//!
//! ```
//! use rand::rngs::StdRng;
//! use rand::SeedableRng;
//! use gymnarium_base::Seed;
//! use gymnarium_base::space::{Format, Space, DimensionBoundaries, Position, DimensionValue};
//!
//! // Define the structure:
//! let mut format = Format::default();
//! format.add("sensors.position".to_string(), vec![3]);
//! format.add("sensors.velocity".to_string(), vec![3]);
//! format.add("sensors.front_cam.0".to_string(), vec![10, 10, 3]);
//! format.add("sensors.front_cam.1".to_string(), vec![10, 10, 3]);
//! format.add("sensors.rear_cam".to_string(), vec![10, 10, 3]);
//! format.add("ext_controller".to_string(), vec![3]);
//! format.add("inner_state.charge".to_string(), vec![1]);
//! format.add("inner_state.system_checks".to_string(), vec![10]);
//! format.add("inner_state.job_status.task".to_string(), vec![1]);
//! format.add("inner_state.job_status.progress".to_string(), vec![1]);
//!
//! assert_eq!(
//!     format.shape_of(&"sensors.position".to_string()),
//!     Some(&vec![3])
//! );
//!
//! // create a new space with this format:
//! let mut space = format.new_space();
//!
//! println!("{:?}", space);
//!
//! // Set the subspace for each substructure:
//! format.set_subspace(
//!     &mut space,
//!     &"sensors.position".to_string(),
//!     Space::all(DimensionBoundaries::from(-100f32..=100f32), vec![3])
//! ).unwrap();
//! // ...
//! format.set_subspace(
//!     &mut space,
//!     &"ext_controller".to_string(),
//!     Space::simple(vec![
//!         DimensionBoundaries::from(4), DimensionBoundaries::from(1), DimensionBoundaries::from(1)
//!     ])
//! ).unwrap();
//! // ...
//!
//! println!("{:?}", space);
//!
//! assert_eq!(
//!     format.get_subspace(&space, &"sensors.position".to_string()),
//!     Ok(Space::all(DimensionBoundaries::from(-100f32..=100f32), vec![3]))
//! );
//!
//! // Generate a sample from this space:
//! let mut rng = StdRng::from_seed(Seed::from("gymnarium").into());
//! let position = space.sample_with(&mut rng);
//!
//! assert_eq!(
//!     format.get_subposition(&position, &"sensors.position".to_string()),
//!     Ok(Position::simple(vec![
//!         DimensionValue::FLOAT(18.130913f32),
//!         DimensionValue::FLOAT(40.79979f32),
//!         DimensionValue::FLOAT(80.73178f32)
//!     ]))
//! );
//!
//! // Get a single value from this position:
//! assert_eq!(
//!     format.get_value(&position, &"ext_controller".to_string(), &vec![2]),
//!     Ok(&DimensionValue::INTEGER(0))
//! );
//! ```
//!

use std::collections::HashMap;

use std::ops::{Index, IndexMut, RangeInclusive};

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

/* --- --- --- INDEX --- --- --- */

/// Calculates the index inside a n-dimensional Vec stored inside a one-dimensional Vec.
fn calculate_index(shape: &[usize], index: &[usize]) -> Result<usize, SpaceError> {
    if index.iter().zip(shape.iter()).any(|(a, b)| a >= b) {
        Err(SpaceError::IndexOutOfBounds)
    } else if index.is_empty() {
        Ok(0)
    } else {
        let mut output_index = index[0];
        for i in 1..index.len() {
            output_index += index[i] * shape[i - 1];
        }
        Ok(output_index)
    }
}

/* --- --- --- SPACE ERROR --- --- --- */

/// General errors for this module.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SpaceError {
    GivenDimensionsDoNotMatch,
    IndexOutOfBounds,
}

impl std::fmt::Display for SpaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GivenDimensionsDoNotMatch => write!(f, "Given dimensions do not match"),
            Self::IndexOutOfBounds => write!(f, "Given index is out of bounds"),
        }
    }
}
impl std::error::Error for SpaceError {}

/* --- --- --- FORMAT --- --- --- */

/// Specific errors applicable to the Format structure.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FormatError {
    KeyAlreadyExistsInFormat(String),
    KeyNotFoundInFormat(String),
    SpaceCreationError(SpaceError),
    PositionCreationError(SpaceError),
    GivenSpaceDoesNotFit { needed: usize, given: usize },
    SpaceIndexError(SpaceError),
    PositionIndexError(SpaceError),
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyAlreadyExistsInFormat(key) => {
                write!(f, "Key \"{}\" has already been added to this format", key)
            }
            Self::KeyNotFoundInFormat(key) => write!(f, "Key \"{}\" not found in format", key),
            Self::SpaceCreationError(space_error) => write!(
                f,
                "Space Error \"{}\" occurred while creation of space",
                space_error
            ),
            Self::PositionCreationError(space_error) => write!(
                f,
                "Space Error \"{}\" occurred while creation of position",
                space_error
            ),
            Self::GivenSpaceDoesNotFit { needed, given } => write!(
                f,
                "Given space ({}) does not fit needed ({})",
                given, needed
            ),
            Self::SpaceIndexError(space_error) => write!(
                f,
                "Space Error \"{}\" occurred while indexing of space",
                space_error
            ),
            Self::PositionIndexError(space_error) => write!(
                f,
                "Space Error \"{}\" occurred while indexing of position",
                space_error
            ),
        }
    }
}

impl std::error::Error for FormatError {}

struct SubFormat {
    offset: usize,
    shape: Vec<usize>,
    length: usize,
}

impl SubFormat {
    fn new(offset: usize, shape: Vec<usize>) -> Self {
        let length = shape.iter().product();
        Self {
            offset,
            shape,
            length,
        }
    }
}

/// Structure to define irregular structures (Read the bottom of the module description).
#[derive(Default)]
pub struct Format {
    v: HashMap<String, SubFormat>,
    length: usize,
}

impl Format {
    /* Allowing clippy::map_entry, because if the key exists a error should be returned.
     * Map_entry does not provide such way or I didn't see that.
     */
    #[allow(clippy::map_entry)]
    pub fn add(&mut self, key: String, shape: Vec<usize>) -> Result<(), FormatError> {
        if self.v.contains_key(&key) {
            Err(FormatError::KeyAlreadyExistsInFormat(key))
        } else {
            let sub_format = SubFormat::new(self.length, shape);
            self.length += sub_format.length;
            self.v.insert(key, sub_format);
            Ok(())
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.v.contains_key(key)
    }

    pub fn shape_of(&self, key: &str) -> Option<&Vec<usize>> {
        self.v.get(key).map(|sf| &sf.shape)
    }

    pub fn keys(&self) -> Vec<&String> {
        self.v.keys().collect()
    }

    pub fn new_space(&self) -> Space {
        Space::simple_all(DimensionBoundaries::INTEGER(0, 0), self.length)
    }

    pub fn get_subspace(&self, space: &Space, key: &str) -> Result<Space, FormatError> {
        let subformat = self.v.get(key);
        if let Some(sf) = subformat {
            let mut space_values = Vec::new();
            for index in sf.offset..(sf.offset + sf.length) {
                space_values.push(space.boundaries[index]);
            }
            Space::new(space_values, sf.shape.clone()).map_err(FormatError::SpaceCreationError)
        } else {
            Err(FormatError::KeyNotFoundInFormat(key.to_string()))
        }
    }

    pub fn set_subspace(
        &self,
        space: &mut Space,
        key: &str,
        subspace: Space,
    ) -> Result<(), FormatError> {
        let subformat = self.v.get(key);
        if let Some(sf) = subformat {
            if subspace.boundaries.len() == sf.length {
                for index in 0..sf.length {
                    space.boundaries[sf.offset + index] = subspace.boundaries[index];
                }
                Ok(())
            } else {
                Err(FormatError::GivenSpaceDoesNotFit {
                    needed: sf.length,
                    given: subspace.boundaries.len(),
                })
            }
        } else {
            Err(FormatError::KeyNotFoundInFormat(key.to_string()))
        }
    }

    pub fn get_boundaries<'a>(
        &self,
        space: &'a Space,
        key: &str,
        index: &[usize],
    ) -> Result<&'a DimensionBoundaries, FormatError> {
        let subformat = self.v.get(key);
        if let Some(sf) = subformat {
            let current_index =
                calculate_index(&sf.shape, index).map_err(FormatError::SpaceIndexError)?;
            Ok(&space.boundaries[sf.offset + current_index])
        } else {
            Err(FormatError::KeyNotFoundInFormat(key.to_string()))
        }
    }

    pub fn set_boundaries(
        &self,
        space: &mut Space,
        key: &str,
        index: &[usize],
        boundaries: DimensionBoundaries,
    ) -> Result<(), FormatError> {
        let subformat = self.v.get(key);
        if let Some(sf) = subformat {
            let current_index =
                calculate_index(&sf.shape, index).map_err(FormatError::SpaceIndexError)?;
            space.boundaries[sf.offset + current_index] = boundaries;
            Ok(())
        } else {
            Err(FormatError::KeyNotFoundInFormat(key.to_string()))
        }
    }

    pub fn get_subposition(&self, position: &Position, key: &str) -> Result<Position, FormatError> {
        let subformat = self.v.get(key);
        if let Some(sf) = subformat {
            let mut position_values = Vec::new();
            for index in sf.offset..(sf.offset + sf.length) {
                position_values.push(position.values[index]);
            }
            Position::new(position_values, sf.shape.clone())
                .map_err(FormatError::PositionCreationError)
        } else {
            Err(FormatError::KeyNotFoundInFormat(key.to_string()))
        }
    }

    pub fn set_subposition(
        &self,
        position: &mut Position,
        key: &str,
        subposition: Position,
    ) -> Result<(), FormatError> {
        let subformat = self.v.get(key);
        if let Some(sf) = subformat {
            if subposition.values.len() == sf.length {
                for index in 0..sf.length {
                    position.values[sf.offset + index] = subposition.values[index];
                }
                Ok(())
            } else {
                Err(FormatError::GivenSpaceDoesNotFit {
                    given: subposition.values.len(),
                    needed: sf.length,
                })
            }
        } else {
            Err(FormatError::KeyNotFoundInFormat(key.to_string()))
        }
    }

    pub fn get_value<'a>(
        &self,
        position: &'a Position,
        key: &str,
        index: &[usize],
    ) -> Result<&'a DimensionValue, FormatError> {
        let subformat = self.v.get(key);
        if let Some(sf) = subformat {
            let current_index =
                calculate_index(&sf.shape, index).map_err(FormatError::PositionIndexError)?;
            Ok(&position.values[sf.offset + current_index])
        } else {
            Err(FormatError::KeyNotFoundInFormat(key.to_string()))
        }
    }

    pub fn set_value(
        &self,
        position: &mut Position,
        key: &str,
        index: &[usize],
        value: DimensionValue,
    ) -> Result<(), FormatError> {
        let subformat = self.v.get(key);
        if let Some(sf) = subformat {
            let current_index =
                calculate_index(&sf.shape, index).map_err(FormatError::PositionIndexError)?;
            position.values[sf.offset + current_index] = value;
            Ok(())
        } else {
            Err(FormatError::KeyNotFoundInFormat(key.to_string()))
        }
    }
}

/* --- --- --- SPACE --- --- --- */

/// Defines a space in which states or positions can be placed.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Space {
    boundaries: Vec<DimensionBoundaries>,
    dimensions: Vec<usize>,
}

impl Space {
    pub fn new(
        dimension_boundaries: Vec<DimensionBoundaries>,
        dimensions: Vec<usize>,
    ) -> Result<Self, SpaceError> {
        if dimensions.iter().product::<usize>() == dimension_boundaries.len() {
            Ok(Self {
                boundaries: dimension_boundaries,
                dimensions,
            })
        } else {
            Err(SpaceError::GivenDimensionsDoNotMatch)
        }
    }

    pub fn all(dimension_boundaries: DimensionBoundaries, dimensions: Vec<usize>) -> Self {
        Self {
            boundaries: vec![dimension_boundaries; dimensions.iter().product()],
            dimensions,
        }
    }

    pub fn simple(dimension_boundaries: Vec<DimensionBoundaries>) -> Self {
        let length = dimension_boundaries.len();
        Self {
            boundaries: dimension_boundaries,
            dimensions: vec![length],
        }
    }

    pub fn simple_all(dimension_boundaries: DimensionBoundaries, times: usize) -> Self {
        Self {
            boundaries: vec![dimension_boundaries; times],
            dimensions: vec![times],
        }
    }

    pub fn dimensions(&self) -> &Vec<usize> {
        &self.dimensions
    }

    pub fn get_boundary(&self, index: &[usize]) -> &DimensionBoundaries {
        &self[index]
    }

    pub fn set_boundary(&mut self, index: &[usize], boundary: DimensionBoundaries) {
        self[index] = boundary;
    }

    pub fn get_boundaries(&self) -> &[DimensionBoundaries] {
        &self.boundaries
    }

    pub fn sample(&self) -> Position {
        Position {
            values: self
                .boundaries
                .iter()
                .map(|boundaries| boundaries.sample())
                .collect(),
            dimensions: self.dimensions.clone(),
        }
    }

    pub fn sample_with<R: Rng + ?Sized>(&self, rng: &mut R) -> Position {
        Position {
            values: self
                .boundaries
                .iter()
                .map(|boundaries| boundaries.sample_with(rng))
                .collect(),
            dimensions: self.dimensions.clone(),
        }
    }

    pub fn matches(&self, other: &Space) -> bool {
        self.dimensions == other.dimensions
            && self
                .boundaries
                .iter()
                .zip(other.boundaries.iter())
                .any(|(a, b)| !a.matches(b))
    }

    pub fn contains(&self, other: &Position) -> bool {
        self.dimensions == other.dimensions
            && self
                .boundaries
                .iter()
                .zip(other.values.iter())
                .any(|(boundaries, value)| boundaries.contains(value))
    }
}

impl Index<&[usize]> for Space {
    type Output = DimensionBoundaries;

    fn index(&self, index: &[usize]) -> &Self::Output {
        let index = calculate_index(self.dimensions(), index)
            .unwrap_or_else(|e| panic!(
                "Could not calculate inner index for position with dimensions {:?} and given index {:?} (cause: {})",
                self.dimensions, index, e
            ));
        &self.boundaries[index]
    }
}

impl IndexMut<&[usize]> for Space {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let index = calculate_index(self.dimensions(), index)
            .unwrap_or_else(|e| panic!(
                "Could not calculate inner index for position with dimensions {:?} and given index {:?} (cause: {})",
                self.dimensions, index, e
            ));
        &mut self.boundaries[index]
    }
}

/* --- --- --- POSITION --- --- --- */

/// Defines the state or position inside a space.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Position {
    values: Vec<DimensionValue>,
    dimensions: Vec<usize>,
}

impl Position {
    pub fn new(
        dimension_values: Vec<DimensionValue>,
        dimensions: Vec<usize>,
    ) -> Result<Self, SpaceError> {
        if dimensions.iter().product::<usize>() == dimension_values.len() {
            Ok(Self {
                values: dimension_values,
                dimensions,
            })
        } else {
            Err(SpaceError::GivenDimensionsDoNotMatch)
        }
    }

    pub fn all(dimension_value: DimensionValue, dimensions: Vec<usize>) -> Self {
        Self {
            values: vec![dimension_value; dimensions.iter().product()],
            dimensions,
        }
    }

    pub fn simple(dimension_values: Vec<DimensionValue>) -> Self {
        let dimension_values_length = dimension_values.len();
        Self {
            values: dimension_values,
            dimensions: vec![dimension_values_length],
        }
    }

    pub fn simple_all(dimension_value: DimensionValue, times: usize) -> Self {
        Self {
            values: vec![dimension_value; times],
            dimensions: vec![times],
        }
    }

    pub fn dimensions(&self) -> &Vec<usize> {
        &self.dimensions
    }

    pub fn matches(&self, other: &Position) -> bool {
        self.dimensions == other.dimensions
            && self
                .values
                .iter()
                .zip(other.values.iter())
                .any(|(a, b)| !a.matches(b))
    }

    pub fn get_value(&self, index: &[usize]) -> &DimensionValue {
        &self[index]
    }

    pub fn set_value(&mut self, index: &[usize], value: DimensionValue) {
        self[index] = value;
    }

    pub fn get_values(&self) -> &[DimensionValue] {
        &self.values
    }
}

impl Index<&[usize]> for Position {
    type Output = DimensionValue;

    fn index(&self, index: &[usize]) -> &Self::Output {
        let index = calculate_index(self.dimensions(), index)
            .unwrap_or_else(|e| panic!(
                "Could not calculate inner index for position with dimensions {:?} and given index {:?} (cause: {})",
                self.dimensions, index, e
            ));
        &self.values[index]
    }
}

impl IndexMut<&[usize]> for Position {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let index = calculate_index(self.dimensions(), index)
            .unwrap_or_else(|e| panic!(
                "Could not calculate inner index for position with dimensions {:?} and given index {:?} (cause: {})",
                self.dimensions, index, e
            ));
        &mut self.values[index]
    }
}

/* --- --- --- DIMENSION BOUNDARIES --- --- --- */

/// The inclusive upper and inclusive lower bound of a dimension.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DimensionBoundaries {
    INTEGER(i32, i32),
    FLOAT(f32, f32),
}

impl DimensionBoundaries {
    pub fn sample(&self) -> DimensionValue {
        self.sample_with(&mut rand::thread_rng())
    }

    pub fn sample_with<R: Rng + ?Sized>(&self, rng: &mut R) -> DimensionValue {
        match self {
            Self::INTEGER(min, max) => {
                DimensionValue::INTEGER(Uniform::new_inclusive(min, max).sample(rng))
            }
            Self::FLOAT(min, max) => {
                DimensionValue::FLOAT(Uniform::new_inclusive(min, max).sample(rng))
            }
        }
    }

    pub fn matches(&self, value: &DimensionBoundaries) -> bool {
        match self {
            Self::INTEGER(_, _) => match value {
                Self::INTEGER(_, _) => true,
                Self::FLOAT(_, _) => false,
            },
            Self::FLOAT(_, _) => match value {
                Self::INTEGER(_, _) => false,
                Self::FLOAT(_, _) => true,
            },
        }
    }

    pub fn contains(&self, value: &DimensionValue) -> bool {
        match self {
            Self::INTEGER(min, max) => match value {
                DimensionValue::INTEGER(val) => *min <= *val && *val <= *max,
                DimensionValue::FLOAT(_) => false,
            },
            Self::FLOAT(min, max) => match value {
                DimensionValue::INTEGER(_) => false,
                DimensionValue::FLOAT(val) => *min <= *val && *val <= *max,
            },
        }
    }

    pub fn expect_integer(&self) -> (i32, i32) {
        if let Self::INTEGER(start, end) = self {
            (*start, *end)
        } else {
            panic!("{:?} is not INTEGER as expected", self);
        }
    }

    pub fn expect_float(&self) -> (f32, f32) {
        if let Self::FLOAT(start, end) = self {
            (*start, *end)
        } else {
            panic!("{:?} is not FLOAT as expected", self);
        }
    }
}

/* i32 */

impl From<i32> for DimensionBoundaries {
    fn from(value: i32) -> Self {
        Self::INTEGER(0.min(value), 0.max(value))
    }
}

impl From<RangeInclusive<i32>> for DimensionBoundaries {
    fn from(range_inclusive: RangeInclusive<i32>) -> Self {
        Self::INTEGER(
            (*(range_inclusive.start())).min(*(range_inclusive.end())),
            (*(range_inclusive.start())).max(*(range_inclusive.end())),
        )
    }
}

/* f32 */

impl From<f32> for DimensionBoundaries {
    fn from(value: f32) -> Self {
        Self::FLOAT(0f32.min(value), 0f32.max(value))
    }
}

impl From<RangeInclusive<f32>> for DimensionBoundaries {
    fn from(range_inclusive: RangeInclusive<f32>) -> Self {
        Self::FLOAT(
            (*(range_inclusive.start())).min(*(range_inclusive.end())),
            (*(range_inclusive.start())).max(*(range_inclusive.end())),
        )
    }
}

/* --- --- --- DIMENSION VALUE --- --- --- */

/// A value inside a dimension.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DimensionValue {
    INTEGER(i32),
    FLOAT(f32),
}

impl DimensionValue {
    pub fn matches(&self, value: &DimensionValue) -> bool {
        match self {
            Self::INTEGER(_) => match value {
                Self::INTEGER(_) => true,
                Self::FLOAT(_) => false,
            },
            Self::FLOAT(_) => match value {
                Self::INTEGER(_) => false,
                Self::FLOAT(_) => true,
            },
        }
    }

    pub fn expect_integer(&self) -> i32 {
        if let Self::INTEGER(value) = self {
            *value
        } else {
            panic!("{:?} is not INTEGER as expected", self);
        }
    }

    pub fn expect_float(&self) -> f32 {
        if let Self::FLOAT(value) = self {
            *value
        } else {
            panic!("{:?} is not FLOAT as expected", self);
        }
    }
}

/* i32 */

impl From<i32> for DimensionValue {
    fn from(value: i32) -> Self {
        Self::INTEGER(value)
    }
}

/* f32 */

impl From<f32> for DimensionValue {
    fn from(value: f32) -> Self {
        Self::FLOAT(value)
    }
}
