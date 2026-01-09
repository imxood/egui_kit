//! 亮色主题模块
//!
//! 包含5种亮色主题.

pub mod modern_light;
pub mod github_light;
pub mod solarized_light;
pub mod catppuccin;
pub mod gruvbox_light;

// 重新导出所有主题函数
pub use modern_light::modern_light;
pub use github_light::github_light;
pub use solarized_light::solarized_light;
pub use catppuccin::catppuccin;
pub use gruvbox_light::gruvbox_light;
