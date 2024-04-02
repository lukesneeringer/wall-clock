# Date

[![ci](https://github.com/lukesneeringer/wall-clock/actions/workflows/ci.yaml/badge.svg)](https://github.com/lukesneeringer/wall-clock/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/lukesneeringer/wall-clock/branch/main/graph/badge.svg?token=GPKesuBDmd)](https://codecov.io/gh/lukesneeringer/wall-clock)
[![release](https://img.shields.io/crates/v/wall-clock.svg)](https://crates.io/crates/wall-clock)
[![docs](https://img.shields.io/badge/docs-release-blue)](https://docs.rs/wall-clock/latest/date/)

`wall-clock` provides a simple and very basic struct for repsenting time as one reads it off a
clock on the wall, e.g. with no concept of date, or time zone.

## Examples

Making a wall clock time:

```rs
use wall_clock::WallClockTime;
let wct = WallClockTime::new(15, 0, 0);
```

You can also use the `time!` macro to get a syntax resembling a literal:

```rs
use wall_clock::time;
let wct = time!(15:00:00);
```

## Features

`wall-clock` ships with the following features:

- **`diesel-pg`**: Enables interop with PostgreSQL `TIME` columns using Diesel.
- **`serde`**: Enables serialization and desearialization with `serde`. _(Enabled by default.)_
