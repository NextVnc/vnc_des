//! VNC DES加密模块
//!
//! 提供符合VNC协议标准的DES加密/解密功能

pub mod des;
pub mod vnc_des;

// 重新导出主要类型
pub use des::VncDesEngine;
pub use vnc_des::{PasswordProcessor, VncDesProcessor};
