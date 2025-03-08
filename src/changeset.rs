use std::{path::Path, sync::Arc};

use crate::{
    Change, ChangeType, PackageName,
    change::{LoadingError, UniqueId},
};

/// A set of [`Change`]s that combine to form [`Release`]s of one or more packages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeSet {
    releases: Vec<Release>,
}

impl ChangeSet {
    /// Load from a directory (usually called `.changeset`) containing markdown files.
    ///
    /// Any files that don't end with `.md` will be ignored.
    ///
    /// # Errors
    ///
    /// 1. Directory doesn't exist
    /// 2. There's a problem loading a file (see [`Change`] for details)
    pub fn from_directory<P: AsRef<Path>>(path: P) -> Result<Self, LoadingError> {
        path.as_ref()
            .read_dir()?
            .filter_map(|entry| {
                entry
                    .map_err(LoadingError::from)
                    .and_then(|entry| {
                        let path = entry.path();
                        if path.extension().is_some_and(|ext| ext == "md") && path.is_file() {
                            Change::from_file(path).map(Some)
                        } else {
                            Ok(None)
                        }
                    })
                    .transpose()
            })
            .collect()
    }
}

impl FromIterator<Change> for ChangeSet {
    fn from_iter<T: IntoIterator<Item = Change>>(iter: T) -> Self {
        let mut releases = iter
            .into_iter()
            .flat_map(|change| {
                let unique_id = Arc::new(change.unique_id);
                let summary: Arc<str> = change.summary.into();
                change
                    .versioning
                    .into_iter()
                    .map(move |(package_name, change_type)| {
                        (
                            package_name,
                            PackageChange {
                                change_type,
                                unique_id: unique_id.clone(),
                                summary: summary.clone(),
                            },
                        )
                    })
            })
            .fold(
                Vec::<Release>::new(),
                |mut releases, (package_name, change)| {
                    if let Some(release) = releases
                        .iter_mut()
                        .find(|release| release.package_name == package_name)
                    {
                        release.changes.push(change);
                    } else {
                        releases.push(Release {
                            package_name,
                            changes: vec![change],
                        });
                    }
                    releases
                },
            );
        for release in &mut releases {
            release
                .changes
                .sort_by(|first, second| first.unique_id.cmp(&second.unique_id));
        }
        Self { releases }
    }
}

impl IntoIterator for ChangeSet {
    type Item = Release;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.releases.into_iter()
    }
}

impl From<ChangeSet> for Vec<Release> {
    fn from(value: ChangeSet) -> Vec<Release> {
        value.releases
    }
}

/// The combination of applicable [`Change`]s in a [`ChangeSet`] for a single package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Release {
    pub package_name: PackageName,
    pub changes: Vec<PackageChange>,
}

impl Release {
    /// The overall [`ChangeType`] for the package's version based on all the [`Release::changes`].
    #[must_use]
    pub fn change_type(&self) -> Option<&ChangeType> {
        self.changes.iter().map(|change| &change.change_type).max()
    }
}

/// A [`Change`] as it applies to a single package for a [`Release`],
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageChange {
    /// The ID of the originating [`Change`].
    pub unique_id: Arc<UniqueId>,
    /// The type of change, which determines how the version will be bumped (if at all).
    pub change_type: ChangeType,
    /// The details of the change, as a markdown-formatted string.
    pub summary: Arc<str>,
}
