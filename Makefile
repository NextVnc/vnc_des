# VNC DES 项目 Makefile
# 支持多种编译模式，统一输出到 release/ 目录

# 项目配置
PROJECT_NAME := vnc_des
BIN_NAME := vnc_des_tool
VERSION := 0.1.0
RELEASE_DIR := release
TARGET_DIR := target

# 特性配置（默认不启用特性）
FEATURES ?= 
FEATURES_FLAG := $(if $(FEATURES),--features $(FEATURES),)

# 目标平台配置
TARGET_ARCH ?= 
TARGET_FLAG := $(if $(TARGET_ARCH),--target $(TARGET_ARCH),)

# 系统检测和平台配置
UNAME_S := $(shell uname -s)

# 获取当前 Rust 工具链的实际目标架构
RUST_HOST_TARGET := $(shell rustc -vV | grep host | cut -d' ' -f2)

# 根据实际工具链设置平台信息
ifeq ($(findstring apple-darwin,$(RUST_HOST_TARGET)),apple-darwin)
    LOCAL_PLATFORM := $(RUST_HOST_TARGET)
    LOCAL_LIB_EXT := dylib
    LOCAL_EXE_EXT := 
else ifeq ($(findstring unknown-linux,$(RUST_HOST_TARGET)),unknown-linux)
    LOCAL_PLATFORM := $(RUST_HOST_TARGET)
    LOCAL_LIB_EXT := so
    LOCAL_EXE_EXT := 
else ifeq ($(findstring pc-windows,$(RUST_HOST_TARGET)),pc-windows)
    LOCAL_PLATFORM := $(RUST_HOST_TARGET)
    LOCAL_LIB_EXT := dll
    LOCAL_EXE_EXT := .exe
else
    # 备用检测（如果 rustc 不可用）
    ifeq ($(UNAME_S),Linux)
        LOCAL_PLATFORM := x86_64-unknown-linux-gnu
        LOCAL_LIB_EXT := so
        LOCAL_EXE_EXT := 
    endif
    ifeq ($(UNAME_S),Darwin)
        LOCAL_PLATFORM := aarch64-apple-darwin
        LOCAL_LIB_EXT := dylib
        LOCAL_EXE_EXT := 
    endif
    ifeq ($(OS),Windows_NT)
        LOCAL_PLATFORM := x86_64-pc-windows-msvc
        LOCAL_LIB_EXT := dll
        LOCAL_EXE_EXT := .exe
    endif
endif

# 目标平台配置（如果没有指定，使用本地平台）
ifeq ($(TARGET_ARCH),)
    PLATFORM := $(LOCAL_PLATFORM)
    LIB_EXT := $(LOCAL_LIB_EXT)
    EXE_EXT := $(LOCAL_EXE_EXT)
    TARGET_SUBDIR := $(TARGET_DIR)/release
else
    # 使用实际的 target triplet 作为平台名称
    PLATFORM := $(TARGET_ARCH)
    TARGET_SUBDIR := $(TARGET_DIR)/$(TARGET_ARCH)/release
    
    # 根据目标架构设置文件扩展名
    ifeq ($(findstring apple-darwin,$(TARGET_ARCH)),apple-darwin)
        LIB_EXT := dylib
        EXE_EXT := 
    else ifeq ($(findstring unknown-linux,$(TARGET_ARCH)),unknown-linux)
        LIB_EXT := so
        EXE_EXT := 
    else ifeq ($(findstring pc-windows,$(TARGET_ARCH)),pc-windows)
        LIB_EXT := dll
        EXE_EXT := .exe
    else
        # 默认值
        LIB_EXT := so
        EXE_EXT := 
    endif
endif

# 平台特定的库文件名格式
ifeq ($(LIB_EXT),dll)
    # Windows 平台：没有 lib 前缀
    LIB_PREFIX := 
    LIB_NAME_PATTERN := $(PROJECT_NAME).$(LIB_EXT)
    LIB_OUTPUT_NAME := $(PROJECT_NAME).$(LIB_EXT)
