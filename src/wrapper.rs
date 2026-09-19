use serde::{Deserialize, Serialize};

use crate::config_tag::Config;

#[derive(Serialize, Deserialize, Clone)]
pub struct Wrapper<T: Config> {
    pub config: T,
}
impl<T: Config> Config for Wrapper<T> {
    fn load_cfg(path: &std::path::Path) -> Self {
        Self {
            config: T::load_cfg(path),
        }
    }

    fn save_cfg(&self, path: &std::path::Path) {
        self.config.save_cfg(path);
    }
}
