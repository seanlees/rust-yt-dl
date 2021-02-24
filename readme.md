###说明：
- 命令：rustup override set nightly 设置为nightly版本。rocket当前版本只能使用nightly版本
- windows上debug的用这个：rustup override set nightly-x86_64-pc-windows-gnu  , rustup override unset
-sqlite:  现使用：libsqlite3-sys = { version = "0.9", features = [ "bundled" ] } ； 可尝试：vcpkg install sqlite3:x64-windows

rustup override set 1.0.0
To see the active toolchain use rustup show. To remove the override and use the default toolchain again, rustup override unset
### info
- 官网： https://rocket.rs/v0.4/guide/
- 静态资源嵌入binary: https://lib.rs/crates/rust-embed
- https://blog.csdn.net/u012067469/article/details/104082331/
- tera官网： https://tera.netlify.app/docs/  https://github.com/Keats/tera
- diesel官网：https://docs.diesel.rs/diesel/ https://github.com/diesel-rs

https://zhuanlan.zhihu.com/p/133524209

### 其他
- https://github.com/ytdl-org/youtube-dl/
- https://wp.gxnas.com/4338.html
- https://github.com/oldiy/youtube-dl-webui-cn
- https://blog.csdn.net/lanchunhui/article/details/79830230

### diesel 使用
- 安装diesel-cli， 官网：https://github.com/diesel-rs/diesel/tree/master/diesel_cli
- rocket默认集成diesel, 网址：https://api.rocket.rs/v0.4/rocket_contrib/databases/index.html#usage


### 问题
- 如何把静态资源包含在二进制文件中：
  example: https://github.com/pyros2097/rust-embed/blob/master/examples/rocket.rs

  - 方法1：rocket提供的static_resources_initialize,但是必须包每个静态文件全部写出来，不实用
  - 方法2：rocket-embed：https://github.com/pyros2097/rust-embed 查看了官方例子，是把静态资源映射一下
  - 方法3：https://www.steadylearner.com/blog/read/How-to-serve-static-files-with-Rust 和方法2相同的原理