else
    # Unix 平台：有 lib 前缀
    LIB_PREFIX := lib
    LIB_NAME_PATTERN := lib$(PROJECT_NAME).$(LIB_EXT)
    LIB_OUTPUT_NAME := lib$(PROJECT_NAME).$(LIB_EXT)
endif

# 平台特定的 release 目录
PLATFORM_RELEASE_DIR := $(RELEASE_DIR)/$(PLATFORM)

# 默认目标
.DEFAULT_GOAL := help

# 创建 release 目录结构
create-dirs:
	@mkdir -p $(PLATFORM_RELEASE_DIR)/bin
	@mkdir -p $(PLATFORM_RELEASE_DIR)/lib
	@mkdir -p $(PLATFORM_RELEASE_DIR)/docs
	@mkdir -p $(PLATFORM_RELEASE_DIR)/examples

# 帮助信息
.PHONY: help
help: ## 显示帮助信息
	@echo "VNC DES 项目构建命令："
	@echo ""
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "编译模式："
	@echo "  debug                - 调试模式编译（快速编译，包含调试信息）"
	@echo "  release              - 发布模式编译（二进制 + 库，优化编译）"
	@echo "  release-bin          - 仅编译二进制文件（不包含库）"
	@echo "  lib                  - 仅编译库文件"
	@echo ""
	@echo "平台交叉编译："
	@echo "  windows              - Windows 平台交叉编译（x86_64-pc-windows-gnu）"
	@echo "  windows-bin          - Windows 平台交叉编译（仅 .exe）"
	@echo "  windows-lib          - Windows 平台交叉编译（仅库文件）"
	@echo "  macos                - macOS 平台交叉编译（aarch64-apple-darwin）"
	@echo "  macos-bin            - macOS 平台交叉编译（仅二进制）"
	@echo "  linux                - Linux 平台交叉编译（x86_64-unknown-linux-gnu）"
	@echo "  all-platforms        - 编译所有支持的平台"
	@echo ""
	@echo "开发工具："
	@echo "  test                 - 运行所有测试"
	@echo "  format               - 格式化代码"
	@echo "  lint                 - 运行代码检查"
	@echo "  check                - 快速检查项目（不生成代码）"
	@echo "  docs                 - 生成项目文档"
	@echo "  clean                - 清理构建产物"
	@echo "  bench                - 运行基准测试"
	@echo ""
	@echo "特性控制："
	@echo "  FEATURES=            - 禁用所有特性（默认）"
	@echo "  FEATURES=async       - 启用异步特性"
	@echo ""
	@echo "目标平台："
	@echo "  TARGET_ARCH=x86_64-pc-windows-gnu    - Windows 64位"
	@echo "  TARGET_ARCH=aarch64-apple-darwin     - macOS Apple Silicon"
	@echo "  TARGET_ARCH=x86_64-apple-darwin      - macOS Intel"
	@echo "  TARGET_ARCH=x86_64-unknown-linux-gnu - Linux 64位"
	@echo ""
	@echo "当前配置："
	@echo "  平台: $(PLATFORM)"
	@echo "  特性: $(if $(FEATURES),$(FEATURES),无)"
	@echo "  输出目录: $(PLATFORM_RELEASE_DIR)/"
	@echo ""
	@echo "示例："
	@echo "  make release                                      # 本地平台编译（二进制 + 库）"
	@echo "  make windows                                      # Windows 交叉编译"
	@echo "  make macos                                        # macOS 交叉编译"
	@echo "  make release TARGET_ARCH=x86_64-pc-windows-gnu   # 指定目标编译"
	@echo "  make release FEATURES=async                      # 启用异步特性编译"

