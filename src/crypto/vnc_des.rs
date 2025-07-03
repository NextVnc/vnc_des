//! VNC DES高级处理器
//!
//! 提供易于使用的密码加密、解密和验证功能

use crate::config::VncDesConfig;
use crate::crypto::des::VncDesEngine;
use crate::error::{Result, VncDesError};

/// VNC DES处理器
#[derive(Debug, Clone)]
pub struct VncDesProcessor {
    config: VncDesConfig,
    engine: VncDesEngine,
}

impl Default for VncDesProcessor {
    fn default() -> Self {
        Self::new(VncDesConfig::default())
    }
}

impl VncDesProcessor {
    /// 使用指定配置创建处理器
    pub fn new(config: VncDesConfig) -> Self {
        Self {
            config,
            engine: VncDesEngine::new(),
        }
    }

    /// 使用默认配置创建处理器
    pub fn with_default_config() -> Self {
        Self::default()
    }

    /// 使用自定义密钥创建处理器
    pub fn with_key(key: [u8; 8]) -> Self {
        let config = VncDesConfig::new().with_key(key);
        Self::new(config)
    }

    /// 从十六进制密钥创建处理器
    pub fn with_hex_key(hex_key: &str) -> Result<Self> {
        let config = VncDesConfig::new().with_hex_key(hex_key)?;
        Ok(Self::new(config))
    }

    /// 获取当前配置
    pub fn config(&self) -> &VncDesConfig {
        &self.config
    }

    /// 更新配置
    pub fn set_config(&mut self, config: VncDesConfig) {
        self.config = config;
    }

    /// 处理密码（截断或验证长度）
    fn process_password(&self, password: &str) -> Result<String> {
        if password.is_empty() {
            return Err(VncDesError::invalid_password_length("密码不能为空"));
        }

        if password.len() > self.config.max_password_length {
            if self.config.strict_mode && !self.config.auto_truncate {
                return Err(VncDesError::invalid_password_length(
                    format!("密码长度超过最大限制 {} 字符", self.config.max_password_length)
                ));
            }
            
            if self.config.auto_truncate {
                return Ok(password[..self.config.max_password_length].to_string());
            }
        }

        Ok(password.to_string())
    }

    /// 加密密码
    pub fn encrypt_password(&mut self, password: &str) -> Result<Vec<u8>> {
        let processed_password = self.process_password(password)?;
        
        // 将密码转换为8字节数组，不足的用0填充
        let mut password_bytes = [0u8; 8];
        let pwd_bytes = processed_password.as_bytes();
        let copy_len = std::cmp::min(pwd_bytes.len(), 8);
        password_bytes[..copy_len].copy_from_slice(&pwd_bytes[..copy_len]);

        // 加密
        let mut encrypted = [0u8; 8];
        self.engine.encrypt(&mut encrypted, &password_bytes, &self.config.encryption_key)
            .map_err(|e| VncDesError::encryption_failed(format!("加密失败: {}", e)))?;

        Ok(encrypted.to_vec())
    }

    /// 解密密码
    pub fn decrypt_password(&mut self, encrypted_password: &[u8]) -> Result<String> {
        if encrypted_password.len() != 8 {
            return Err(VncDesError::invalid_password_format(
                format!("加密密码长度必须为8字节，实际长度: {}", encrypted_password.len())
            ));
        }

        let mut encrypted_array = [0u8; 8];
        encrypted_array.copy_from_slice(encrypted_password);

        let mut decrypted = [0u8; 8];
        self.engine.decrypt(&mut decrypted, &encrypted_array, &self.config.encryption_key)
            .map_err(|e| VncDesError::decryption_failed(format!("解密失败: {}", e)))?;

        // 移除尾部的0字节并转换为字符串
        let end_pos = decrypted.iter().position(|&x| x == 0).unwrap_or(8);
        let password_str = std::str::from_utf8(&decrypted[..end_pos])
            .map_err(|e| VncDesError::decryption_failed(format!("解密结果不是有效的UTF-8: {}", e)))?;

        Ok(password_str.to_string())
    }

    /// 验证密码
    pub fn verify_password(&mut self, plain_password: &str, encrypted_password: &[u8]) -> Result<bool> {
        let encrypted_plain = self.encrypt_password(plain_password)?;
        Ok(encrypted_plain == encrypted_password)
    }

    /// 将加密密码转换为十六进制字符串
    pub fn to_hex_string(encrypted_password: &[u8]) -> String {
        hex::encode(encrypted_password)
    }

    /// 从十六进制字符串解析加密密码
    pub fn from_hex_string(hex_string: &str) -> Result<Vec<u8>> {
        let clean_hex = hex_string.trim().to_lowercase();
        if clean_hex.len() != 16 {
            return Err(VncDesError::hex_decode_error(
                format!("十六进制字符串长度必须为16字符，实际长度: {}", clean_hex.len())
            ));
        }

        hex::decode(&clean_hex)
            .map_err(|e| VncDesError::hex_decode_error(format!("无法解析十六进制字符串: {}", e)))
    }

    /// 生成测试用的密码对（明文和加密后的十六进制）
    pub fn generate_test_pair(&mut self, plain_password: &str) -> Result<(String, String)> {
        let encrypted = self.encrypt_password(plain_password)?;
        let hex_string = Self::to_hex_string(&encrypted);
        Ok((plain_password.to_string(), hex_string))
    }
}

