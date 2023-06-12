use std::{
    cmp::Ordering,
    collections::HashMap,
    convert::Infallible,
    error::Error,
    fmt::{Display, Formatter},
    str::FromStr,
};

/// Describes how a [`crate::Change`] affects the version of relevant packages.
///
/// This is guaranteed to never be empty, as a changeset must always apply to at least one package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Versioning(HashMap<PackageName, BumpType>);

impl From<(&str, BumpType)> for Versioning {
    fn from(value: (&str, BumpType)) -> Self {
        let value = (PackageName::from(value.0), value.1);
        Self::from(value)
    }
}

impl From<(PackageName, BumpType)> for Versioning {
    fn from(value: (PackageName, BumpType)) -> Self {
        let mut map = HashMap::new();
        map.insert(value.0, value.1);
        Self(map)
    }
}

impl Versioning {
    /// Creates a new [`Versioning`] from an iterator of tuples.
    ///
    /// # Errors
    ///
    /// 1. If the values type cannot be converted into a [`BumpType`], you'll get [`BuildVersioningError::BumpTypeParsingError`].
    /// 2. If the iterator is empty, you'll get [`BuildVersioningError::EmptyVersioningError`].
    pub fn try_from_iter<Key, Value, ParseError, Iter>(
        iter: Iter,
    ) -> Result<Self, BuildVersioningError>
    where
        Key: Into<PackageName>,
        Value: TryInto<BumpType, Error = ParseError>,
        ParseError: Into<BuildVersioningError>,
        Iter: IntoIterator<Item = (Key, Value)>,
    {
        let map = iter
            .into_iter()
            .map(|(key, value)| {
                value
                    .try_into()
                    .map_err(Into::into)
                    .map(|value| (key.into(), value))
            })
            .collect::<Result<HashMap<PackageName, BumpType>, BuildVersioningError>>()?;
        if map.is_empty() {
            Err(BuildVersioningError::EmptyVersioningError)
        } else {
            Ok(Self(map))
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&PackageName, &BumpType)> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for Versioning {
    type Item = (PackageName, BumpType);
    type IntoIter = std::collections::hash_map::IntoIter<PackageName, BumpType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// The error that occurs if you try to create a [`Versioning`] out of an iterator which has no items.
#[derive(Debug)]
pub enum BuildVersioningError {
    /// The iterator was empty.
    EmptyVersioningError,
    /// The iterator contained an invalid [`BumpType`].
    BumpTypeParsingError(BumpTypeParsingError),
}

impl Display for BuildVersioningError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyVersioningError => {
                f.write_str("Versioning needs to contain at least one item.")
            }
            Self::BumpTypeParsingError(error) => error.fmt(f),
        }
    }
}

impl From<BumpTypeParsingError> for BuildVersioningError {
    fn from(value: BumpTypeParsingError) -> Self {
        Self::BumpTypeParsingError(value)
    }
}

impl From<Infallible> for BuildVersioningError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl Error for BuildVersioningError {}

/// An alias to [`String`] to encode semantic meaning in [`Change::versioning`]
pub type PackageName = String;

/// The [Semantic Versioning](https://semver.org/) component which should be incremented when a [`Change`]
/// is applied.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum BumpType {
    #[default]
    None,
    Patch,
    Minor,
    Major,
}

impl Display for BumpType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BumpType::None => write!(f, "none"),
            BumpType::Patch => write!(f, "patch"),
            BumpType::Minor => write!(f, "minor"),
            BumpType::Major => write!(f, "major"),
        }
    }
}

impl FromStr for BumpType {
    type Err = BumpTypeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(BumpType::None),
            "patch" => Ok(BumpType::Patch),
            "minor" => Ok(BumpType::Minor),
            "major" => Ok(BumpType::Major),
            _ => Err(BumpTypeParsingError(String::from(s))),
        }
    }
}

impl Ord for BumpType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (BumpType::None, BumpType::None)
            | (BumpType::Major, BumpType::Major)
            | (BumpType::Patch, BumpType::Patch)
            | (BumpType::Minor, BumpType::Minor) => Ordering::Equal,
            (BumpType::None, _) => Ordering::Less,
            (_, BumpType::None) => Ordering::Greater,
            (BumpType::Patch, _) => Ordering::Less,
            (_, BumpType::Patch) => Ordering::Greater,
            (BumpType::Minor, _) => Ordering::Less,
            (_, BumpType::Minor) => Ordering::Greater,
        }
    }
}

impl PartialOrd for BumpType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The error that occurs when [`BumpType::from_str`] fails due to an invalid input.
#[derive(Debug)]
pub struct BumpTypeParsingError(String);

impl Display for BumpTypeParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a valid BumpType", self.0)
    }
}

impl Error for BumpTypeParsingError {}
