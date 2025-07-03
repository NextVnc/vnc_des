# VNC DES - 模块化VNC密码加密库

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

一个模块化、可扩展的VNC DES密码加密/解密库，符合VNC协议标准（RFC 6143），与所有标准VNC实现兼容。既可作为Rust库使用，也可编译为独立的命令行工具。

## ✨ 特性

- 🔒 **VNC协议兼容**: 完全符合VNC协议标准（RFC 6143）的DES认证算法，兼容TightVNC、RealVNC等实现
- ⚙️ **可配置密钥**: 支持自定义密钥，兼容各种VNC实现的默认密钥（如TightVNC）
- 🏗️ **模块化设计**: 清晰的模块分离，易于扩展和维护
- 📚 **双重用途**: 既可作为库集成，也可作为独立工具使用
- 🧪 **全面测试**: 完整的单元测试和文档测试覆盖
- 📖 **丰富文档**: 详细的API文档和使用示例

## 🚀 快速开始

### 作为库使用

将以下内容添加到你的 `Cargo.toml`:

```toml
[dependencies]
vnc_des = "0.1.0"
```

#### 基本用法

```rust
use vnc_des::{VncDesProcessor, PasswordProcessor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 方法1: 使用处理器实例
    let mut processor = VncDesProcessor::default();
    let encrypted = processor.encrypt_password("password")?;
    let decrypted = processor.decrypt_password(&encrypted)?;
    assert_eq!(decrypted, "password");

    // 方法2: 使用静态方法（更简单）
    let encrypted = PasswordProcessor::encrypt_with_default("password")?;
    let hex_string = VncDesProcessor::to_hex_string(&encrypted);
    println!("加密密码: {}", hex_string);

    Ok(())
}
```

#### 自定义密钥

```rust
use vnc_des::{VncDesProcessor, VncDesConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用自定义密钥
    let custom_key = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let mut processor = VncDesProcessor::with_key(custom_key);
    
    let encrypted = processor.encrypt_password("secret")?;
    let decrypted = processor.decrypt_password(&encrypted)?;
    assert_eq!(decrypted, "secret");

    Ok(())
}
```

### 作为命令行工具使用

#### 安装

```bash
# 从源码编译
git clone https://github.com/your-repo/vnc_des.git
cd vnc_des
cargo build --release
```

#### 基本命令

```bash
# 加密密码
./target/release/vnc_des_tool encrypt "password"

# 解密密码
./target/release/vnc_des_tool decrypt "dbd83cfd727a1458"

# 验证密码
./target/release/vnc_des_tool verify "password" "dbd83cfd727a1458"

# 演示功能
./target/release/vnc_des_tool demo

# 查看帮助
./target/release/vnc_des_tool --help
```

#### 高级用法

```bash
# 使用自定义密钥
./target/release/vnc_des_tool --key "0123456789abcdef" encrypt "test"

# 显示详细信息
./target/release/vnc_des_tool -v encrypt "password"

# 静默模式（仅输出结果）
./target/release/vnc_des_tool encrypt "password" -q

# 生成配置文件
./target/release/vnc_des_tool config --generate config.json

# 从配置文件读取设置
./target/release/vnc_des_tool --key-file config.json encrypt "password"
```

## 📚 API 文档

### 核心类型

- `VncDesProcessor`: 主要的加密处理器
- `VncDesConfig`: 配置管理
- `PasswordProcessor`: 静态方法集合，无状态操作
- `VncDesEngine`: 底层DES算法引擎

### 配置选项

```rust
use vnc_des::{VncDesConfig, VncDesConfigBuilder};

let config = VncDesConfigBuilder::new()
    .hex_key("17526b06234e5807")?      // 某些VNC实现的默认密钥（如TightVNC）
    .strict_mode(false)                // 非严格模式
    .auto_truncate(true)               // 自动截断长密码
    .max_password_length(8)            // 最大密码长度
    .build()?;
```

## 🏗️ 项目结构

```
vnc_des/
├── src/
│   ├── lib.rs              # 库入口
│   ├── config.rs           # 配置管理
│   ├── error.rs            # 错误处理
│   ├── crypto/             # 加密模块
│   │   ├── mod.rs          # 模块入口
│   │   ├── des.rs          # DES算法核心
│   │   └── vnc_des.rs      # 高级处理器
│   └── bin/
│       └── vnc_des_tool.rs # 命令行工具
├── Cargo.toml              # 项目配置
└── README.md              # 项目文档
```

## 🧪 测试

运行所有测试：

```bash
cargo test
```

运行文档测试：

```bash
cargo test --doc
```

运行性能测试：

```bash
cargo test --release
```

## 🔧 开发

### 本地开发

```bash
# 克隆仓库
git clone https://github.com/your-repo/vnc_des.git
cd vnc_des

# 运行测试
cargo test

# 构建项目
cargo build

# 运行示例
cargo run --bin vnc_des_tool demo
```

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

## 📊 性能

这个实现针对性能进行了优化：

- 零拷贝设计，最小化内存分配
- 编译时常量查找表
- 高效的位操作实现
- Release模式下的LTO优化

## 🔒 安全性

- 实现了符合VNC协议标准（RFC 6143）的DES认证算法
- 与所有标准VNC实现兼容（TightVNC、RealVNC等）
- 支持安全的密钥清理
- 内存安全的Rust实现
- 完整的错误处理

## 🤝 贡献

欢迎贡献！请查看我们的贡献指南：

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- VNC协议标准（RFC 6143）提供了DES认证算法规范
- TightVNC项目提供了参考实现
- Rust社区提供了优秀的加密和CLI库

## 📞 联系

如有问题或建议，请通过以下方式联系：

- 提交 [Issue](https://github.com/your-repo/vnc_des/issues)
- 发送邮件: your-email@example.com

---

**注意**: 这个实现符合VNC协议标准（RFC 6143）的DES认证算法，是VNC协议特化的DES实现，与标准DES略有不同。它与所有标准VNC实现兼容，包括TightVNC、RealVNC等。
