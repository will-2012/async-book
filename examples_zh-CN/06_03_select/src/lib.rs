#![cfg(test)]
#![recursion_limit="128"]

mod example {
// ANCHOR: example
use futures::{
    future::FutureExt, // 为了 `.fuse()`
    pin_mut,
    select,
};

async fn task_one() { /* ... */ }
async fn task_two() { /* ... */ }

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    select! {
        () = t1 => println!("task one completed first"),
        () = t2 => println!("task two completed first"),
    }
}
// ANCHOR_END: example
}

mod default_and_complete {
// ANCHOR: default_and_complete
use futures::{future, select};

async fn count() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => unreachable!(), // 永远不会被执行(futures都准备好了,然后complete分支被执行)
        };
    }
    assert_eq!(total, 10);
}
// ANCHOR_END: default_and_complete

#[test]
fn run_count() {
    futures::executor::block_on(count());
}
}

mod fused_stream {
// ANCHOR: fused_stream
use futures::{
    stream::{Stream, StreamExt, FusedStream},
    select,
};

async fn add_two_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}
// ANCHOR_END: fused_stream
}

mod fuse_terminated {
// ANCHOR: fuse_terminated
use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    stream::{FusedStream, Stream, StreamExt},
    pin_mut,
    select,
};

async fn get_new_num() -> u8 { /* ... */ 5 }

async fn run_on_new_num(_: u8) { /* ... */ }

async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // 计时器已经完成了.
                // 如果没有`get_new_num_fut`正在执行的话,就启动一个新的.
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                // 一个新的数字到达了
                // 启动一个新的`run_on_new_num_fut`并且扔掉旧的.
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            // 执行`run_on_new_num_fut`
            () = run_on_new_num_fut => {},
            // 当所有都完成时panic,
            // 因为理论上`interval_timer`会不断地产生值.
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}
// ANCHOR_END: fuse_terminated
}

mod futures_unordered {
// ANCHOR: futures_unordered
use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
    pin_mut,
    select,
};

async fn get_new_num() -> u8 { /* ... */ 5 }

async fn run_on_new_num(_: u8) -> u8 { /* ... */ 5 }

// 用从`get_new_num`获取的最新的数字运行`run_on_new_num`.
//
// 每当定时器到期后,都会重新执行`get_new_num`,
// 并立即取消正在执行的`run_on_new_num`,随后用新返回值替换`run_on_new_num`.
async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let mut run_on_new_num_futs = FuturesUnordered::new();
    run_on_new_num_futs.push(run_on_new_num(starting_num));
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // 计时器已经完成了.
                // 如果没有`get_new_num_fut`正在执行的话,就启动一个新的.
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                // 一个新的数字到达了,启动一个新的`run_on_new_num_fut`.
                run_on_new_num_futs.push(run_on_new_num(new_num));
            },
            // 执行`run_on_new_num_futs`并检查有没有完成的.
            res = run_on_new_num_futs.select_next_some() => {
                println!("run_on_new_num_fut returned {:?}", res);
            },
            // 当所有都完成时panic,
            // 因为理论上`interval_timer`会不断地产生值.
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}

// ANCHOR_END: futures_unordered
}
