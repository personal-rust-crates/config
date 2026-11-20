cargo.toml

```
[dependencies]
config = { git = "https://github.com/personal-rust-crates/config.git", branch = "master" }
```

example.rs

```
use std::path::Path;

use crate::config_tag::{Config, ConfigTag};
use crate::wrapper::Wrapper;
use my_derive::ConfigTag;
use serde::{Deserialize, Serialize};

#[derive(ConfigTag, Serialize, Deserialize, PartialEq, Eq, Debug)]
struct TestConfig {
    number: usize,
    string: String,
}
```
