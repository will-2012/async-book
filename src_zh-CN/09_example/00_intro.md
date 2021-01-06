# 最终项目：用异步 Rust 构建一个并发 Web 服务器

在这一章，我们会使用异步 Rust 来修改 Rust 书的 [一个单线程 web 服务器](https://doc.rust-lang.org/book/ch20-01-single-threaded.html) 来并发地服务请求


## 回顾
以下是我们那节课[^1]最后得到的代码：

`src/main.rs`:
```rust
{{#include ../../examples/09_01_sync_tcp_server/src/main.rs}}
```

`hello.html`:
```html
{{#include ../../examples/09_01_sync_tcp_server/hello.html}}
```

`404.html`:
```html
{{#include ../../examples/09_01_sync_tcp_server/404.html}}
```

如果你使用 `cargo run` 运行这个服务器，然后在浏览器中访问 `127.0.0.1:7878`，你会受到
Ferris 友好欢迎！

[^1]: 原文为 “at the end of the lesson.”，根据上下文应该是指 [官方书的 HTTP 服务器章节](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