# 调试模式编译
.PHONY: debug
debug: create-dirs ## 调试模式编译所有目标
	@echo "🔨 调试模式编译 ($(PLATFORM))..."
	@echo "   特性: $(if $(FEATURES),$(FEATURES),无)"
	@echo "   目标: $(if $(TARGET_ARCH),$(TARGET_ARCH),本地)"
	cargo build $(TARGET_FLAG) $(FEATURES_FLAG)
	@echo "📦 复制调试版本到 $(PLATFORM_RELEASE_DIR)..."
	@cp $(TARGET_SUBDIR:release=debug)/$(BIN_NAME)$(EXE_EXT) $(PLATFORM_RELEASE_DIR)/bin/$(BIN_NAME)-debug$(EXE_EXT) 2>/dev/null || true
	@cp $(TARGET_SUBDIR:release=debug)/$(LIB_NAME_PATTERN) $(PLATFORM_RELEASE_DIR)/lib/$(LIB_PREFIX)$(PROJECT_NAME)-debug.$(LIB_EXT) 2>/dev/null || true
	@cp $(TARGET_SUBDIR:release=debug)/lib$(PROJECT_NAME).rlib $(PLATFORM_RELEASE_DIR)/lib/lib$(PROJECT_NAME)-debug.rlib 2>/dev/null || true
	@echo "✅ 调试版本编译完成，输出到 $(PLATFORM_RELEASE_DIR)/"

# 发布模式编译
.PHONY: release
release: create-dirs ## 发布模式编译所有目标（优化版本，包含二进制和库）
	@echo "🚀 发布模式编译 ($(PLATFORM))..."
	@echo "   特性: $(if $(FEATURES),$(FEATURES),无)"
	@echo "   目标: $(if $(TARGET_ARCH),$(TARGET_ARCH),本地)"
	@echo "📦 编译二进制文件..."
	cargo build --release $(TARGET_FLAG) $(FEATURES_FLAG)
	@echo "📚 编译库文件..."
	cargo build --release --lib $(TARGET_FLAG) $(FEATURES_FLAG)
	@echo "📦 复制发布版本到 $(PLATFORM_RELEASE_DIR)..."
	@cp $(TARGET_SUBDIR)/$(BIN_NAME)$(EXE_EXT) $(PLATFORM_RELEASE_DIR)/bin/$(BIN_NAME)$(EXE_EXT) 2>/dev/null || true
	@cp $(TARGET_SUBDIR)/$(LIB_NAME_PATTERN) $(PLATFORM_RELEASE_DIR)/lib/$(LIB_OUTPUT_NAME) 2>/dev/null || true
	@cp $(TARGET_SUBDIR)/lib$(PROJECT_NAME).rlib $(PLATFORM_RELEASE_DIR)/lib/lib$(PROJECT_NAME).rlib 2>/dev/null || true
	@echo "📋 复制配置文件和示例..."
	@$(MAKE) copy-extras
	@echo "✅ 发布版本编译完成，输出到 $(PLATFORM_RELEASE_DIR)/"

# 仅编译二进制文件（不包含库）
.PHONY: release-bin
release-bin: create-dirs ## 仅编译二进制文件（不包含库）
	@echo "🚀 二进制文件编译 ($(PLATFORM))..."
	@echo "   特性: $(if $(FEATURES),$(FEATURES),无)"
	@echo "   目标: $(if $(TARGET_ARCH),$(TARGET_ARCH),本地)"
	cargo build --release --bin $(BIN_NAME) $(TARGET_FLAG) $(FEATURES_FLAG)
	@echo "📦 复制二进制文件到 $(PLATFORM_RELEASE_DIR)..."
	@cp $(TARGET_SUBDIR)/$(BIN_NAME)$(EXE_EXT) $(PLATFORM_RELEASE_DIR)/bin/$(BIN_NAME)$(EXE_EXT)
	@echo "📋 复制配置文件和示例..."
	@$(MAKE) copy-extras
	@echo "✅ 二进制文件编译完成，输出到 $(PLATFORM_RELEASE_DIR)/"

