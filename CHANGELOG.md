# 0.3.7

## Changed

- Use the debug format of `Duration` for `Session` elapsed time.
- Use more forgiving type `impl Into<String>` and `impl AsRef<str>` for string arguments.

## Fixed

- `README.md` use the correct version of `Session` writing in the example.


# 0.3.6

## Fixed

- `Session::log` was not correctly formatted the log message for writing.


# 0.3.5

## Fixed

- `Global::session` didn't track the location of where it was created.
- `Global::session_then` was missing from the implementation, it's now added.
- `Session` when created from `Logger` or `Global` was not correctly used right `Source`.
