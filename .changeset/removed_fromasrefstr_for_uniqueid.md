---
default: major
---

# Removed `From<AsRef<str>>` for `UniqueId`

Instead, use either `UniqueId::normalize` or `UniqueId::exact` to specify if you'd like the value to be transformed.
