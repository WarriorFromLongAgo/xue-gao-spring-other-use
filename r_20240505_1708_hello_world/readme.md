
问题解决
问题：VSCode打开项目时报错：rust-analyzer failed to load workspace
解决：项目下新建文件 setting.json，增加如下内容

{
    "rust-analyzer.server.extraEnv": {
            "CARGO": "cargo.exe"
     }
}



安装Rust时会自动安装Cargo，cargo --version 检查是否安装成功。


创建项目
cargo new r_20240505_1708_hello_world

编译
cargo build

创建可执行文件：target\debug\hello_cargo.exe
生成 cargo.lock 文件，负责追踪项目依赖的精确版本，不需要手动修改该文件


cargo run：构建和运行项目

cargo check：编译检查，但不生成可执行文件（开发过程中可反复调用，提高效率）

cargo build --release：为发布构建项目
编译时会进行优化
创建可执行文件：target\release\hello_cargo.exe


