# kedis-rust

`kedis-rust` 是一个学习rust高级进阶的项目，此项目使用tokio作为线程io模型来让大家学习体验rust异步编程的魅力。

本项目参考 [Kedis](https://github.com/kwsc98/kedis) 该项目为 `kedis-java` 的java实现版本，使用netty作为线程模型，和我们介绍tokio有异曲同工的妙处，有兴趣的同学可以用来学习与参考。

同时此项目还参考 [tokio-rs/mini-redis](https://github.com/tokio-rs/mini-redis) 
但是由于该项目只是一个tokio示例项目，并没有考虑redis的各种特性，比如在命令处理时 `mini-redis` 为了满足并发需求，只是单纯的对HashMap进行加锁处理，
而我们的 kedis-rust 则在此基础上进行优化，实现了多线程的io处理，和单线程的命令处理，以满足标准的redis特性，通过此项目也是学习redis为何要实现这样的线程模型来满足高性能的要求。

## 线程模型
我们在实现一个项目时首先就是要考虑我们要用什么网络io模型，拿系统io举例我们大概可以分为三种，同步阻塞io、同步非阻塞io、异步io模型，网上有个很形象的例子，就是把io操作比作我们去商场或食堂
吃饭。

`同步阻塞io` 就是我们点完餐之后只能一直在店里等着取餐，这种方式是很简单，但是效率很低我们只能无所事事的干等着（线程阻塞）。

`同步非阻塞io` 则是我们点完菜之后并不需要一直等着，而是我们可以去点个奶茶或者逛一下街，然后时不时回来看一下我们的餐好没好，比起之前一直等是不是高效很多。

` 异步io` 则是更高大上了，我们点完餐之后，商家直接给我们一个电子号牌，当餐做好了之后店家就给电子号牌给信号，电子号牌一响你就知道餐好了，你就可以直接去取餐了，这效率就更高了。

`io多路复用` 是进一步抽象，指在单个线程中可以监听多个文件描述符也就是io事件，程序会阻塞到系统调用上去轮询这些文件操作符，当io事件被标记为可读或者可写的时候再进行相应的处理，
而这又分为三种实现模式，也是大家所熟知的select、poll、epoll，简单介绍select就是把这些文件操作符存在数组上，poll就是存在一个链表中，epoll则是把未就绪的文件操作符存储在红黑树上，
把已就绪的文件操作符存储到链表中，其中优劣相信大家都知道，此项目的java版本 `kedis-java` 使用的netty就是对系统io多路复用封装实现。

这里说的一点就是很多同学都认为netty是属于异步io，但是从上面我们对io多路复用的描述实际上使用netty再系统io层面实际上还是属于 `同步非阻塞io` 因为两个用户线程之间还需要保持一个socket长连接，
只是netty在自己的进程中使用的是Reactor模型，所以在自己进程内部是异步线程进行数据处理。

因为本项目属于一个 `学习资源` 也算是一个学习笔记，所以说的可能有些多，不感兴趣或者重点关注rust如何使用的同学可以直接看代码是如何实现的，铺垫了很多，那我们具体来看一下tokio，官方的介绍是说
异步编程框架，但实际上tokio使用的还是 `同步非阻塞io` 的理论基础。就像我们之前介绍的epoll机制一样，我们tokio中的任务线程并不是像我们介绍的 `异步io` 那样有人来通知，而是把待执行任务（Future）放到准备队列中，
让任务线程去轮询执行。当然tokio对此有更复杂的封装实现。

我们用 [Rust Course](https://course.rs/about-book.html) 中的例子来介绍 （这里推荐想了解rust或者刚入门的同学去看一下这个项目，能为你建立起对rust的基本认知）
 ```rust
use futures::executor::block_on;

async fn hello_world() {
    hello_cat().await;
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, kitty!");
}
fn main() {
    let future = hello_world();
    block_on(future);
}
 ```
这里有两个rust关键词，async，await，async可以理解为这个方法就属于一个待执行任务（Future），
当你直接 ``let future = hello_world();`` 时并不会执行方法内的代码而是会返回一个future，只有当使用特殊api执行这个Future，或者使用await关键字的时候，才会把future放入就绪队列等待任务线程执行。
上面的例子中，``hello_world（）``和 ``hello_cat（）``就被看作为两个待执行任务Future，当 ``hello_world（）``被执行时会先把``hello_cat（）``放入待执行队列，并且挂起``hello_world（）``任务，
而当``hello_cat（）``执行完毕之后，又会把 ``hello_world（）``再放入待执行队列等待执行，所以上面代码的执行结果为。
 ```rust
hello, kitty!
hello, world!
 ```
从上面的例子我们还能得出一个结论，就是在很多场景使用异步编程是没有必要的，反而会有很多线程切换的消耗，这里我初步得出一个结论，就是在我们代码最底层的Future,使用了异步io的api,我们才有必要使用异步编程也就是使用async关键字，
否则比如说CPU密集型操作，或者阻塞的io，那你任务线程还是会一直阻塞，反而多了很多线程切换的资源消耗，当然我们用来实现的是一个高性能的redis，涉及大量的io操作，那异步编程就完美适配我们这个项目。

## 网络协议
我们此项目实现了标准的resp协议，支持常见的redis客户端，resp协议可能是属于最简单的协议标准了，这也是redis高性能的特性之一，自己也可以进行快速的实现，序列化和反序列化的代码可以看本项目源码。

这里还有一个需要注意的是作为TCP协议我们不可避免会遇到粘包和半包的情况，各个语言都有不同的处理方法，但大体读书通过一个特殊定义的buffer进行处理，这个buffer具有标记的功能，我们每次反序列化的时候
尝试根据协议进行解析，每当解析成功后就把标记之前的数据清空，否则进行不下去则证明遇到了半包的情况，那我们就重置标记位置为0，然后等待再次尝试解析buffer内容。
 ```rust
pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
    loop { //一直循环尝试解析数据帧
        if let Some(frame) = self.parse_frame()? { //尝试解析数据帧
            debug!("read frame [{:?}]", frame);
            return Ok(Some(frame)); //解析成功返回数据帧
        }
        if 0 == self.stream.read_buf(&mut self.buffer).await? {
            return if self.buffer.is_empty() {
                Ok(None)
            } else {
                Err("connection reset by peer".into())
            };
        }
    }
}

fn parse_frame(&mut self) -> crate::Result<Option<Frame>> {
    let mut buf = Cursor::new(&self.buffer[..]); //实例化一个具有标记功能的buffer
    return match Frame::parse(&mut buf) {
        Ok(frame) => {
            self.buffer.advance(buf.position() as usize); //如果解析成功则将解析完的字节进行清除
            Ok(Some(frame))
        }
        Err(Error::Incomplete) => Ok(None),
        Err(Error::Other(e)) => Err(e.into()),
    };
}
 ```
具体定义解析可以学习
[redis 通信协议（RESP），最简单的应用层协议，没有之一](https://juejin.cn/post/7145819945442967583?searchId=20230917220453D76E08BDF0766CA37AF8)

推荐redis客户端
[Another Redis Desktop Manager](https://github.com/qishibo/AnotherRedisDesktopManager/releases)，这个开源客户端带有命令行模式，笔者也是用这个客户端进行调试的。