# 编译库文件
.PHONY: lib
lib: create-dirs ## 编译库文件版本
	@echo "📚 编译库文件 ($(PLATFORM))..."
	@echo "   特性: $(if $(FEATURES),$(FEATURES),无)"
	@echo "   目标: $(if $(TARGET_ARCH),$(TARGET_ARCH),本地)"
	cargo build --release --lib $(TARGET_FLAG) $(FEATURES_FLAG)
	@echo "📦 复制库文件到 $(PLATFORM_RELEASE_DIR)..."
	@cp $(TARGET_SUBDIR)/$(LIB_NAME_PATTERN) $(PLATFORM_RELEASE_DIR)/lib/$(LIB_OUTPUT_NAME) 2>/dev/null || true
	@cp $(TARGET_SUBDIR)/lib$(PROJECT_NAME).rlib $(PLATFORM_RELEASE_DIR)/lib/lib$(PROJECT_NAME).rlib 2>/dev/null || true
	@echo "✅ 库文件编译完成"

# 复制额外文件
.PHONY: copy-extras
copy-extras: ## 复制配置文件、文档和示例
	@echo "📋 复制额外文件..."
	@cp README.md $(PLATFORM_RELEASE_DIR)/ 2>/dev/null || true
	@cp LICENSE $(PLATFORM_RELEASE_DIR)/ 2>/dev/null || true
	@echo "#!/bin/bash" > $(PLATFORM_RELEASE_DIR)/examples/example-encrypt.sh
	@echo "# Example script to encrypt data using VNC DES" >> $(PLATFORM_RELEASE_DIR)/examples/example-encrypt.sh
	@echo "./bin/$(BIN_NAME) encrypt --key=1234567890123456 --data=\"Hello World\"" >> $(PLATFORM_RELEASE_DIR)/examples/example-encrypt.sh
	@chmod +x $(PLATFORM_RELEASE_DIR)/examples/example-encrypt.sh
	@echo "#!/bin/bash" > $(PLATFORM_RELEASE_DIR)/examples/example-decrypt.sh
	@echo "# Example script to decrypt data using VNC DES" >> $(PLATFORM_RELEASE_DIR)/examples/example-decrypt.sh
	@echo "./bin/$(BIN_NAME) decrypt --key=1234567890123456 --data=\"encrypted_data_here\"" >> $(PLATFORM_RELEASE_DIR)/examples/example-decrypt.sh
	@chmod +x $(PLATFORM_RELEASE_DIR)/examples/example-decrypt.sh

# 运行测试
.PHONY: test
test: ## 运行所有测试
	@echo "🧪 运行测试..."
	cargo test $(FEATURES_FLAG)
	@echo "✅ 测试完成"

# 运行基准测试
.PHONY: bench
bench: ## 运行基准测试
	@echo "📊 运行基准测试..."
	cargo bench $(FEATURES_FLAG)
	@echo "✅ 基准测试完成"

# Windows 平台交叉编译
.PHONY: windows
windows: ## Windows 平台交叉编译 (x86_64-pc-windows-gnu, 包含库文件和 .exe)
	@$(MAKE) release TARGET_ARCH=x86_64-pc-windows-gnu FEATURES=$(FEATURES)

# Windows 平台交叉编译（仅二进制）
.PHONY: windows-bin
windows-bin: ## Windows 平台交叉编译 (仅 .exe 文件，不包含库文件)
	@$(MAKE) release-bin TARGET_ARCH=x86_64-pc-windows-gnu FEATURES=$(FEATURES)

# Windows 平台交叉编译（仅库）
.PHONY: windows-lib
windows-lib: ## Windows 平台交叉编译 (仅库文件)
	@$(MAKE) lib TARGET_ARCH=x86_64-pc-windows-gnu FEATURES=$(FEATURES)

# macOS 平台交叉编译
.PHONY: macos
macos: ## macOS 平台交叉编译 (aarch64-apple-darwin)
	@$(MAKE) release TARGET_ARCH=aarch64-apple-darwin FEATURES=$(FEATURES)

