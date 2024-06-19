---
default: major
---

# Rework `ChangeSet` and `PackageChange` interfaces

`ChangeSet` now uses a `Vec` internally instead of a `HashMap` to perform better for low/single-package repos.
Instead of accessing the internal `releases` field, you can use `into::<Vec<_>>()` or `into_iter()`.

`PackageChange` now stores both `unique_id` and `summary` in `Arc`s, since in multi-package repos, these strings
were potentially being cloned a lot.
