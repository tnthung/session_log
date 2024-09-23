# 0.3.5

## Fixed

- `Global::session` didn't track the location of where it was created.
- `Global::session_then` was missing from the implementation, it's now added.
- `Session` when created from `Logger` or `Global` was not correctly used right `Source`.