# macOS 平台交叉编译（仅二进制）
.PHONY: macos-bin
macos-bin: ## macOS 平台交叉编译 (仅二进制文件)
	@$(MAKE) release-bin TARGET_ARCH=aarch64-apple-darwin FEATURES=$(FEATURES)

# macOS Intel 平台交叉编译
.PHONY: macos-intel
macos-intel: ## macOS Intel 平台交叉编译 (x86_64-apple-darwin)
	@$(MAKE) release TARGET_ARCH=x86_64-apple-darwin FEATURES=$(FEATURES)

# Linux 平台交叉编译
.PHONY: linux
linux: ## Linux 平台交叉编译 (x86_64-unknown-linux-gnu)
	@$(MAKE) release TARGET_ARCH=x86_64-unknown-linux-gnu FEATURES=$(FEATURES)

# 所有平台编译
.PHONY: all-platforms
all-platforms: ## 编译所有支持的平台
	@echo "🌍 编译所有平台..."
	@$(MAKE) windows
	@$(MAKE) macos
	@$(MAKE) linux
	@echo "✅ 所有平台编译完成"

# 代码格式化
.PHONY: format
format: ## 格式化代码
	@echo "🎨 格式化代码..."
	cargo fmt
	@echo "✅ 代码格式化完成"

# 代码检查
.PHONY: lint
lint: ## 运行代码检查
	@echo "🔍 运行代码检查..."
	cargo clippy -- -D warnings
	@echo "✅ 代码检查完成"

# 快速检查
.PHONY: check
check: ## 快速检查项目（不生成代码）
	@echo "✔️  检查项目状态..."
	cargo check $(TARGET_FLAG) $(FEATURES_FLAG)
	@echo "✅ 项目检查完成"

# 生成文档
.PHONY: docs
docs: create-dirs ## 生成项目文档
	@echo "📖 生成文档..."
	cargo doc --no-deps --document-private-items $(TARGET_FLAG) $(FEATURES_FLAG)
	@echo "📦 复制文档到 $(PLATFORM_RELEASE_DIR)..."
	@cp -r $(TARGET_DIR)/doc $(PLATFORM_RELEASE_DIR)/docs/ 2>/dev/null || true
	@echo "✅ 文档生成完成，位置：$(PLATFORM_RELEASE_DIR)/docs/"

# 完整构建
.PHONY: all
all: format lint test release docs ## 完整构建（格式化、检查、测试、编译、文档）
	@echo ""
	@echo "🎉 完整构建完成！"
	@echo "📁 所有产物位于：$(PLATFORM_RELEASE_DIR)/"
	@echo "   - 可执行文件：$(PLATFORM_RELEASE_DIR)/bin/$(BIN_NAME)$(EXE_EXT)"
	@echo "   - 库文件：$(PLATFORM_RELEASE_DIR)/lib/"
	@echo "   - 文档：$(PLATFORM_RELEASE_DIR)/docs/"
	@echo "   - 运行示例：$(PLATFORM_RELEASE_DIR)/examples/"

# 快速构建（跳过测试和文档）
.PHONY: quick
quick: format release ## 快速构建（跳过测试和文档）
	@echo "⚡ 快速构建完成！"

# 开发模式（调试+监视文件变化）
.PHONY: dev
dev: ## 开发模式，自动重新编译
	@echo "🔧 进入开发模式..."
	cargo watch -x "build" -x "test"

# 清理构建产物
.PHONY: clean
clean: ## 清理所有构建产物
	@echo "🧹 清理构建产物..."
	cargo clean
	@rm -rf $(RELEASE_DIR)
	@echo "✅ 清理完成"

# 更新依赖
.PHONY: update
update: ## 更新项目依赖
	@echo "📦 更新依赖..."
	cargo update
	@echo "✅ 依赖更新完成"

