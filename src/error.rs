//! 错误处理模块
//!
//! 定义了VNC DES模块中使用的各种错误类型

use thiserror::Error;

/// VNC DES库的统一结果类型
pub type Result<T> = std::result::Result<T, VncDesError>;

/// VNC DES错误类型
#[derive(Error, Debug)]
pub enum VncDesError {
    #[error("密码长度无效: {0}")]
    InvalidPasswordLength(String),
    
    #[error("DES加密失败: {0}")]
    EncryptionFailed(String),
    
    #[error("DES解密失败: {0}")]
    DecryptionFailed(String),
    
    #[error("密钥格式错误: {0}")]
    InvalidKeyFormat(String),
    
    #[error("密码格式错误: {0}")]
    InvalidPasswordFormat(String),
    
    #[error("十六进制解析错误: {0}")]
    HexDecodeError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("I/O错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("十六进制编码错误: {0}")]
    HexEncodingError(#[from] hex::FromHexError),
}

impl VncDesError {
    /// 创建一个无效密码长度错误
    pub fn invalid_password_length<T: Into<String>>(msg: T) -> Self {
        Self::InvalidPasswordLength(msg.into())
    }
    
    /// 创建一个加密失败错误
    pub fn encryption_failed<T: Into<String>>(msg: T) -> Self {
        Self::EncryptionFailed(msg.into())
    }
    
    /// 创建一个解密失败错误
    pub fn decryption_failed<T: Into<String>>(msg: T) -> Self {
        Self::DecryptionFailed(msg.into())
    }
    
    /// 创建一个无效密钥格式错误
    pub fn invalid_key_format<T: Into<String>>(msg: T) -> Self {
        Self::InvalidKeyFormat(msg.into())
    }
    
    /// 创建一个无效密码格式错误
    pub fn invalid_password_format<T: Into<String>>(msg: T) -> Self {
        Self::InvalidPasswordFormat(msg.into())
    }
    
    /// 创建一个配置错误
    pub fn config_error<T: Into<String>>(msg: T) -> Self {
        Self::ConfigError(msg.into())
    }
    
    /// 创建一个十六进制解析错误
    pub fn hex_decode_error<T: Into<String>>(msg: T) -> Self {
        Self::HexDecodeError(msg.into())
    }
} 