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
git clone https://github.com/NextVNC/vnc_des.git
cd vnc_des

# 方法1: 使用 Makefile (推荐)
make release              # 本地平台编译
make windows              # Windows 交叉编译
make macos                # macOS 交叉编译  
make linux                # Linux 交叉编译

# 方法2: 直接使用 Cargo
cargo build --release
```

#### 基本命令

```bash
# 使用 Makefile 构建后的可执行文件（推荐）
./release/aarch64-apple-darwin/bin/vnc_des_tool encrypt "password"
./release/aarch64-apple-darwin/bin/vnc_des_tool decrypt "dbd83cfd727a1458"
./release/aarch64-apple-darwin/bin/vnc_des_tool verify "password" "dbd83cfd727a1458"

# 或者使用快捷命令
make run-release

# 使用 Cargo 构建的可执行文件
./target/release/vnc_des_tool encrypt "password"
./target/release/vnc_des_tool decrypt "dbd83cfd727a1458"
./target/release/vnc_des_tool verify "password" "dbd83cfd727a1458"

# 通用命令
vnc_des_tool demo                    # 演示功能
vnc_des_tool --help                  # 查看帮助
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
├── release/                # 编译输出目录（make 构建后生成）
│   └── <platform>/         # 平台特定目录
│       ├── bin/            # 可执行文件
│       ├── lib/            # 库文件
│       ├── docs/           # 文档
│       └── examples/       # 示例脚本
├── Cargo.toml              # 项目配置
├── Makefile                # 构建配置
└── README.md               # 项目文档
```

## 🧪 测试

### 使用 Makefile（推荐）

```bash
# 运行所有测试
make test

# 运行基准测试
make bench

# 完整构建和测试
make all
```

### 使用 Cargo

```bash
# 运行所有测试
cargo test

# 运行文档测试
cargo test --doc

# 运行性能测试
cargo test --release

# 运行基准测试
cargo bench
```

## 🔧 开发

### Makefile 支持

本项目提供了功能完整的Makefile，支持多种编译模式和平台交叉编译：

```bash
# 查看所有可用命令
make help

# 编译相关
make release              # 发布模式编译（二进制 + 库）
make debug                # 调试模式编译
make release-bin          # 仅编译二进制文件
make lib                  # 仅编译库文件

# 多平台交叉编译
make windows              # Windows 平台 (x86_64-pc-windows-gnu)
make macos                # macOS 平台 (aarch64-apple-darwin)
make macos-intel          # macOS Intel (x86_64-apple-darwin)
make linux                # Linux 平台 (x86_64-unknown-linux-gnu)
make all-platforms        # 编译所有支持的平台

# 开发工具
make test                 # 运行所有测试
make bench                # 运行基准测试
make format               # 代码格式化
make lint                 # 代码检查
make check                # 快速检查项目
make docs                 # 生成文档

# 项目管理
make clean                # 清理构建产物
make info                 # 显示项目信息
make package              # 创建发布包
make install              # 安装到系统
make uninstall            # 从系统卸载

# 运行和发布
make run                  # 运行开发版本
make run-release          # 运行发布版本
make publish-check        # 检查发布准备
make publish              # 发布到 crates.io
```

#### 编译输出

使用Makefile编译后，文件会输出到 `release/<platform>/` 目录：

```
release/
└── aarch64-apple-darwin/     # 平台特定目录
    ├── bin/                  # 可执行文件
    │   └── vnc_des_tool
    ├── lib/                  # 库文件
    │   └── libvnc_des.rlib
    ├── docs/                 # 文档
    ├── examples/             # 示例脚本
    │   ├── example-encrypt.sh
    │   └── example-decrypt.sh
    ├── README.md
    └── LICENSE
```

#### 特性控制

```bash
# 启用异步特性
make release FEATURES=async

# 指定目标架构
make release TARGET_ARCH=x86_64-pc-windows-gnu

# 组合使用
make windows FEATURES=async
```

### 本地开发

```bash
# 克隆仓库
git clone https://github.com/NextVNC/vnc_des.git
cd vnc_des

# 推荐：使用 Makefile 进行开发
make all                  # 完整构建（格式化、检查、测试、编译、文档）
make quick                # 快速构建（跳过测试和文档）
make dev                  # 开发模式（自动重新编译）

# 或者直接使用 Cargo
cargo test                # 运行测试
cargo build               # 构建项目
cargo run --bin vnc_des_tool demo  # 运行示例
```

### 代码质量工具

```bash
# 使用 Makefile（推荐）
make format               # 代码格式化
make lint                 # 代码检查（clippy）
make test                 # 运行测试

# 或者直接使用 Cargo
cargo fmt                 # 代码格式化
cargo clippy              # 代码检查
cargo test                # 运行测试
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

- 提交 [Issue](https://github.com/NextVNC/vnc_des/issues)

---

**注意**: 这个实现符合VNC协议标准（RFC 6143）的DES认证算法，是VNC协议特化的DES实现，与标准DES略有不同。它与所有标准VNC实现兼容，包括TightVNC、RealVNC等。
