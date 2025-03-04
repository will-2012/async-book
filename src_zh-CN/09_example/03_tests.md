# 测试 TCP 服务器

现在我们来测试我们的 `handle_connection` 函数。

首先，我们需要 `TcpStream` 来支持工作。在一个端对端或者集成测试中，我们可能需要一个真正的 TCP 连接来测试我们的代码。一个做到这样的策略是在 `localhost` 的端口 0 启动一个监听器。端口 0 并不是一个合法 UNIX 端口，但它可以用于测试。操作系统会帮我们挑一个开放的 TCP 端口。

替代的，这个示例中会给连接处理器写一个单元测试，来检查 正确的响应会返回给对应的输入。为了当我们的单元测试是隔离的以及决定性的，我们会用 mock 来替换 `TcpStream`。

首先，我们要更改 `handle_connection` 的签名，来使得它更容易测试。`handle_connection` 其实并不需要 `async_std::net::TcpStream`，它需要的是任意已经实现了 `async_std::io::Read`, `async_std::io::Write` 和 `marker::Unpin`。这样修改类型签名允许我们传递一个 mock 来测试。

```rust,ignore
use async_std::io::{Read, Write};

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
```

接下来，我们需要将建一个实现了这些 trait 的 mock `TcpStream`。首先，我们先实现 `Read` trait，只需要一个方法 `poll_read`。我们的 mock `TcpStream` 会包含一些需要拷贝到读取缓存的数据，然后我们返回 `Poll::Ready` 来表示读取已经完成。

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:mock_read}}
```

我们 `Write` trait 的实现非常简单，尽管我们需要写三个方法: `poll_write`, `poll_flush`, 和 `poll_close`。 `poll_write` 会拷贝任何输入数据到 mock `TcpStream`，然后回在完成时返回 `Poll::Ready`。没有工作需要 flush 或者 close 这个 mock `TcpStream`, 所以 `poll_flush` 和 `poll_close` 可以直接返回 `Poll::Ready`。

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:mock_write}}
```

最后，我们的 mock 还需要实现 `Unpin`，标记它所在的内存位置可以安全地转移。关于固定和 `Unpin` 的更多信息，请查看 [关于固定的章节](../04_pinning/01_chapter.md)。

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:unpin}}
```

现在我们准备好测试这个 `handle_connection` 函数了。设置好包含初始数据的 `MockTcpStream` 之后，我们能够通过属性注解 `#[async_std::test]` 执行 `handle_connection`，这很类似我们怎么使用 `#[async_std::main]`。为了保证 `handle_connection` 正常工作，我们要根据 `MockTcpStream` 的初始内容来检查正确的数据已经写入。

```rust,ignore
{{#include ../../examples/09_05_final_tcp_server/src/main.rs:test}}
```
