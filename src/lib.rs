extern crate futures;

#[cfg(test)]
mod tests {

    use std::thread;
    use std::time::Duration;
    use futures::{future, Future};

    use futures::sync::oneshot;
    use futures::sync::mpsc;
    use futures::Stream;
    use futures::Sink;

    fn expensive_computation() -> u32 {
        // ....
        thread::sleep(Duration::from_millis(10));
        200
    }

    #[test]
    fn oneshot_sender_send() {
        let (tx, rx) = oneshot::channel();
        thread::spawn(move || {
            tx.send(expensive_computation()).unwrap();
        });

        let rx = rx.map(|x| x + 3);
        let result = rx.wait().unwrap();
        assert_eq!(result, 200 + 3);
    }

    #[test]
    fn oneshot_receiver_drop_before_sender() {
        let (tx, rx) = oneshot::channel();
        drop(rx);
        thread::spawn(move || {
            let res = tx.send(expensive_computation());
            assert_eq!(res, Err(200));
        });
    }

    #[test]
    fn oneshot_sender_is_canceled() {
        let (tx, rx) = oneshot::channel::<()>();
        drop(rx);
        thread::spawn(move || {
            // 其他线程在执行复杂任务的时候可以阶段性地调用这个方法，
            // 测试rx是否已经对结果不感兴趣了，这样可以提前终止执行。
            let res = tx.is_canceled();
            assert_eq!(res, true);
        });
    }

    #[test]
    fn oneshot_sender_poll_cancel() {
        let (mut tx, rx) = oneshot::channel::<()>();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(20));
            drop(rx);
        });

        let future = future::poll_fn(|| {
            // 在future中可以使用该方法来检查是否取消了，这样在没有取消的情况下
            // 能够注册事件。而直接调用is_cancel方法不能注册.
            tx.poll_cancel()
        });

        future.wait().unwrap();
    }

    #[test]
    fn mpsc_channel() {
        let (mut tx, rx) = mpsc::channel(1);

        thread::spawn(|| {
            for i in 0..5 {
                match tx.send(i).wait() {
                    Ok(t) => tx = t,
                    Err(_) => break, // Receiver has gone away
                }
            }
        });

        let res = rx.collect().wait().unwrap();
        assert_eq!(res, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
