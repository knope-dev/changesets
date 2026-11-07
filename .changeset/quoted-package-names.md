---
changesets: major
---

Strip a matching pair of single or double quotes from package names when parsing change files. Quoted package names now produce the unquoted package name in `Change::versioning`.
