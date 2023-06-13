# Changesets

A Rust crate implementing [Changesets]. If you want a CLI which supports this format, check out [Knope].

## What can this do?

This crate programmatically works with [changesets], and only concerns itself with the [changeset][changesets] formats and conventions. It specifically _does not_ parse Markdown, bodies of changes are considered plain text.

Examples are not copy/pasted here, because it's hard to test them. So instead, here are links to common tasks:

- [Create a change](https://github.com/knope-dev/changesets/blob/61a3f4887e23af02542da66428d4364ee6025f00/tests/change.rs#L5)
- [Load a change](https://github.com/knope-dev/changesets/blob/61a3f4887e23af02542da66428d4364ee6025f00/tests/change.rs#LL46C6-L46C6)
- [Load a changeset](https://github.com/knope-dev/changesets/blob/61a3f4887e23af02542da66428d4364ee6025f00/tests/change_set.rs#L5)

## What is a changeset?

Releasing a project requires two things at a minimum:

1. Setting a new version, preferably a [Semantic Version][semver].
2. Describing the changes in some sort of release notes, like a [changelog](https://keepachangelog.com).

The manual way to do this is to review all the changes since the last release, write a changelog, and decide on a new version. However, the longer you go between _making the change_ (e.g., merging a pull request) and _releasing the change_, the more likely you are to forget something. This is especially true if you have a lot of changes, or if you have a lot of projects.

Changesets are a way of tracking changes as they happen, then bundling them up into a release. For each change you create a Markdown file containing which packages the change effects, how it effects them (in [semver terms][semver], and a Markdown summary of that change. For example, you might merge a PR which has these two change files:

### `.changeset/new_feature_to_fix_bug.md`

```markdown
---
changesets: minor
knope: patch
---

Added a feature to `changesets` to fix a bug in `knope`.
```

### `.changeset/new_feature_for_knope.md`

```markdown
---
knope: minor
---

This is a feature for Knope in the same PR
```

When you release, the `knope` package would contain both summaries in its changelog (and apply the greatest bump type), and the `changesets` package would contain only the first summary in its changelog.

This works very similarly to [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/), but does not rely on Git. You can use this together _with_ conventional commits using a tool like [Knope].

## Terminology in this project

- **Change**: A single Markdown file (usually in the `.changeset` directory) describing a change to one or more packages. Note that this matches the original definition of [changesets]. A change contains a summary (in Markdown), a list of packages affected, and the semver "bump type" for each package.
- **Change summary**: The Markdown description of a change. This is the body of the change file. It should be included in the generated changelog.
- **Bump type**: One of `none`, `patch`, `minor`, or `major`, describing which components of a semantic version are affected by the change.
- **Package**: A releasable unit of code. Examples include a Rust crate, a JavaScript package, a Go module. A change can affect multiple packages.
- **Changeset**: A _set_ of _changes_ which will be released together. Notably, this differs from the original definition of [changesets], which is does not have a term for the bundle of multiple changes. A changeset may affect any number of packages.
- **Release**: The part of a changeset that applies to a single package and determines how that package is released.

[semver]: https://semver.org/
[changesets]: https://github.com/changesets/changesets
[Knope]: https://github.com/knope-dev/knope
