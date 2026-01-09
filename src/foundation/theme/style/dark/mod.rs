//! 暗色主题模块
//!
//! 包含10种暗色主题.

pub mod modern_dark;
pub mod nord;
pub mod dracula;
pub mod tokyo_night;
pub mod one_dark;
pub mod deep_black;
pub mod cyberpunk;
pub mod matrix;
pub mod monokai;
pub mod ayu_dark;

// 重新导出所有主题函数
pub use modern_dark::modern_dark;
pub use nord::nord;
pub use dracula::dracula;
pub use tokyo_night::tokyo_night;
pub use one_dark::one_dark;
pub use deep_black::deep_black;
pub use cyberpunk::cyberpunk;
pub use matrix::matrix;
pub use monokai::monokai;
pub use ayu_dark::ayu_dark;
