# README
**Rust-mix**

### 目录结构
```plaintext
my_project/                  # 工作区根目录
├── Cargo.toml              # 工作区配置文件
├── apps/                   # 可执行程序（二进制包）
│   ├── cli/                # 命令行工具
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── web_server/         # Web 服务
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
├── crates/                 # 共享库（库包）
│   ├── core/               # 核心逻辑库
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── utils/              # 工具库
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── examples/               # 示例代码
│   └── demo.rs
├── tests/                  # 集成测试
│   └── integration_test.rs
└── benches/                # 基准测试
    └── benchmark.rs
```

### 构建和运行
1. 构建整个工作区
   ```bash
   # 在工作区根目录执行
   cargo build
   ```

2. 运行指定二进制
   ```bash
   cargo run -p my-cli       # 运行 `apps/cli` 的二进制
   cargo run -p web_server   # 运行 `apps/web_server` 的二进制
   ```
3. 测试特定库
   ```bash
   cargo test -p core        # 测试 `crates/core`
   cargo test -p utils       # 测试 `crates/utils`
   ```
4. 发布单个包到 crates.io
   ```bash
   cd crates/core
   cargo publish             # 单独发布 `core` 库
   ```
5. 基准测试
   ```shell
   cargo bench
   cargo bench --bench compare_algorithms
   ```

### 最佳实践
1. 模块化拆分：
   - 将可复用的代码封装到`crates/*`中。
   - 每个二进制（`apps/*`）仅负责入口逻辑，业务逻辑委托给库。

2. 依赖管理：
   - 工作区内依赖通过`path`引用（如`utils = { path = "../utils" }`）。
   - 外部依赖尽量统一版本（通过根目录的`Cargo.toml`或共享配置）。

3. 测试和文档：
   - 单元测试写在库的`src/`目录中。
   - 集成测试放在`tests/`目录。
   - 使用`cargo doc --open`生成统一文档。