## 0.5.0 (2026-11-07)

### Breaking Changes

- Strip a matching pair of single or double quotes from package names when parsing change files. Quoted package names now produce the unquoted package name in `Change::versioning`.

### Fixes

- strip quotes from parsed package names (#156)

## 0.4.0 (2025-03-08)

### Breaking Changes

- Update edition to 2024 and MSRV to 1.85

#### Stop normalizing paths for existing files

If you already have a change file,
potentially created by another tool,
this library renormalizing the file name can cause unexpected errors (for example, when writing _back_ to the file).

Internally, `Change::from_file`, `Change::from_file_name_and_content`,
and `ChangeSet::from_directory` all now use `UniqueId::exact`.

When creating a _new_ change file (not opening an existing one),
you should construct a `Change { ... }` yourself and use `UniqueId::normalize` to get the previous behavior.

#### Removed `From<AsRef<str>>` for `UniqueId`

Instead, use either `UniqueId::normalize` or `UniqueId::exact` to specify if you'd like the value to be transformed.
