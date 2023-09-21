# kedis-rust

`kedis-rust` 是一个学习rust高级进阶的项目，此项目使用tokio作为线程io模型，实现了一个多线程io处理，单线程命令处理，具有渐进式rehash的高性能redis。

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

## 多线程IO处理
我们知道Redis在最近几个版本支持了多线程操作，实际上之前也支持，比如说持久化/异步删除等功能，但redis6开始支持了多线程处理io事件和协议的序列化和反序列化，那我们当然要与时俱进，tokio实际上也支持多线程并发处理，废话不多说我们之间看下面代码。
 ```rust
impl Listener { //端口进行监听
    async fn run(&mut self) -> crate::Result<()> {
        loop {
            self.limit_connections.acquire().await.unwrap().forget();
            let socket = self.accept().await?; //这里就是等待新的socket连接建立
            debug!("receive new connect");
            let mut handler = Handler { //连接建立成功我们生成一个handler处理器来专门对这个socket连接进行处理
                handler_name: None,
                buffer: Buffer::new(socket),
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
                limit_connections: self.limit_connections.clone(),
                _shutdown_complete: self.shutdown_complete_tx.clone(),
                db_sender: self.db_handler.as_ref().get_sender(0).unwrap(),
                db_handler: self.db_handler.clone(),
            };
            tokio::spawn(async move { //然后将handler移交给tokio线程池去处理
                if let Err(err) = handler.run().await {
                    error!(cause = ?err, "handler error");
                }
            });
        }
    }
}
 ```
从上门的代码我们可以看出来，我们的main线程一直在轮询尝试获取新的socket连接，获取到之后我们就将连接进行包装然后移交给tokio线程池进行处理，光看这段代码实际上是不是就像我们之前说的io多路复用模型，我们一个main线程不停的轮询去处理新建立的连接，
实际上tokio线程池底层也是如此实现的，我们所谓的异步线程也是一直以轮询的方式去处理待处理的Future，这样做的好处就是减少了线程睡眠，唤醒的资源消耗，同时也简化了流程，当然tokio在此实现上是更加复杂的，其中一个核心的问题就是如何处理两个线程获取到同一个任务Future的情况，
目前tokio是采用调度策略（例如工作窃取算法）来选择任务进行执行，当然现在rust和tokio还在不断的优化更新，相信之后会有更优的处理方法。

## 单线程命令处理
redis快的原因大家都知道是内存存储，当然这个是很重要的原因，但是大家很多时候会忽略为什么redis要使用单线程模型来对命令进行处理，我们都知道，在java的各种标准库里，使用的对象大体可分为两种就是线程安全，和线程不安全，就那hashMap来说，
如果当两个线程同时往一个bucket桶插入数据的时候，那么有可能就会造成元素丢失的情况，那么这时我们的处理方法就是只能想办法把hashMap变为线程安全的类，那么最简单的方式就是给它加上一个锁。就比如`mini-redis`这个项目。
 ```rust
#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
    background_task: Notify,
}
#[derive(Debug)]
struct State {
    entries: HashMap<String, Entry>,
    pub_sub: HashMap<String, broadcast::Sender<Bytes>>,
    expirations: BTreeSet<(Instant, String)>,
    shutdown: bool,
}
 ```
`mini-redis`就是通过添加一个读占锁Mutex来实现并发安全的，那显而易见的就是锁是一个很耗费资源的东西涉及到锁的竞争和线程的切换，而我们对内存的读取写入确是非常快的，有可能锁带来的线程切换的消耗都比我们业务处理所消耗的资源高，大家可能对内存读取速度还没有概念，我们举个例子。
 ```rust
#[test]
fn dict_test_1() {
    let start_time = DateUtil::get_now_date_time_as_millis();
    let mut dict = Dict::new(2);
    for idx in 0..1000000 {
        dict.insert(idx, idx + 1);
    }
    let mut res = 0;
    for idx in 0..1000000 {
        if let Some(value) = dict.get(&idx) {
            res += 1;
            assert_eq!(idx, value.value.unwrap() - 1);
        }
    }
    assert_eq!(1000000, res);
    println!(
        "dict_test_1 done run time (millis): {}",
        DateUtil::get_now_date_time_as_millis() - start_time
    );
}
#[test]
fn dict_test_2() {
    let start_time = DateUtil::get_now_date_time_as_millis();
    let mut dict = HashMap::with_capacity(2);
    for idx in 0..1000000 {
        dict.insert(idx, idx + 1);
    }
    let mut res = 0;
    for idx in 0..1000000 {
        if let Some(value) = dict.get(&idx) {
            res += 1;
            assert_eq!(idx, *value - 1);
        }
    }
    assert_eq!(1000000, res);
    println!(
        "dict_test_2 done run time (millis): {}",
        DateUtil::get_now_date_time_as_millis() - start_time
    );
}
 ```
