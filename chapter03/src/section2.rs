/*
Section3-2: Rustを支える言語機能
*/

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub fn main() {
    /* ゼロコスト抽象化 */
    /* traitとdyn */
    // trait Tweet {
    //     fn tweet(&self);
    //     fn tweet_twice(&self) {
    //         self.tweet();
    //         self.tweet();
    //     }
    //     fn shout(&self) {
    //         println!("Uoooooooooooooohhh!!!!!!");
    //     }
    // }
    // struct Dove;
    // struct Duck;
    // impl Tweet for Dove {
    //     fn tweet(&self) {
    //         println!("Coo!");
    //     }
    // }
    // impl Tweet for Duck {
    //     fn tweet(&self) {
    //         println!("Quack!");
    //     }
    // }
    // let dove = Dove {};
    // dove.tweet();
    // dove.tweet_twice();
    // dove.shout();
    // let duck = Duck {};
    // let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    // for bird in bird_vec {
    //     bird.tweet();
    // }

    /* ジェネリクス */
    // fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    //     (t, s)
    // }
    // let t1 = make_tuple(1, 2);
    // let t2 = make_tuple("Hello", "World!");
    // let t3 = make_tuple(vec![1,2,3], vec![4, 5]);
    // let t4 = make_tuple(3, "years old");

    /* スレッド安全性 */
    /* スレッドを作る */
    let mut handles = Vec::new();
    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("Hello, world!: {}", x);
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }

    /* 共有メモリ */
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));
    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            // lockを使ってdataへの可変参照を得る
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);

    /* メッセージパッシング */
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data);
    });
    let _ = tx.send("Hello, world!");
    let _ = handle.join();

    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();
    for _ in 0..10 {
        // mainから各スレッドへのチャンネル
        let (snd_tx, snd_rx) = mpsc::channel();
        // 各スレッドからmainへのチャンネル
        let (rcv_tx, rcv_rx) = mpsc::channel();
        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);
        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }
    // 各スレッドにdataの値を送信
    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }
    // 各スレッドからの結果をdataに格納
    for x in 0..10 {
        data[x] = rcv_channels[x].recv().unwrap();
    }
    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);

    /* 非同期処理 */
    struct CountDown(u32);
    impl Future for CountDown {
        type Output = String;
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
            if self.0 == 0 {
                Poll::Ready("Zero!!!".to_string())
            } else {
                println!("{}", self.0);
                self.0 -= 1;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
    let countdown_future1 = CountDown(10);
    let countdown_future2 = CountDown(20);
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = executor::block_on(cd_set);
    for (i, s) in res.iter().enumerate() {
        println!("{}: {}", i, s);
    }

    async fn async_add(left: i32, right: i32) -> i32 {
        left + right
    }
    async fn something_great_async_function() -> i32 {
        let ans = async_add(2, 3).await; // この時点で5という値を取り出せる。
        // 何か処理をはさむこともできる。
        println!("{}", ans);
        ans
    }
    // 関数を実行する。5が出力される。
    executor::block_on(something_great_async_function());

    
}