//! Redis native module to add support for compare and swap

#[macro_use]
extern crate redis_module;

use redis_module::{Context, RedisError, RedisResult, RedisString, RedisValue};

/// Compare and swap if current value unchanged
pub fn cas(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
  match args.as_slice() {
    [_, key, current, new_value] => {
      if let RedisValue::SimpleString(value) = ctx.call("GET", &[key.try_as_str()?])? {
        if value.eq(&current.try_as_str()?) {
          let _ = ctx.call("SET", &[key.try_as_str()?, new_value.try_as_str()?])?;
          return Ok(RedisValue::Integer(1));
        }
      }

      Ok(RedisValue::Integer(0))
    }
    _ => Err(RedisError::WrongArity),
  }
}

redis_module! {
  name: "redis_cas",
  version: 1,
  data_types: [],
  commands: [
    ["cas", cas, "write fast", 1, 1, 0],
  ],
}
