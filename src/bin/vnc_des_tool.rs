#!/usr/bin/env rust
//! VNC DES Tool - VNC DES密码加解密命令行工具
//!
//! 这是一个独立的工具，用于VNC密码的加密、解密和验证
//! 支持可配置的密钥和多种操作模式
//!
//! 用法:
//!   vnc_des_tool encrypt <PASSWORD>                    # 加密明文密码为16进制
//!   vnc_des_tool decrypt <HEX_PASSWORD>                # 解密16进制密码为明文
//!   vnc_des_tool verify <PASSWORD> <HEX_PASSWORD>      # 验证密码是否匹配
//!   vnc_des_tool demo [PASSWORD]                       # 演示加解密功能
//!
//! 密钥选项:
//!   --key <HEX_KEY>                                     # 使用自定义16进制密钥
//!   --key-file <FILE>                                   # 从配置文件读取密钥
//!
//! 示例:
//!   vnc_des_tool encrypt "password123"
//!   vnc_des_tool decrypt "33483fd570cf869b"
//!   vnc_des_tool verify "password123" "33483fd570cf869b"
//!   vnc_des_tool --key "0123456789abcdef" encrypt "test"

use clap::{Arg, ArgMatches, Command};
use std::process;
use vnc_des::{
    VncDesConfig, VncDesProcessor, PasswordProcessor, VncDesError,
    TIGHTVNC_DEFAULT_KEY, info, version
};

fn main() {
    // 解析命令行参数
    let matches = build_cli().get_matches();

    // 根据子命令执行不同的操作
    let result = match matches.subcommand() {
        Some(("encrypt", sub_matches)) => handle_encrypt(sub_matches),
        Some(("decrypt", sub_matches)) => handle_decrypt(sub_matches),
        Some(("verify", sub_matches)) => handle_verify(sub_matches),
        Some(("demo", sub_matches)) => handle_demo(sub_matches),
        Some(("config", sub_matches)) => handle_config(sub_matches),
        _ => {
            eprintln!("❌ 未知命令，请使用 --help 查看帮助");
            process::exit(1);
        }
    };

    // 处理结果
    match result {
        Ok(_) => {
            // 成功时什么都不做，输出已在各个函数中处理
        }
        Err(e) => {
            eprintln!("❌ 操作失败: {}", e);
            process::exit(1);
        }
    }
}

