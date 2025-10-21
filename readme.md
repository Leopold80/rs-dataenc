

# data-enc

一个基于 Rust 开发的命令行文件加密/解密工具，支持对单个文件或整个目录进行 AES-256-GCM 加密，并可自动删除原始文件。

## 功能特性

- 使用 AES-256-GCM 算法加密文件
- 支持 SHA-256 哈希密码作为密钥
- 支持批量处理文件和目录
- 并行处理多个文件以提高性能
- 显示进度条反馈处理状态
- 加密后自动删除原文件，解密后自动删除 .enc 文件

## 安装

确保你已经安装了 Rust 和 Cargo。然后克隆项目并构建，随后找到目标可执行文件：

```bash
git clone <repository-url>
cd data-enc-src 
cargo build --release
```

## 使用方法

### 加密文件或目录

```bash
./data-enc --mode enc --passwd <your_password> <file_or_directory_path>
```

### 解密文件或目录

```bash
./data-enc --mode dec --passwd <your_password> <encrypted_file_or_directory_path>
```

### 参数说明

- `--mode` 或 `-m`: 操作模式，`enc` 表示加密，`dec` 表示解密
- `--passwd` 或 `-p`: 用于加密/解密的密码
- `<file_or_directory_path>`: 要处理的文件或目录路径

## 示例

### 加密单个文件

```bash
./data-enc --mode enc --passwd mysecretpassword ./photo.jpg
```

### 加密整个目录

```bash
./data-enc --mode enc --passwd mysecretpassword ./photos/
```

### 解密单个文件

```bash
./data-enc --mode dec --passwd mysecretpassword ./photo.jpg.enc
```

### 解密整个目录

```bash
./data-enc --mode dec --passwd mysecretpassword ./photos/
```

## 技术细节

### 加密流程

1. 使用 SHA-256 哈希算法将用户提供的密码转换为 256 位密钥
2. 生成 12 字节的随机 nonce
3. 使用 AES-256-GCM 算法加密数据
4. 将 `nonce` 和加密后的数据连接存储
5. 加密完成后删除原始文件

### 解密流程

1. 提取前 12 字节作为 `nonce`
2. 使用相同密码和 SHA-256 哈希生成密钥
3. 使用 AES-256-GCM 算法解密剩余数据
4. 解密完成后删除 `.enc` 文件

## 依赖库

- `aes-gcm`: AES-GCM 加密实现
- `sha2`: SHA-256 哈希算法
- `clap`: 命令行参数解析
- `indicatif`: 进度条显示
- `rayon`: 并行处理
- `walkdir`: 遍历目录

## 注意事项

1. 请务必记住你的密码，一旦丢失将无法恢复数据
2. 加密/解密操作会删除原始文件，请确保已备份重要数据
3. 本工具仅适用于文件加密，不适用于实时通信加密

