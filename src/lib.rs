pub use my_derive::ConfigTag;
pub mod config_tag;
pub mod wrapper;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn it_works() {
        let actual = Wrapper::<TestConfig>::load_cfg(Path::new("test.json")).config;
        let expected = TestConfig {
            number: 1,
            string: "hello".to_string(),
        };

        assert_eq!(actual, expected);
    }
}