执行结果
 ```rust
 dict_test_1 done run time (millis): 1306
 dict_test_2 done run time (millis): 824
 ```
这第一个例子是我自己实现的渐进式刷新dict，写一百万数据，然后再读一百万数据所耗费的时间，我们可以看到耗费了1300毫秒，而标准库的hashMap甚至只用了800毫秒的时间，可想而知就算是单线程处理内存的读写实际上也是非常快的，这就是为什么redis使用单线程模型的原因。

那么我们如果实现一个没有锁竞争和线程切换的单线程模型呢？答案很简单，也是我们之前提到好多次的方法，那就是轮询，我们可以将redis每个db分配一个线程，而每个db循环去消费命令不就好了吗？我们看一下代码。
 ```rust
pub struct Handler {
    handler_name: Option<String>,
    buffer: Buffer,
    db_sender: crate::MpscSender,
    db_handler: Arc<DbHandler>,
}
 ```
我们首先看Handler，一个Handler代表着一个socket连接，而我们这里结构体存储的并不是一个db的实例，而是存储着一个db_sender，就是一个发送者。
 ```rust
impl Handler {
    async fn run(&mut self) -> crate::Result<()> {
        loop {
            let frame = tokio::select! {
                res = self.buffer.read_frame() => res?, //解析数据帧
                _ = self.shutdown.recv() => {
                    return Ok(());
                }
            };
            if let Some(frame) = frame {
                let result_cmd = Command::from_frame(frame); //将数据帧转换为命令
                let result_frame = match result_cmd { //如果命令是系统命令则直接执行
                    Ok(cmd) => match cmd {
                        Command::Unknown(unknown) => unknown.apply(),
                        Command::Info(info) => info.apply(),
                        Command::Ping(ping) => ping.apply(),
                        Command::Select(select) => select.apply(self),
                        Command::Config(config) => config.apply(self),
                        Command::Client(client) => client.apply(self),
                        Command::Quit(_quit) => {
                            self.shutdown.shutdown();
                            break;
                        }
                        _ => { //如果为缓存数据操作命令则通过消息管道发送至db处理
                            let (sender, receiver) = oneshot::channel(); //这里是再生成一个管道作为消息命令处理的结果通知
                            self.db_sender.send((sender, cmd)).await?; //将待处理命令和回调发送者一同传给db线程
                            receiver.await? //异步等待命令的结果回调
                        }
                    },
                    Err(err_info) => Ok(Frame::Error(err_info.to_string())),
                };
                let frame = match result_frame {
                    Ok(frame) => frame,
                    Err(err_info) => Frame::Error(err_info.to_string()),
                };
                self.buffer.write_frame(&frame).await?;
            }
        }
        return Ok(());
    }
}
impl Db {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(1024);
        return Db {
            dict: Dict::new(1024),
            sender: tx,
            receiver: rx,
        };
    }
    async fn run(&mut self) {
        while let Some((sender, command)) = self.receiver.recv().await { //接受者一直从循环队列里读取命令
            let frame = match command { //处理任务
                Command::Get(get) => get.apply(self),
                Command::Set(set) => set.apply(self),
                Command::Scan(scan) => scan.apply(self),
                Command::Type(scan) => scan.apply(self),
                Command::Ttl(ttl) => ttl.apply(self),
                _ => Err("Error".into()),
            };
            let _ = sender.send(frame); //再通过Handler生成的发送者，把处理结果发送回去
        }
    }
}
 ```
这里我们实际上用到的就是线程直接数据共享的一种方式，“消息通道”，而消息通道又分为多种方式，比如说多发送者，单消费者，或者单生产者，单消费者，我们这里用到的就是多发送者，单消费者的模式，具体有兴趣的同学可以移步
 [Rust Course](https://course.rs/about-book.html) ，或者直接看源码学习。

 这里我们Handler每次解析成功数据帧后，会将数据帧再解析为命令，之后判断命令是否为系统命令，比如说ping，select，info，quit这种命令，如果是系统命令那我们之间在当前Handler线程直接进行处理，如果是需要操作db的命令，
 那我们就需要发命令发送给db线程进行处理，这里需要注意，因为消息通道并不是双向的，所以我们还需要生成一对生产者和消费者通道，用来进行命令处理结果的回调，而我们的db线程则是，一直通过接受者进行轮询处理，到此为止，我们实际上
 就实现了多线程io处理+单线程命令处理的线程模型。


