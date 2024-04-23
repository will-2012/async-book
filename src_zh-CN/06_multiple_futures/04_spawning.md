# `分派`

分派（Spawning）允许你在后台运行新的异步任务（task）。这允许我们在该任务运行时执行其他代码。

例如我们又一台服务器，想要它接受新连接时又不阻塞住线程。为此，我们可以用 `async_std::task::spawn` 函数来创建并运行一个新任务，来处理链接。这个函数输入一个 future，并且返回一个 `JoinHandle` 来等待这个任务（一旦它完成）的结果。

```rust,edition2018
{{#include ../../examples/06_04_spawning/src/lib.rs:example}}
```

`spawn` 返回的 `JoinHandle` 实现了 `Future` trait, 所以我们可以 `.await` 来获取任务的结果。这会阻塞当前任务，直到分派的任务完成。如果任务没有被await，你的程序会继续执行而不等待该任务，该任务也会在函数比任务先完成时被取消。

```rust,edition2018
{{#include ../../examples/06_04_spawning/src/lib.rs:join_all}}
```

为了在主任务与分派的任务间沟通，我们可以使用异步运行时提供的通道（channels）。
