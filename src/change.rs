use std::{
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::{BuildVersioningError, ChangeType, PackageName, Versioning};

/// Represents a single [change](https://github.com/knope-dev/changesets#terminology) which is
/// applicable to any number of packages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Change {
    /// Something to uniquely identify a change.
    ///
    /// This is the name of the file (without the `.md` extension) which defines this changeset.
    pub unique_id: UniqueId,
    /// Describes how a changeset affects the relevant packages.
    pub versioning: Versioning,
    /// The details of the change which will be written to a Changelog file
    pub summary: String,
}

impl Change {
    /// Create a markdown file in the provided directory with the contents of this [`Change`].
    ///
    /// The name of the created file will be the [`Change::unique_id`] with the `.md` extension—
    /// that path is returned.
    ///
    /// # Errors
    ///
    /// If the file cannot be written, an [`std::io::Error`] is returned. This may happen if the
    /// directory does not exist.
    pub fn write_to_directory<T: AsRef<Path>>(&self, path: T) -> std::io::Result<PathBuf> {
        let output_path = path.as_ref().join(self.unique_id.to_file_name());
        std::fs::write(&output_path, self.to_string())?;
        Ok(output_path)
    }

    /// Load a [`Change`] from a Markdown file.
    ///
    /// # Errors
    ///
    /// - If the file cannot be read
    /// - If the file does not have a valid name (i.e. it does not end in `.md`)
    /// - If the file does not have a valid front matter
    /// - If the file does not have a valid versioning info in the front matter
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, LoadingError> {
        let path = path.as_ref();
        let file_name = path
            .file_name()
            .ok_or(LoadingError::InvalidFileName)?
            .to_string_lossy();
        let unique_id = file_name
            .strip_suffix(".md")
            .ok_or(LoadingError::InvalidFileName)?
            .into();
        let contents = std::fs::read_to_string(path)?;
        Self::from_str(unique_id, &contents).map_err(LoadingError::from)
    }

    fn from_str(unique_id: UniqueId, content: &str) -> Result<Self, ParsingError> {
        let mut lines = content.lines();
        let first_line = lines.next().ok_or(ParsingError::MissingFrontMatter)?;
        if first_line.trim() != "---" {
            return Err(ParsingError::MissingFrontMatter);
        }
        let versioning_iter = lines
            .clone()
            .take_while(|line| line.trim() != "---")
            .map(|line| {
                let parts = line
                    .split_once(':')
                    .ok_or(ParsingError::InvalidFrontMatter)?;
                let package_name = PackageName::from(parts.0.trim());
                let change_type = ChangeType::from(parts.1.trim());
                Ok((package_name, change_type))
            })
            .collect::<Result<Vec<(String, ChangeType)>, ParsingError>>()?;
        let versioning = Versioning::try_from_iter(versioning_iter)?;
        let mut lines = lines.skip(versioning.len());
        let end_front_matter = lines.next().ok_or(ParsingError::InvalidFrontMatter)?;
        if end_front_matter.trim() != "---" {
            return Err(ParsingError::InvalidFrontMatter);
        }
        let summary = lines
            .skip_while(|line| line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(Self {
            unique_id,
            versioning,
            summary,
        })
    }
}

impl Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---")?;
        for (package_name, change_type) in self.versioning.iter() {
            writeln!(f, "{package_name}: {change_type}")?;
        }
        writeln!(f, "---")?;
        writeln!(f)?;
        writeln!(f, "{}", self.summary)
    }
}

/// The unique ID of a [`Change`], parsed from and used to set the file name of the Markdown file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UniqueId(String);

impl UniqueId {
    pub fn to_file_name(&self) -> String {
        format!("{self}.md")
    }
}

impl<T: AsRef<str>> From<T> for UniqueId {
    fn from(s: T) -> Self {
        Self(
            s.as_ref()
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_alphanumeric() {
                        Some(c.to_ascii_lowercase())
                    } else if c == ' ' {
                        Some('_')
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}

impl Display for UniqueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
#[test]
fn test_create_unique_id() {
    assert_eq!(
        UniqueId::from("`[i carry your heart with me(i carry it in]`").to_string(),
        "i_carry_your_heart_with_mei_carry_it_in"
    );
}

#[derive(Debug)]
pub enum ParsingError {
    MissingFrontMatter,
    InvalidFrontMatter,
    InvalidVersioning(BuildVersioningError),
}

impl From<BuildVersioningError> for ParsingError {
    fn from(err: BuildVersioningError) -> Self {
        ParsingError::InvalidVersioning(err)
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::MissingFrontMatter => write!(f, "missing front matter"),
            ParsingError::InvalidFrontMatter => write!(f, "invalid front matter"),
            ParsingError::InvalidVersioning(err) => {
                write!(f, "invalid front matter: {err}")
            }
        }
    }
}

impl Error for ParsingError {}

#[derive(Debug)]
pub enum LoadingError {
    InvalidFileName,
    Io(std::io::Error),
    Parsing(ParsingError),
}

impl From<std::io::Error> for LoadingError {
    fn from(err: std::io::Error) -> Self {
        LoadingError::Io(err)
    }
}

impl From<ParsingError> for LoadingError {
    fn from(err: ParsingError) -> Self {
        LoadingError::Parsing(err)
    }
}

impl Display for LoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingError::InvalidFileName => write!(f, "invalid file name"),
            LoadingError::Io(err) => Display::fmt(err, f),
            LoadingError::Parsing(err) => Display::fmt(err, f),
        }
    }
}

impl Error for LoadingError {}
