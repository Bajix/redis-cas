# Redis CAS (Compare and Swap)

![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/redis-cas.svg)](https://crates.io/crates/redis-cas)
[![Documentation](https://docs.rs/redis-cas/badge.svg)](https://docs.rs/redis-cas)


Redis native module to add support for compare and swap

## Syntax
```
CAS key current_value new_value
```

Compare the value of a key and set if the passed current value hasn't changed

## Return

Returns number of modified keys

## Example usage
```
127.0.0.1:6379> set foo bar
OK
127.0.0.1:6379> cas foo bar baz
(integer) 1
127.0.0.1:6379> cas foo bar baz
(integer) 0
```

## Build and load with taskfile

```
task load-redis-module
```