# 显示项目信息
.PHONY: info
info: ## 显示项目信息
	@echo "📋 项目信息："
	@echo "  名称：$(PROJECT_NAME)"
	@echo "  二进制：$(BIN_NAME)"
	@echo "  版本：$(VERSION)"
	@echo "  目标平台：$(PLATFORM)"
	@echo "  目标架构：$(if $(TARGET_ARCH),$(TARGET_ARCH),本地)"
	@echo "  启用特性：$(if $(FEATURES),$(FEATURES),无)"
	@echo "  本地平台：$(LOCAL_PLATFORM)"
	@echo "  目标目录：$(TARGET_DIR)"
	@echo "  发布目录：$(PLATFORM_RELEASE_DIR)"
	@echo "  可执行文件扩展名：$(EXE_EXT)"
	@echo "  动态库扩展名：$(LIB_EXT)"
	@echo ""
	@echo "📁 平台目录结构："
	@tree $(RELEASE_DIR) 2>/dev/null || find $(RELEASE_DIR) -type d 2>/dev/null | head -20 || echo "  $(RELEASE_DIR)/ 目录不存在，运行 'make release' 创建"

# 安装到系统（需要权限）
.PHONY: install
install: release ## 安装到系统目录
	@echo "📦 安装到系统..."
ifeq ($(findstring windows,$(PLATFORM)),windows)
	@echo "Windows 平台暂不支持自动安装，请手动复制文件到目标目录"
else
	@sudo cp $(PLATFORM_RELEASE_DIR)/bin/$(BIN_NAME)$(EXE_EXT) /usr/local/bin/
	@sudo cp $(PLATFORM_RELEASE_DIR)/lib/* /usr/local/lib/ 2>/dev/null || true
	@echo "✅ 安装完成"
endif

# 卸载
.PHONY: uninstall
uninstall: ## 从系统卸载
	@echo "🗑️  从系统卸载..."
ifeq ($(findstring windows,$(PLATFORM)),windows)
	@echo "Windows 平台暂不支持自动卸载，请手动删除相关文件"
else
	@sudo rm -f /usr/local/bin/$(BIN_NAME)$(EXE_EXT)
	@sudo rm -f /usr/local/lib/lib$(PROJECT_NAME).*
	@echo "✅ 卸载完成"
endif

# 创建发布包
.PHONY: package
package: release ## 创建发布包
	@echo "📦 创建发布包..."
	@cd $(PLATFORM_RELEASE_DIR) && tar -czf ../../$(PROJECT_NAME)-$(VERSION)-$(PLATFORM).tar.gz *
	@echo "✅ 发布包创建完成：$(PROJECT_NAME)-$(VERSION)-$(PLATFORM).tar.gz"

# 创建所有平台发布包
.PHONY: package-all
package-all: all-platforms ## 创建所有平台的发布包
	@echo "📦 创建所有平台发布包..."
	@$(MAKE) package TARGET_ARCH=x86_64-pc-windows-gnu
	@$(MAKE) package TARGET_ARCH=aarch64-apple-darwin
	@$(MAKE) package TARGET_ARCH=x86_64-unknown-linux-gnu
	@echo "✅ 所有平台发布包创建完成"

# 运行 vnc_des_tool (开发模式)
.PHONY: run
run: ## 运行 vnc_des_tool（开发模式）
	@echo "🚀 运行 $(BIN_NAME)（开发模式）..."
	cargo run --bin $(BIN_NAME) $(FEATURES_FLAG) -- --help

# 运行发布版本的 vnc_des_tool
.PHONY: run-release
run-release: release-bin ## 运行发布版本的 vnc_des_tool
	@echo "🚀 运行发布版本 $(BIN_NAME)..."
	@$(PLATFORM_RELEASE_DIR)/bin/$(BIN_NAME)$(EXE_EXT) --help

# 发布检查
.PHONY: publish-check
publish-check: ## 检查包是否准备好发布
	@echo "📋 检查发布准备..."
	cargo publish --dry-run
	@echo "✅ 发布检查完成"

# 发布到 crates.io
.PHONY: publish
publish: publish-check ## 发布到 crates.io（需要API token）
	@echo "📦 发布到 crates.io..."
	cargo publish
	@echo "✅ 发布完成" 