###说明：
- 命令：rustup override set nightly 设置为nightly版本。rocket当前版本只能使用nightly版本

### info
- 官网： https://rocket.rs/v0.4/guide/
- 静态资源嵌入binary: https://lib.rs/crates/rust-embed
- https://blog.csdn.net/u012067469/article/details/104082331/

https://zhuanlan.zhihu.com/p/133524209

### 其他
- https://github.com/ytdl-org/youtube-dl/
- https://wp.gxnas.com/4338.html
- https://github.com/oldiy/youtube-dl-webui-cn
- https://blog.csdn.net/lanchunhui/article/details/79830230


### 问题
- 如何把静态资源包含在二进制文件中：

  - 方法1：rocket提供的static_resources_initialize,但是必须包每个静态文件全部写出来，不实用
  - 方法2：rocket-embed：https://github.com/pyros2097/rust-embed 查看了官方例子，是把静态资源映射一下
  - 方法3：https://www.steadylearner.com/blog/read/How-to-serve-static-files-with-Rust 和方法2相同的原理