/// 密码处理器（无状态版本）
pub struct PasswordProcessor;

impl PasswordProcessor {
    /// 使用默认配置加密密码
    pub fn encrypt_with_default(password: &str) -> Result<Vec<u8>> {
        let mut processor = VncDesProcessor::default();
        processor.encrypt_password(password)
    }

    /// 使用默认配置解密密码
    pub fn decrypt_with_default(encrypted_password: &[u8]) -> Result<String> {
        let mut processor = VncDesProcessor::default();
        processor.decrypt_password(encrypted_password)
    }

    /// 使用默认配置验证密码
    pub fn verify_with_default(plain_password: &str, encrypted_password: &[u8]) -> Result<bool> {
        let mut processor = VncDesProcessor::default();
        processor.verify_password(plain_password, encrypted_password)
    }

    /// 使用自定义密钥加密密码
    pub fn encrypt_with_key(password: &str, key: &[u8; 8]) -> Result<Vec<u8>> {
        let mut processor = VncDesProcessor::with_key(*key);
        processor.encrypt_password(password)
    }

    /// 使用自定义密钥解密密码
    pub fn decrypt_with_key(encrypted_password: &[u8], key: &[u8; 8]) -> Result<String> {
        let mut processor = VncDesProcessor::with_key(*key);
        processor.decrypt_password(encrypted_password)
    }

    /// 使用自定义密钥验证密码
    pub fn verify_with_key(plain_password: &str, encrypted_password: &[u8], key: &[u8; 8]) -> Result<bool> {
        let mut processor = VncDesProcessor::with_key(*key);
        processor.verify_password(plain_password, encrypted_password)
    }

    /// 演示加密解密过程
    pub fn demo_encryption(password: &str) -> Result<()> {
        println!("🔐 VNC DES 密码加解密演示");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        let mut processor = VncDesProcessor::default();
        
        // 显示配置信息
        println!("🔧 配置信息:");
        println!("   密钥: {}", processor.config().key_as_hex());
        println!("   严格模式: {}", processor.config().strict_mode);
        println!("   自动截断: {}", processor.config().auto_truncate);
        println!("   最大密码长度: {}", processor.config().max_password_length);
        println!();

        // 加密
        println!("📝 原始密码: '{}'", password);
        let encrypted = processor.encrypt_password(password)?;
        let hex_string = VncDesProcessor::to_hex_string(&encrypted);
        println!("🔒 加密字节: {:?}", encrypted);
        println!("🔤 十六进制: {}", hex_string);
        println!();

        // 解密验证
        let decrypted = processor.decrypt_password(&encrypted)?;
        println!("🔓 解密结果: '{}'", decrypted);
        
        // 验证
        let is_valid = processor.verify_password(password, &encrypted)?;
        println!("✅ 验证结果: {}", if is_valid { "匹配" } else { "不匹配" });
        
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TIGHTVNC_DEFAULT_KEY;

    #[test]
    fn test_processor_creation() {
        let processor = VncDesProcessor::default();
        assert_eq!(processor.config().encryption_key, TIGHTVNC_DEFAULT_KEY);
    }

    #[test]
    fn test_password_encryption_decryption() {
        let mut processor = VncDesProcessor::default();
        let password = "test123";
        
        let encrypted = processor.encrypt_password(password).unwrap();
        let decrypted = processor.decrypt_password(&encrypted).unwrap();
        
        assert_eq!(password, decrypted);
    }

    #[test]
    fn test_password_verification() {
        let mut processor = VncDesProcessor::default();
        let password = "password123";
        
        let encrypted = processor.encrypt_password(password).unwrap();
        let is_valid = processor.verify_password(password, &encrypted).unwrap();
        
        assert!(is_valid);
        
        let is_invalid = processor.verify_password("wrongpass", &encrypted).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_hex_conversion() {
        let encrypted = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        let hex_string = VncDesProcessor::to_hex_string(&encrypted);
        assert_eq!(hex_string, "123456789abcdef0");
        
        let decoded = VncDesProcessor::from_hex_string(&hex_string).unwrap();
        assert_eq!(encrypted, decoded);
    }

    #[test]
    fn test_password_processor() {
        let password = "test";
        let encrypted = PasswordProcessor::encrypt_with_default(password).unwrap();
        let decrypted = PasswordProcessor::decrypt_with_default(&encrypted).unwrap();
        assert_eq!(password, decrypted);
    }

    #[test]
    fn test_custom_key() {
        let custom_key = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let password = "custom";
        
        let encrypted = PasswordProcessor::encrypt_with_key(password, &custom_key).unwrap();
        let decrypted = PasswordProcessor::decrypt_with_key(&encrypted, &custom_key).unwrap();
        
        assert_eq!(password, decrypted);
    }

    #[test]
    fn test_password_truncation() {
        let config = VncDesConfig::new()
            .with_auto_truncate(true)
            .with_max_password_length(4);
        
        let mut processor = VncDesProcessor::new(config);
        let long_password = "verylongpassword";
        
        let encrypted = processor.encrypt_password(long_password).unwrap();
        let decrypted = processor.decrypt_password(&encrypted).unwrap();
        
        assert_eq!(decrypted, "very"); // 截断为前4个字符
    }
} 