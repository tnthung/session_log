# `Session_log` crate

This crate tries to provide a easy to use logging system that can group / nest logs in a session like
manner. This is useful for systems that having multiple threads and you want to group logs by session.

## Usage

For most basic usage, you just need to import everything from `session_log::prelude` and you're good
to go.

```rust
use session_log::prelude::*;

fn main() {
    // Create a new logging entry point with default settings
    let logger = Logger::new("main");

    // Log a message directly to the logger
    logger.info("Hello, world!");

    // Create a new session
    {
        let session = logger.session();

        // Log some messages in the session
        session.info("Hello, session!");

        // Create a new session in the session
        {
            let session = session.session();

            // Log some messages in the session
            session.info("Hello, nested session!");
        }

        // Log some messages in the session
        session.info("Goodbye, session!");
    }
}
```

The creation of a logger is very cheap, essentially it's just a wrapper around `String` as the key
to a global map of loggers.

```rust
use session_log::prelude::*;

fn main() {
    // Create a new logging entry point with default settings
    let logger1 = Logger::new("main");
    logger1.set_level(Level::Verbose);

    // Create a new logger with the same name
    let logger2 = Logger::new("main");

    // They are the same logger
    assert_eq!(
        logger1.get_level(),
        logger2.get_level());
}

```

Each entry point have their own logging level and output directory can be set either at initialization
or at runtime.

```rust
use session_log::{
    prelude::*,
    Level,
};

fn main() {
    { // Create a new logger with custom settings
        let logger1 = Logger::with_options("1",
            Level::Verbose, "logs/1").unwrap();

        // If you try to use `with_options` to create a logger with the same name
        // but different settings, it will return an error
        assert_eq!(
            Logger::with_options("1",
                Level::Info, "logs/1").is_err(),
            true);

        // Same options will not return an error
        assert_eq!(
            Logger::with_options("1",
                Level::Verbose, "logs/1").is_err(),
            false);
    }

    { // Create a new logger with custom settings that may be omitted
        // If "2" is never been created before, it will use the given options
        let logger2 = Logger::try_with_options("2",
            Level::Verbose, "logs/2");

        // If "2" is already created, it will omit the given options
        let logger2 = Logger::try_with_options("2",
            Level::Verbose, "logs/2");
    }

    { // Do the setting after initialization
        // This will create a new logger with default settings or retrieve the existing one
        // regardless of the settings. Default setting: (Level::Info, "logs")
        let logger2 = Logger::new("3");

        // Configure the logger afterward
        logger2.set_level(Level::Verbose);
        logger2.set_directory("logs/3");
    }
}
```
