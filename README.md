# `Session_log` crate

This crate tries to provide a easy to use logging system that can group / nest logs in a session like
manner. This is useful for systems that having multiple threads and you want to group logs by session.

## Kind of logger

There are two types of basic logger, `Logger` and `Global`. The difference between them is that `Global`
will preserve the configuration for any instance with the same name, while `Logger` will not. Each
`Logger` can be configured independently even if they have the same name. `Global` is useful for when
a logger should be globally available, such as in an multi-threaded api applications, `Network` or
`Router` global may be created through out the whole lifespan of the application.

`Logger` on the other hand is suitable for short lived objects, such as for a multi-step request you
may want to create a logger for whole process, and then create a `Session` for each step.

Because of the register-once and use-anywhere nature of `Global`, a separate set of logging macros are
provided for `Global` to avoid boilerplate code for creating a logger and passing it around.

## Logger

As we mentioned before, `Logger` is much suitable for short lived objects where dynamic name or config
may be required for each instance. `Logger` can be created with `Logger::new` or `Logger::default`.
The `Logger::new` will create a new logger with user provided name & formatter and configurations, while
`Logger::default` only takes the name and use the default formatter and configurations.


```rust
// custom configuration
let logger = Logger::new<Formatter>("logger", Config {
    // configurations
});

// default configuration
let logger = Logger::default("logger");
// equivalent to
let logger = Logger::new<DefaultFormatter>("logger", Config::default());
```

To log a message, you can use either macros for methods directly:

```rust
// using methods
let name = "world";
logger.info(&format!("hello, {}", name));

// using macros
let name = "world";
log_info!(logger, "hello, {}", name);
```


## Global

`Global` as we talked about, you only need to configure it once and it will be available for any
instance with the same name. `Global` can be created with `Global::new` or `Global::register`. The
`Global::new` will create a new logger with default configurations and register it, while
`Global::register` needs the formatter and configurations to be provided.

```rust
// register a new logger with custom configuration
let logger = Global::register("logger", Config {
    // configurations
});

// new logger with default configuration
let logger = Global::new("logger");
// equivalent to
let logger = Global::register("logger", Config::default());
```

Reason for using word `register` for fully customized logger is for it's opposite `unregister`. When
you want to remove a logger from the global registry, you can use `Global::unregister` to remove it.

```rust
Global::unregister("logger");
```

To log a message, you can use either macros for methods directly:

```rust
// using methods
let name = "world";
logger.info(&format!("hello, {}", name));

// using macros
let name = "world";
log_info!(logger, "hello, {}", name);
```

As we said, global loggers have their own set of macros, which instead of passing the logger instance
you only need to pass the name of the logger. They're all prefixed with `glog_` to avoid conflict with
`log_` macros.

```rust
let name = "world";
glog_info!("logger", "hello, {}", name);
```


## Session

`Session` is a logger that will group all logs together when writing to the file. This is perfect for
multi-threaded or async applications where at any given time there are multiple threads running and
can potentially log things. In traditional logging system, logs from same session may be interleaved
with logs from other sessions, making it hard to track the logs. With `Session`, all logs from the same
session will be grouped together and write once for better readability.

A session can be created from `Logger`, `Global` or another `Session` to create a nested session.

```rust
// first layer session
let session = Logger::new("logger").session("session", true);
let session = Global::new("global").session("session", true);

// nested session
let sub_session = session.session("sub-session", true);

// session from global without creating global instance
let session = Global::session("logger", "session", true);
// equivalent to
let session = Global::new("logger").session("session", true);
```

Sessions are having a `silent` flag, which when set to will not log / write anything to the output when
no logs are written. What that means is, when a session is created with `silent` flag set to `false`,
the `Session Start` and `Session End` will always be printed / written to the output, even if no logs
were written during the session's lifespan. When set to `true`, the `Session Start` will only be printed
when the first log is written, and if the session end up with no logs written, the `Session End` will
also not get printed.

```rust
// silent session
let session = logger.session("session", true);

// non-silent session
let session = logger.session("session", false);
```

When logging a message, session is having the exact interface as `Logger`.

