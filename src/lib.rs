//! VNC DES - 模块化和可扩展的VNC DES加密库
//!
//! 这个库提供了符合VNC协议标准（RFC 6143）的DES加密/解密功能，
//! 与所有标准VNC实现（如TightVNC、RealVNC等）兼容，
//! 支持可配置的密钥和灵活的使用方式。
//!
//! # 特性
//!
//! - 🔒 符合VNC协议标准（RFC 6143）的DES认证算法
//! - ⚙️ 可配置的加密密钥
//! - 🏗️ 模块化和可扩展的设计
//! - 📚 既可作为库使用，也可编译为独立可执行程序
//! - 🧪 完整的测试覆盖
//!
//! # 快速开始
//!
//! ## 作为库使用
//!
//! ### 使用默认配置（VNC协议标准）
//!
//! ```rust
//! use vnc_des::{VncDesProcessor, PasswordProcessor};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // 方法1: 使用处理器实例
//! let mut processor = VncDesProcessor::default();
//! let encrypted = processor.encrypt_password("password")?;
//! let decrypted = processor.decrypt_password(&encrypted)?;
//! assert_eq!(decrypted, "password");
//!
//! // 方法2: 使用静态方法（更简单）
//! let encrypted = PasswordProcessor::encrypt_with_default("password")?;
//! let decrypted = PasswordProcessor::decrypt_with_default(&encrypted)?;
//! assert_eq!(decrypted, "password");
//! # Ok(())
//! # }
//! ```
//!
//! ### 使用自定义密钥
//!
//! ```rust
//! use vnc_des::{VncDesProcessor, VncDesConfig};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // 方法1: 使用配置
//! let config = VncDesConfig::new()
//!     .with_hex_key("0123456789abcdef")?
//!     .with_strict_mode(true);
//!
//! let mut processor = VncDesProcessor::new(config);
//! let encrypted = processor.encrypt_password("test")?;
//!
//! // 方法2: 直接指定密钥
//! let custom_key = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
//! let mut processor = VncDesProcessor::with_key(custom_key);
//! let encrypted = processor.encrypt_password("test")?;
//! # Ok(())
//! # }
//! ```
//!
//! ### 十六进制转换
//!
//! ```rust
//! use vnc_des::{VncDesProcessor, PasswordProcessor};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let encrypted = PasswordProcessor::encrypt_with_default("password")?;
//! let hex_string = VncDesProcessor::to_hex_string(&encrypted);
//! println!("加密密码: {}", hex_string);
//!
//! let decoded = VncDesProcessor::from_hex_string(&hex_string)?;
//! assert_eq!(encrypted, decoded);
//! # Ok(())
//! # }
//! ```
//!
//! # 配置选项
//!
//! ```rust
//! use vnc_des::{VncDesConfig, VncDesConfigBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // 使用构建器模式
//! let config = VncDesConfigBuilder::new()
//!     .hex_key("17526b06234e5807")?  // 某些VNC实现的默认密钥（如TightVNC）
//!     .strict_mode(false)            // 非严格模式
//!     .auto_truncate(true)           // 自动截断长密码
//!     .max_password_length(8)        // 最大密码长度
//!     .build()?;
//!
//! // 或者使用链式调用
//! let config = VncDesConfig::new()
//!     .with_strict_mode(true)
//!     .with_auto_truncate(false);
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod crypto;
pub mod error;

// 重新导出主要类型以便外部使用
pub use config::{VncDesConfig, VncDesConfigBuilder, TIGHTVNC_DEFAULT_KEY};
pub use crypto::{PasswordProcessor, VncDesEngine, VncDesProcessor};
pub use error::{Result, VncDesError};

// 版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// 获取库版本信息
pub fn version() -> &'static str {
    VERSION
}

/// 获取库名称
pub fn name() -> &'static str {
    NAME
}

/// 获取库的完整信息
pub fn info() -> String {
    format!("{} v{}", NAME, VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        assert!(!version().is_empty());
        assert!(!name().is_empty());
        assert!(info().contains(version()));
        assert!(info().contains(name()));
    }

    #[test]
    fn test_basic_encryption() {
        let mut processor = VncDesProcessor::default();
        let password = "test123";

        let encrypted = processor.encrypt_password(password).unwrap();
        let decrypted = processor.decrypt_password(&encrypted).unwrap();

        assert_eq!(password, decrypted);
    }

    #[test]
    fn test_static_methods() {
        let password = "static"; // 使用8字符以内的密码，符合默认配置

        let encrypted = PasswordProcessor::encrypt_with_default(password).unwrap();
        let decrypted = PasswordProcessor::decrypt_with_default(&encrypted).unwrap();

        assert_eq!(password, decrypted);
    }

    #[test]
    fn test_custom_config() {
        let config = VncDesConfig::new()
            .with_strict_mode(true)
            .with_auto_truncate(false);

        let mut processor = VncDesProcessor::new(config);
        let password = "test";

        let encrypted = processor.encrypt_password(password).unwrap();
        let decrypted = processor.decrypt_password(&encrypted).unwrap();

        assert_eq!(password, decrypted);
    }
}