/// 构建命令行接口
fn build_cli() -> Command {
    Command::new("vnc_des_tool")
        .version(version())
        .about("VNC DES密码加密/解密工具")
        .long_about(format!(
            "{}\n\n一个模块化、可扩展的VNC DES密码处理工具，支持可配置的密钥和多种操作模式。", 
            info()
        ))
        .subcommand_required(true)
        .arg_required_else_help(true)
        
        // 全局选项
        .arg(
            Arg::new("key")
                .long("key")
                .value_name("HEX_KEY")
                .help("使用自定义16进制密钥（16字符）")
                .global(true)
        )
        .arg(
            Arg::new("key_file")
                .long("key-file")
                .value_name("FILE")
                .help("从配置文件读取密钥")
                .global(true)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("显示详细信息")
                .action(clap::ArgAction::SetTrue)
                .global(true)
        )
        
        // 加密子命令
        .subcommand(
            Command::new("encrypt")
                .about("加密明文密码为16进制格式")
                .long_about("将明文密码加密为VNC兼容的16进制格式，用于配置存储")
                .arg(
                    Arg::new("password")
                        .help("要加密的明文密码")
                        .value_name("PASSWORD")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .help("静默模式，仅输出结果")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        
        // 解密子命令
        .subcommand(
            Command::new("decrypt")
                .about("解密16进制密码为明文格式")
                .long_about("将16进制格式的加密密码解密为明文密码")
                .arg(
                    Arg::new("hex_password")
                        .help("16进制格式的加密密码（16个字符）")
                        .value_name("HEX_PASSWORD")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .help("静默模式，仅输出结果")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        
        // 验证子命令
        .subcommand(
            Command::new("verify")
                .about("验证明文密码与16进制密码是否匹配")
                .long_about("验证明文密码加密后是否与给定的16进制密码匹配")
                .arg(
                    Arg::new("password")
                        .help("明文密码")
                        .value_name("PASSWORD")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("hex_password")
                        .help("16进制格式的加密密码")
                        .value_name("HEX_PASSWORD")
                        .required(true)
                        .index(2)
                )
                .arg(
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .help("静默模式，仅输出结果")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        
        // 演示子命令
        .subcommand(
            Command::new("demo")
                .about("演示VNC DES密码加解密功能")
                .long_about("展示VNC DES密码加解密的完整流程和相关信息")
                .arg(
                    Arg::new("password")
                        .help("用于演示的密码（可选，默认使用 'demo123'）")
                        .value_name("PASSWORD")
                        .index(1)
                )
        )
        
        // 配置子命令
        .subcommand(
            Command::new("config")
                .about("显示或管理配置信息")
                .long_about("显示当前配置信息或生成配置文件")
                .arg(
                    Arg::new("show")
                        .long("show")
                        .help("显示当前配置")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("generate")
                        .long("generate")
                        .value_name("FILE")
                        .help("生成配置文件")
                )
                .arg(
                    Arg::new("validate")
                        .long("validate")
                        .value_name("FILE")
                        .help("验证配置文件")
                )
        )
}

/// 创建VNC DES处理器（根据命令行参数）
fn create_processor(matches: &ArgMatches) -> Result<VncDesProcessor, VncDesError> {
    // 检查是否指定了自定义密钥
    if let Some(hex_key) = matches.get_one::<String>("key") {
        if matches.get_flag("verbose") {
            println!("🔧 使用自定义密钥: {}", hex_key);
        }
        return VncDesProcessor::with_hex_key(hex_key);
    }
    
    // 检查是否指定了配置文件
    if let Some(config_file) = matches.get_one::<String>("key_file") {
        if matches.get_flag("verbose") {
            println!("🔧 从文件加载配置: {}", config_file);
        }
        let config = VncDesConfig::from_file(config_file)?;
        return Ok(VncDesProcessor::new(config));
    }
    
    // 使用默认配置
    if matches.get_flag("verbose") {
        println!("🔧 使用默认VNC密钥: {}", hex::encode(TIGHTVNC_DEFAULT_KEY));
    }
    Ok(VncDesProcessor::default())
}

/// 处理加密命令
fn handle_encrypt(matches: &ArgMatches) -> Result<(), VncDesError> {
    let password = matches.get_one::<String>("password").unwrap();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");

    let mut processor = create_processor(matches)?;
    
    // 加密密码
    let encrypted = processor.encrypt_password(password)?;
    let hex_string = VncDesProcessor::to_hex_string(&encrypted);

    if quiet {
        // 静默模式，仅输出结果
        println!("{}", hex_string);
    } else {
        // 详细模式，显示完整信息
        println!("🔐 VNC DES 密码加密");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        if verbose {
            println!("🔧 使用密钥: {}", processor.config().key_as_hex());
        }
        
        println!("📝 原始密码: '{}'", password);
        if password.len() > processor.config().max_password_length {
            let truncated = &password[..processor.config().max_password_length];
            println!("⚠️  警告: 密码长度超过{}字符，已截断为: '{}'", 
                processor.config().max_password_length, truncated);
        }
        
        if verbose {
            println!("🔒 加密字节: {:?}", encrypted);
        }
        println!("🔤 十六进制: {}", hex_string);
        println!("✅ 加密完成");
        
        // 验证加密正确性
        if verbose {
            match processor.decrypt_password(&encrypted) {
                Ok(decrypted) => {
                    let expected = if password.len() > processor.config().max_password_length {
                        &password[..processor.config().max_password_length]
                    } else {
                        password
                    };
                    
                    if decrypted == expected {
                        println!("✅ 验证: 加密解密一致");
                    } else {
                        println!("⚠️  验证: 加密解密不一致");
                    }
                }
                Err(e) => {
                    println!("❌ 验证失败: {}", e);
                }
            }
        }
    }

    Ok(())
}

/// 处理解密命令
fn handle_decrypt(matches: &ArgMatches) -> Result<(), VncDesError> {
    let hex_password = matches.get_one::<String>("hex_password").unwrap();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");

    let mut processor = create_processor(matches)?;

    // 清理输入（移除空格，转为小写）
    let clean_hex = hex_password.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase();

    if !quiet {
        println!("🔓 VNC DES 密码解密");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        if verbose {
            println!("🔧 使用密钥: {}", processor.config().key_as_hex());
        }
        
        println!("🔤 输入十六进制: {}", hex_password);
        if clean_hex != *hex_password {
            println!("🧹 清理后格式: {}", clean_hex);
        }
    }

    // 解析十六进制并解密
    let encrypted = VncDesProcessor::from_hex_string(&clean_hex)?;
    let decrypted = processor.decrypt_password(&encrypted)?;

    if quiet {
        // 静默模式，仅输出结果
        println!("{}", decrypted);
    } else {
        if verbose {
            println!("🔒 加密字节: {:?}", encrypted);
        }
        println!("🔓 解密结果: '{}'", decrypted);
        println!("✅ 解密完成");
    }

    Ok(())
}

/// 处理验证命令
fn handle_verify(matches: &ArgMatches) -> Result<(), VncDesError> {
    let password = matches.get_one::<String>("password").unwrap();
    let hex_password = matches.get_one::<String>("hex_password").unwrap();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");

    let mut processor = create_processor(matches)?;

    if !quiet {
        println!("🔍 VNC DES 密码验证");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        if verbose {
            println!("🔧 使用密钥: {}", processor.config().key_as_hex());
        }
        
        println!("📝 明文密码: '{}'", password);
        println!("🔤 加密密码: {}", hex_password);
    }

    // 解析十六进制
    let clean_hex = hex_password.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase();
    
    let encrypted = VncDesProcessor::from_hex_string(&clean_hex)?;
    
    // 验证密码
    let is_match = processor.verify_password(password, &encrypted)?;

    if quiet {
        // 静默模式，输出布尔值
        println!("{}", is_match);
        if !is_match {
            process::exit(1);
        }
    } else {
        if is_match {
            println!("✅ 验证结果: 密码匹配");
        } else {
            println!("❌ 验证结果: 密码不匹配");
            
            if verbose {
                // 显示实际加密的结果用于调试
                let actual_encrypted = processor.encrypt_password(password)?;
                let actual_hex = VncDesProcessor::to_hex_string(&actual_encrypted);
                println!("🔍 实际加密结果: {}", actual_hex);
                println!("🔍 预期加密结果: {}", clean_hex);
            }
            
            process::exit(1);
        }
    }

    Ok(())
}

/// 处理演示命令
fn handle_demo(matches: &ArgMatches) -> Result<(), VncDesError> {
    let password = matches.get_one::<String>("password")
        .map(|s| s.as_str())
        .unwrap_or("demo123");

    let processor = create_processor(matches)?;
    
    println!("🎯 VNC DES 功能演示");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // 显示库信息
    println!("📚 库信息: {}", info());
    println!();
    
    // 显示配置信息
    println!("🔧 当前配置:");
    println!("   密钥: {}", processor.config().key_as_hex());
    println!("   严格模式: {}", processor.config().strict_mode);
    println!("   自动截断: {}", processor.config().auto_truncate);
    println!("   最大密码长度: {}", processor.config().max_password_length);
    println!();

    // 执行演示
    PasswordProcessor::demo_encryption(password)?;

    // 显示更多示例
    println!();
    println!("💡 更多用法示例:");
    println!("   # 使用自定义密钥加密");
    println!("   vnc_des_tool --key \"0123456789abcdef\" encrypt \"test\"");
    println!();
    println!("   # 验证密码");
    println!("   vnc_des_tool verify \"password\" \"33483fd570cf869b\"");
    println!();
    println!("   # 从配置文件读取设置");
    println!("   vnc_des_tool --key-file config.json encrypt \"password\"");

    Ok(())
}

/// 处理配置命令
fn handle_config(matches: &ArgMatches) -> Result<(), VncDesError> {
    if matches.get_flag("show") {
        // 显示当前配置
        let processor = create_processor(matches)?;
        let config = processor.config();
        
        println!("🔧 当前配置信息");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("密钥 (16进制): {}", config.key_as_hex());
        println!("严格模式: {}", config.strict_mode);
        println!("自动截断: {}", config.auto_truncate);
        println!("最大密码长度: {}", config.max_password_length);
        
        println!();
        println!("配置JSON格式:");
        println!("{}", config.to_json()?);
        
        return Ok(());
    }
    
    if let Some(file_path) = matches.get_one::<String>("generate") {
        // 生成配置文件
        let config = VncDesConfig::default();
        config.save_to_file(file_path)?;
        
        println!("✅ 配置文件已生成: {}", file_path);
        println!("📝 内容:");
        println!("{}", config.to_json()?);
        
        return Ok(());
    }
    
    if let Some(file_path) = matches.get_one::<String>("validate") {
        // 验证配置文件
        match VncDesConfig::from_file(file_path) {
            Ok(config) => {
                println!("✅ 配置文件有效: {}", file_path);
                println!("🔧 配置内容:");
                println!("   密钥: {}", config.key_as_hex());
                println!("   严格模式: {}", config.strict_mode);
                println!("   自动截断: {}", config.auto_truncate);
                println!("   最大密码长度: {}", config.max_password_length);
            }
            Err(e) => {
                println!("❌ 配置文件无效: {}", e);
                process::exit(1);
            }
        }
        
        return Ok(());
    }
    
    // 如果没有指定任何选项，显示帮助
    println!("请使用以下选项之一:");
    println!("  --show           显示当前配置");
    println!("  --generate FILE  生成配置文件");
    println!("  --validate FILE  验证配置文件");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_build() {
        let app = build_cli();
        app.debug_assert();
    }

    #[test]
    fn test_processor_creation() {
        // 测试默认处理器创建（使用真实的CLI结构）
        let app = build_cli();
        let matches = app.try_get_matches_from(vec!["vnc_des_tool", "demo"]).unwrap();
        let processor = create_processor(&matches).unwrap();
        assert_eq!(processor.config().encryption_key, TIGHTVNC_DEFAULT_KEY);
    }
} 