```rust
let session = logger.session("session", true);

// using methods
let name = "world";
session.info(&format!("hello, {}", name));

// using macros
let name = "world";
log_info!(session, "hello, {}", name);
```

For all three types of logger that can spawn new session, there are also `session_then` method that
will create a new session and execute the provided closure with the new session.

```rust
let session = logger.session_then("session", true, |session| {
    let name = "world";
    session.info(&format!("hello, {}", name));
});
```

## Example

Following example is demonstrating how does `session_log` differ from other more traditional logging
system. In this example, we have three threads that will log a message every second for 5 times.

### Traditional logging

```rust
let mut threads = vec![];

let logger = Logger::default("logger");
for i in 0..3 {
    let logger = logger.clone();
    threads.push(spawn(move || {
        for j in 0..5 {
            logger.info(&format!("thread {i}: {j} hello"));
            sleep(Duration::from_secs(1));
        }
    }));
}

for thread in threads {
    thread.join().unwrap();
}
```

The code above may yield the result like this:

```
thread 0: 0 hello
thread 1: 0 hello
thread 2: 0 hello
thread 2: 1 hello
thread 0: 1 hello
thread 1: 1 hello
thread 2: 2 hello
thread 1: 2 hello
thread 0: 2 hello
thread 0: 3 hello
thread 2: 3 hello
thread 1: 3 hello
thread 1: 4 hello
thread 0: 4 hello
thread 2: 4 hello
```

### With `session_log`

```rust
let mut threads = vec![];

let logger = Logger::default("logger");
for i in 0..3 {
    let session = logger.session(&format!("thread {i}"), true);
    threads.push(spawn(move || {
        for j in 0..5 {
            session.info(&format!("{j} hello"));
            sleep(Duration::from_secs(1));
        }
    }));
}

for thread in threads {
    thread.join().unwrap();
}
```

When printing:

```
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     logger:thread 1 - src\lib.rs:52 - Session Start
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     logger:thread 2 - src\lib.rs:52 - Session Start
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     logger:thread 0 - src\lib.rs:52 - Session Start
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 0 - src\lib.rs:55 - 0 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 2 - src\lib.rs:55 - 0 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 1 - src\lib.rs:55 - 0 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 2 - src\lib.rs:55 - 1 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 1 - src\lib.rs:55 - 1 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 0 - src\lib.rs:55 - 1 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 2 - src\lib.rs:55 - 2 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 1 - src\lib.rs:55 - 2 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 0 - src\lib.rs:55 - 2 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 2 - src\lib.rs:55 - 3 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 1 - src\lib.rs:55 - 3 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 0 - src\lib.rs:55 - 3 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 2 - src\lib.rs:55 - 4 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 1 - src\lib.rs:55 - 4 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] logger:thread 0 - src\lib.rs:55 - 4 hello
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     logger:thread 2 - src\lib.rs:52 - Session End (5.0018507s)
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     logger:thread 1 - src\lib.rs:52 - Session End (5.0018858s)
YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     logger:thread 0 - src\lib.rs:52 - Session End (5.0023585s)
```

From this look it's still interleaved, but when the file is actually written:

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
┃ Logger : logger
┃ Session: thread 2
┃ Elapsed: 5.0018507s
┃
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     src\lib.rs:55 - Session Start
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 0 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 1 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 2 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 3 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 4 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     src\lib.rs:55 - Session End
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
┃ Logger : logger
┃ Session: thread 1
┃ Elapsed: 5.0018858s
┃
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     src\lib.rs:55 - Session Start
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 0 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 1 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 2 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 3 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 4 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     src\lib.rs:55 - Session End
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
┃ Logger : logger
┃ Session: thread 0
┃ Elapsed: 5.0023585s
┃
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     src\lib.rs:55 - Session Start
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 0 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 1 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 2 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 3 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ [I] src\lib.rs:58 - 4 hello
┃ YYYY-MM-DDTHH:mm:ss.ssssss+ZZ:ZZ     src\lib.rs:55 - Session End
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

This is much easier to read the log from each thread.
