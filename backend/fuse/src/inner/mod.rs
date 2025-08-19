mod algorithm;
mod config;
mod core;
mod fuseable;
mod types;

// 保持与原来的 API 兼容，使用最小的重导出
pub use config::{Fuse, FuseBuilder};
pub use fuseable::Fuseable;
pub use types::FuseProperty;
