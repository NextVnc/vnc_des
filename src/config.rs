//! 配置管理模块
//!
//! 提供VNC DES模块的配置管理功能，支持可配置的密钥和其他参数

use crate::error::{Result, VncDesError};
use serde::{Deserialize, Serialize};

/// 常见VNC实现的默认硬编码密钥（如TightVNC）
/// 来源：TightVNC源代码 util/VncPassCrypt.cpp:29
/// 注意：这是VNC协议实现层面的约定，不是协议标准本身的一部分
pub const TIGHTVNC_DEFAULT_KEY: [u8; 8] = [23, 82, 107, 6, 35, 78, 88, 7];

/// VNC DES配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VncDesConfig {
    /// DES加密密钥（8字节）
    pub encryption_key: [u8; 8],
    /// 是否使用严格模式（严格验证密码长度等）
    pub strict_mode: bool,
    /// 是否自动截断超长密码
    pub auto_truncate: bool,
    /// 最大密码长度
    pub max_password_length: usize,
}

impl Default for VncDesConfig {
    fn default() -> Self {
        Self {
            encryption_key: TIGHTVNC_DEFAULT_KEY,
            strict_mode: false,
            auto_truncate: true,
            max_password_length: 8,
        }
    }
}

impl VncDesConfig {
    /// 创建一个新的配置
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置加密密钥
    pub fn with_key(mut self, key: [u8; 8]) -> Self {
        self.encryption_key = key;
        self
    }
    
    /// 从十六进制字符串设置密钥
    pub fn with_hex_key(mut self, hex_key: &str) -> Result<Self> {
        let key_bytes = hex::decode(hex_key)
            .map_err(|e| VncDesError::hex_decode_error(format!("无法解析十六进制密钥: {}", e)))?;
        
        if key_bytes.len() != 8 {
            return Err(VncDesError::invalid_key_format(
                format!("密钥长度必须为8字节，实际长度: {}", key_bytes.len())
            ));
        }
        
        let mut key = [0u8; 8];
        key.copy_from_slice(&key_bytes);
        self.encryption_key = key;
        Ok(self)
    }
    
    /// 设置严格模式
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }
    
    /// 设置自动截断
    pub fn with_auto_truncate(mut self, truncate: bool) -> Self {
        self.auto_truncate = truncate;
        self
    }
    
    /// 设置最大密码长度
    pub fn with_max_password_length(mut self, length: usize) -> Self {
        self.max_password_length = length;
        self
    }
    
    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        if self.max_password_length == 0 {
            return Err(VncDesError::config_error("最大密码长度不能为0"));
        }
        
        if self.max_password_length > 256 {
            return Err(VncDesError::config_error("最大密码长度不能超过256"));
        }
        
        Ok(())
    }
    
    /// 获取密钥的十六进制表示
    pub fn key_as_hex(&self) -> String {
        hex::encode(self.encryption_key)
    }
    
    /// 从JSON字符串加载配置
    pub fn from_json(json: &str) -> Result<Self> {
        let config: Self = serde_json::from_str(json)?;
        config.validate()?;
        Ok(config)
    }
    
    /// 转换为JSON字符串
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    
    /// 从文件加载配置
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_json(&content)
    }
    
    /// 保存配置到文件
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

/// 配置构建器
#[derive(Debug, Default)]
pub struct VncDesConfigBuilder {
    config: VncDesConfig,
}

impl VncDesConfigBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置加密密钥
    pub fn encryption_key(mut self, key: [u8; 8]) -> Self {
        self.config.encryption_key = key;
        self
    }
    
    /// 从十六进制字符串设置密钥
    pub fn hex_key(mut self, hex_key: &str) -> Result<Self> {
        let key_bytes = hex::decode(hex_key)
            .map_err(|e| VncDesError::hex_decode_error(format!("无法解析十六进制密钥: {}", e)))?;
        
        if key_bytes.len() != 8 {
            return Err(VncDesError::invalid_key_format(
                format!("密钥长度必须为8字节，实际长度: {}", key_bytes.len())
            ));
        }
        
        let mut key = [0u8; 8];
        key.copy_from_slice(&key_bytes);
        self.config.encryption_key = key;
        Ok(self)
    }
    
    /// 设置严格模式
    pub fn strict_mode(mut self, strict: bool) -> Self {
        self.config.strict_mode = strict;
        self
    }
    
    /// 设置自动截断
    pub fn auto_truncate(mut self, truncate: bool) -> Self {
        self.config.auto_truncate = truncate;
        self
    }
    
    /// 设置最大密码长度
    pub fn max_password_length(mut self, length: usize) -> Self {
        self.config.max_password_length = length;
        self
    }
    
    /// 构建配置
    pub fn build(self) -> Result<VncDesConfig> {
        self.config.validate()?;
        Ok(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VncDesConfig::default();
        assert_eq!(config.encryption_key, TIGHTVNC_DEFAULT_KEY);
        assert!(!config.strict_mode);
        assert!(config.auto_truncate);
        assert_eq!(config.max_password_length, 8);
    }

    #[test]
    fn test_config_builder() {
        let config = VncDesConfigBuilder::new()
            .strict_mode(true)
            .auto_truncate(false)
            .max_password_length(16)
            .build()
            .unwrap();
        
        assert!(config.strict_mode);
        assert!(!config.auto_truncate);
        assert_eq!(config.max_password_length, 16);
    }

    #[test]
    fn test_hex_key() {
        let hex_key = "17526b06234e5807";
        let config = VncDesConfig::new()
            .with_hex_key(hex_key)
            .unwrap();
        
        assert_eq!(config.encryption_key, TIGHTVNC_DEFAULT_KEY);
        assert_eq!(config.key_as_hex(), hex_key);
    }

    #[test]
    fn test_json_serialization() {
        let config = VncDesConfig::default();
        let json = config.to_json().unwrap();
        let deserialized = VncDesConfig::from_json(&json).unwrap();
        
        assert_eq!(config.encryption_key, deserialized.encryption_key);
        assert_eq!(config.strict_mode, deserialized.strict_mode);
    }
} 