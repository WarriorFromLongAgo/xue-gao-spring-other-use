#[cfg(test)]
mod block_on_test {
    use futures::executor::block_on;

    async fn do_something() {
        println!("go go go !");
    }

    async fn hello_world() {
        println!("hello, world!");
    }

    async fn hello_world_2() {
        hello_cat();
        println!("hello, world!");
    }

    async fn hello_world_3() {
        hello_cat().await;
        println!("hello, world!");
    }

    async fn hello_cat() {
        println!("hello, kitty!");
    }

    #[test]
    fn block_on_test_1() {
        // 返回一个Future, 因此不会打印任何输出
        let future = hello_world();
        // 执行`Future`并等待其运行完成，此时"hello, world!"会被打印输出

        // `block_on`会阻塞当前线程直到指定的`Future`执行完成，这种阻塞当前线程以等待任务完成的方式较为简单、粗暴，
        // 好在其它运行时的执行器(executor)会提供更加复杂的行为，例如将多个`future`调度到同一个线程上执行。
        block_on(future);
    }

    #[test]
    fn block_on_test_2() {
        // 没有执行 hello_cat
        let future = hello_world_2();
        block_on(future);
    }

    #[test]
    fn block_on_test_3() {
        let future = hello_world_3();
        block_on(future);
    }

    struct Song {
        author: String,
        name: String,
    }

    async fn learn_song() -> Song {
        Song {
            author: "曲婉婷".to_string(),
            name: String::from("《我的歌声里》"),
        }
    }

    async fn sing_song(song: Song) {
        println!(
            "给大家献上一首{}的{} ~ {}",
            song.author, song.name, "你存在我深深的脑海里~ ~"
        );
    }

    async fn dance() {
        println!("唱到情深处，身体不由自主的动了起来~ ~");
    }

    async fn learn_and_sing() {
        // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
        let song = learn_song().await;

        // 唱歌必须要在学歌之后
        sing_song(song).await;
    }

    async fn async_main() {
        let f1 = learn_and_sing();
        let f2 = dance();

        // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
        // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
        futures::join!(f1, f2);
    }

    #[test]
    fn block_on_test_join_1() {
        block_on(async_main());
    }
}