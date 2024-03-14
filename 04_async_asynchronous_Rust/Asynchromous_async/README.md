# 1 Hour Dive into Asynchronous Rust | Ardan Labs 

- https://www.youtube.com/live/0HwrZp9CBD4?si=FLcUShZwx5pjOfmC

- https://github.com/thebracket/Ardan-1HourAsync

<hr>

# Dive into Asynchronous Rust

## What We're Going to Cover
- [1. Threads vs Async - Together or Separate](#1-threads-vs-async---together-or-separate)
- [2. Async Runtimes (Executors)](#2-async-runtimes-executors)
- [3. What does "block on" readlly mean = and the Tokio macros](#3tokio-startup-with-block-on)
- [4. Running Async Code](#hello-asyncawait)
- [5. Blocking, and Sending Blocking Tasks to Threads](#joining--simultaneous-tasks-wait-for-all)
- [6. Inter-Task Communication: Channels](#communicating-with-channels)
- [7. Streams: Async Iterators](#streams-async-iterators)
- [8. Tokio + Axum: High-Performance Web Services with Dependenc Injection and Ergonomic Development](#tokio-console-its-like-htop-for-async)
- [9. Tracing & Performance Metrics](#open-telemetry--more)
- 10. Q&A
 
# 1. Threads vs Async - Together or Separate[[🔝]](#1-hour-dive-into-asynchronous-rust--ardan-labs)

## System Threads
- Created with std::thread
- Map directly to Operating System Threads
  - 운영 체제 스레드에 직접 매핑
- Heavy-weight
  - Limited number(60,000 on my Linux system).
    - 제한된 수(Linux 시스템에서 60,000개).
  - Each thread gets a full stack
    - 각 스레드는 전체 스택을 갖습니다
  - Each thread is scheduled by the Operating System as a full entity.
    - 각 스레드는 운영 체제에 의해 완전한 개체로 예약됩니다.
  - Many thousands of threads don't scale.
    - 수천 개의 스레드가 확장되지 않습니다.
   
- Acts like "normal" code - it runs from end to end, the OS interrupts and switches threads when it decides to do so.
  - "일반" 코드처럼 작동합니다. 이 코드는 끝에서 끝까지 실행되며, OS가 스레드를 중단하고 전환하기로 결정할 때 스레드를 전환합니다.
- Great for CPU intensive work, no need to "yield" control.
  - CPU 집약적인 작업에 적합하며, 제어를 "수율"할 필요가 없습니다.

<hr>

## So Why not Use Threads for Everything?

- With a simple model of one thread per network connection, a busy server could have thousands of threads.
  - 네트워크 연결당 하나의 스레드로 구성된 간단한 모델을 사용하면 사용 중인 서버에 수천 개의 스레드가 있을 수 있습니다.
- Each thread will spend most of its time idle:각 스레드는 대부분의 시간을 유휴 상태로 보냅니다:
  - Waiting for the Network.
  - Waiting for Storage.
  - Waiting for Database Queries.
 
- Scheduler thread polling and RAM usage quickly adds up to a sluggish server.
  - 스케줄러 스레드 폴링 및 RAM 사용은 서버 부진을 빠르게 가중시킵니다
- You can select from a group of sockets to see which are ready for you.
  - 소켓 그룹에서 어떤 소켓이 준비되었는지 확인할 수 있습니다.
- This was an early form of async programming.
  - 이것은 비동기 프로그래밍의 초기 형태였습니다.

<hr>

## Asynchronous
- Async can be single-threaded or run across multiple threads.
  - 비동기는 단일 스레드이거나 여러 스레드에서 실행될 수 있습니다.
- Whenever you create an async task it is:비동기 태스크를 생성할 때마다 다음과 같습니다:
  - Lightweight(ver little memory usage)
    - 경량(메모리 사용량이 거의 없음)
  - Cooperatively Scheduled by Your Runtime
    - 런타임에 따라 협력적으로 예약
  - Easy to Cancel
    - 취소하기 쉽습니다
   
- Async queues run one task at a time.
  - 비동기 대기열은 한 번에 하나의 작업을 실행합니다.
- Your runtime thread will still be scheduled by the Operating System.
  - 런타임 스레드는 운영 체제에서 여전히 예약됩니다.

<hr>

## Async and Threading - Why Not Both?

- The larger/more powerful runtimes for Rust combine threading and async.
  - Rust의 더 크고 더 강력한 실행 시간은 스레드화와 비동기화를 결합합니다.
    - Each CPU core gets a thread (or you can customize the number of threads).
      - 각 CPU 코어는 스레드를 갖거나 스레드 수를 사용자 정의할 수 있습니다.
    - Each thread maintains a task list.
      - 각 스레드는 작업 목록을 유지합니다.
    - Threads can "steal work" from one another - if a core finds itself idle, it can reach into another queue and "steal" a job.
      - 스레드는 서로 작업을 "도둑질"할 수 있습니다. 코어가 유휴 상태인 경우 다른 대기열에 도달하여 작업을 "도둑질"할 수 있습니다.
    - The result? You are benefiting from network/storage/database latency, maximizing CPU utilization while not wasting time polling idle threads.
      - 결과는? 네트워크/스토리지/데이터베이스 대기 시간을 활용하여 CPU 활용률을 극대화하는 동시에 유휴 스레드를 폴링하는 데 시간을 낭비하지 않습니다.
     
- You can divide cores as needed:필요에 따라 코어를 분할할 수 있습니다:
  - x cores running one more more async runtimes.
    - 비동기 실행 시간을 하나 더 실행하는 x cores.
  - y cores dedicated to commute.
    - 출퇴근(갈다, 교환하다) 전용 코어입니다.
  - Communicate between the groups with high-performance channels.
    - 고성능 채널을 사용하여 그룹 간 의사소통을 수행합니다.
  - Result:
    - No I/O wait thread starvation on the tasks receiving work and sending results.
      - 작업을 수신하고 결과를 전송하는 작업에 대한 I/O 대기 스레드 부족이 없습니다.
    - High-performance cores perform CPU intensive tasks.
      - 고성능 코어는 CPU 집약적인 작업을 수행합니다.

<hr>

# 2. Async Runtimes (Executors)[[🔝]](#1-hour-dive-into-asynchronous-rust--ardan-labs)

## Rust is Agnostic - and Flexible(러스트(Rust)은 불가지론적이며 유연합니다)

- Many languages chose a predetermined async strategy.(많은 언어들이 미리 정해진 비동기 전략을 선택했습니다.)
  - This is great, so long as you are writing the type of application for which the async environment was designed.
    - 비동기 환경을 설계한 응용 프로그램 유형을 작성하는 한 좋습니다.
  - This can leads to jumping through hoops if your goals don't align with the language design.
    - 목표가 언어 디자인과 일치하지 않으면 후프를 뛰어 넘을 수 있습니다.

  - Most languages offer some tuning to mitigate this.
    - 대부분의 언어는 이를 완화하기 위해 약간의 튜닝을 제공합니다.
   
- Rust :
  - Implemented the plumbing for async in the language core (designed so that the core won't allocate memory) - and leaves the implementation to runtimes / executors.
    - 언어 코어에서 비동기를 위한 배관 작업(코어가 메모리를 할당하지 않도록 설계됨)을 구현하고 실행 시간/실행자에게 구현을 맡겼습니다.
  - This gives flexible choice of runtimes - you can even build your own.
    - 이를 통해 실행 시간을 유연하게 선택할 수 있습니다. 직접 만들 수도 있습니다.
  - Runtimes are available for small embedded projects, all the way up to high-performance targets.
    - 실행 시간은 고성능 대상까지 소규모 임베디드 프로젝트에 사용할 수 있습니다.
  - 5 of TechEmpower's 10 fastest webservers are built on Rust.
    - TechEmpower의 가장 빠른 웹 서버 10대 중 5대가 Rust를 기반으로 구축되었습니다.

<hr>

## Choosing a Runtime
- **Tokio** : a "do everything" high-performance runtime.
  - "모든 작업을 수행"하는 고성능 런타임.
- **Async-std** : Also high in features, not as popular but more focused on standardization.
  - 또한 높은 기능으로, 대중적이지는 않지만 표준화에 더 중점을 둡니다.
- **Futures** : A partial runtime, many useful utilities.
  - 부분 런타임, 유용한 유틸리티가 많습니다.
- **Smol** : A small runtime, kept simple.
- **Pasts** : A simple runtime that runs without the standard library and minimal memory allocations - works with embedded and WASM.
  - 표준 라이브러리와 최소 메모리 할당 없이 실행되는 간단한 런타임 - 내장형 및 WASM과 함께 작동합니다.

## For this talk, we're going to use **Tokio**.
- Tokio provides a broad ecosystem. and high performance. It pairs nicely with an existing stack of Hyper (for HTTP), Tower(for service management) and web servers (Axum and Actix being two popular ones).
  - Tokio는 광범위한 에코시스템과 고성능을 제공합니다. 기존의 하이퍼(HTTP용), 타워(서비스 관리용) 및 웹 서버(Axum과 Actix는 인기 있는 두 가지) 스택과 잘 어울립니다

<hr>

# All Programs Start Synchronosly모든 프로그램 동기화 시작

[code](a01_futures_async_fn/src/main.rs)

- You can't run async functions outside of an async runtime.
  - 비동기 런타임 외에는 비동기 기능을 실행할 수 없습니다.
- Most runtimes are started with a call to block_on(<<your async function>>)
  - 대부분의 런타임은 block_on(<당신의 비동기 기능>>)에 대한 호출로 시작됩니다
- This grants flexibility:(이를 통해 유연성을 확보할 수 있습니다:)
  - For an all async app, you can use one runtime.
    - 모든 비동기 앱의 경우 한 번의 런타임을 사용할 수 있습니다.
  - You can launch threads, and spawn runtimes inside them.
    - 스레드를 시작하고 그 안에 실행 시간을 생성할 수 있습니다.
  - You can even spawn a runtime just to access a single async service.
    - 단일 비동기 서비스에 액세스하기 위해 런타임을 생성할 수도 있습니다.

# 3.Tokio Startup with Block On[[🔝]](#1-hour-dive-into-asynchronous-rust--ardan-labs)
[code_block_on](a02_tokio_block_on/src/main.rs)
- Tokio also starts with `block_on` - and lets you tailor it to what you need(it also picks good defaults).
  - Tokio는 또한 'block_on'으로 시작하여 필요한 것에 맞게 조정할 수 있습니다(기본값도 양호합니다).

## Or: Quick-Start Tokio with a Macro
```rs
#[tokio::main]
async fn main() {
    hello().await;
}
```

- Tokio's #[tokio::main] macro lets you just write an async main function.
  - Tokio의 #[tokio::main] 매크로를 사용하면 비동기 주 기능만 작성할 수 있습니다.
- You can specify parameters (e.g. flavor="current_thread") to apply the same customization options.
  - 동일한 사용자 지정 옵션을 적용할 매개 변수(예: flavor="current_thread")를 지정할 수 있습니다. 
- You can even still spawn threads - and runtimes inside them.
  - 스레드를 생성하고 그 안에서 실행 시간을 설정할 수도 있습니다.

## Hello Async/Await
```rs
async fn hello() {
    println!("Hello from async");
}

#[tokio::main]
async fn main() {
    hello().await;
}
```

- Decorating a function with async changes the return type to a Future: a handle for the task, and a link to the result when it becomes available.
  - 함수를 비동기로 장식하면 반환 유형이 Future: 작업에 대한 핸들 및 결과가 사용 가능해지면 결과에 대한 링크로 변경됩니다.
- Futures don't run until you tell them to.
  - Futures은 당신이 하라고 말할 때까지 실행되지 않습니다.
- The most common way to launch an async task is to await it.
  - 비동기 작업을 시작하는 가장 일반적인 방법은 작업을 (await)기다리는 것입니다.
- When you await a task:(작업을 기다리는 경우:)
  - Your task enters a paused state.
    - 작업이 일시 중지 상태로 전환됩니다.
  - The task you are waiting for is added to the task list.
    - 대기 중인 태스크가 태스크 목록에 추가됩니다.
  - When the task finishes (it may, in turn, await) control returns to your function with the result available.
    - 작업이 완료되면
      - (it may(그럴 수도 있습니다)
      - in turn(결국.)
      - (await)
      -  제어 기능은 결과를 사용할 수 있는 상태로 기능으로 돌아갑니다.
     
## Joining : Simultaneous tasks, wait for all
[Joining__async tokio(code)](a04_tokio_join_spawn/src/main.rs)

- You can use Tokio's `join!` macro to spawn several tasks at once, and wait for all of them.
  - Tokio의 'join!' 매크로를 사용하여 여러 작업을 한 번에 생성하고 모든 작업을 기다릴 수 있습니다.
    - Results are returned in a tuple, one per task.
      - 결과는 작업당 하나씩 튜플로 반환됩니다
    - Your task waits for all of the joined tasks to complete.
      - 작업은 가입된 모든 작업이 완료될 때까지 기다립니다.
  - You can use `join_all` from futures, or Tokio's `JoinSet` to join an arbitrary vector of futures.
    - Future에서 join_all을 사용하거나 Tokio의 JoinSet을 사용하여 임의의 Vector of Future 에 가입할 수 있습니다.
   
## Spawn: detached tasks
[Spawn : datached tasks(code)](a05_tokio_spawn_detached_tasks/src/main.rs)
- Calling tokio launches an async task detached. Your task remains active, and the spawned task joins the task queue.
  - tokio를 호출하면 분리된 비동기 작업이 시작됩니다. 작업이 활성 상태로 유지되고 생성된 작업이 작업 대기열에 합류합니다.
- If you are in a multi-threaded context, the task is likely to start on another thread.
  - 다중 스레드 컨텍스트에 있는 경우 다른 스레드에서 작업이 시작될 가능성이 높습니다.

## Yielding Control
[Yielding__async tokio(code)](a06_tokio_yielding_control/src/main.rs)
- If your async funtion is doing too much, you can explicityly yield control to another task.
  - 비동기 기능이 너무 많은 작업을 수행하는 경우 다른 작업에 대한 제어 권한을 명시적으로 부여할 수 있습니다.

- When it's your function's turn to run again, it will resume where it left off.
  - 기능을 다시 실행할 차례가 되면 꺼졌던 곳에서 다시 시작됩니다.
- Yielding puts your task at the back of the work queue. If there are no other tasks, you'll keep running.
  - 양보하면 작업 대기열 뒤에 작업이 놓입니다. 다른 작업이 없으면 계속 실행됩니다.
- Yielding can be a good band-aid, but if you are doing enough to regularly slow down other tasks
  - 양보하는 것은 좋은 반창고가 될 수 있지만 정기적으로 다른 작업의 속도를 늦출 만큼 충분히 하고 있다면
    - you need blocking.
      - 차단이 필요합니다.

## Select: Process Whichever Returns First
선택: 먼저 반품하는 것을 처리하세요

- Tokio's select! Macro launches all of the listed futures - and waits for any one of them to return. The other futures are canceled.
도쿄의 선택! 매크로는 나열된 모든 미래를 시작하고 그들 중 하나가 돌아오기를 기다린다. 다른 미래는 취소되었다.

- This can be useful for:

  - Adding timeouts to operations.작업에 타임아웃 추가하기.
  - Listening to multiple data sources (channels & streams), and responding to whichever has an event ready.
    - 여러 데이터 소스(채널 및 스트림)를 듣고, 이벤트가 준비된 것에 응답합니다.
  - Retrieving data from several sources, and using whichever one answers first (the DNS or NTP model).
    - 여러 소스에서 데이터를 검색하고 먼저 응답하는 것 중 하나(DNS 또는 NTP 모델)를 사용합니다.

## The Problem with Blocking

- Async functions are cooperatively multitasked. So if you call a synchronous function that takes a while to run, you risk "blocking" the whole task system.

- Calling std:: thread: :sleep can be really bad, you might put the whole runtime to sleep!
Fortunately, Tokio includes
- tokio:: time: :sleep for async sleeping.

## Spawning Blocking Tasks

- Tokio includes a command named
spawn_blocking for launching synchronous blocking tasks.

- Blocking tasks run in a system thread, and don't pause the async runtime. You await the spawned task and your task is idle until it returns.
- You can configure the size of the blocking thread pool on runtime startup.

<hr>

# Communicating with Channels

## Inter-task Communication

- Let's create an asynchronous MPSC
(Multi-Producer, Single Consumer) channel.
- We'll send the producer to one task that periodically sends out tick messages.
- We'll also spawn a consumer that sits and waits for messages.
- Identical functionality is provided in the standard library for inter-thread communication.


## Listening to Multiple Channels

- Let's create a second channel, and a second message producer on a different cadence.
- Now we'll use select! to listen to both channels, and process whichever one has a message for us.
- Channels queue data - up to the specified maximum length - you won't lose data.


## Channels Between Threads and Tasks

- You can mix-and-match channel communication between threads and tasks - or even async runtimes - by using standard library channels and obtaining a "handle" to the runtime.

- Let's create a standard library channel, and pass the sender to a system thread along with a handle for connecting to a runtime from the outside.

- The thread sends data into the channel.

- This can work in either direction. It's useful for:

- System threads managing intensive/time critical tasks and notifying the runtime of updates.

- CPU intensive tasks can run on a dedicated thread pool, and receive tasks from (and send results to) the async system.
- This is a great way to have the best of both worlds.


## Streams: Async Iterators

- Iterators work by:

  - Storing a type they will yield.
  - Returning the next item with next) or None if the iterator is done.

- Streams are the same, but next () is an async call.
  - This gives some benefits:
    - • Iterating a huge data-set yields on each element, ensuring smooth task flow.
    - • Streams become self-pacing. Other


## Streams as Generators

- You can use a stream as a generator - it doesn't have to be streaming pre-existing content.

- Each call to poll_next() generates a new item.

- You can use this for anything from random number generation - when you need random numbers, subscribe to the stream - to unique identifiers or iterative math.


## Convert a Reader into a Stream

- Tokio's stream helpers can convert readers into streams (and vice versa).

- So you can open a file, stream the content, and File 10 becomes self-pacing - and no need to read the whole file at once.


## Simple Trace Logging

- The tracing crate has become the standard way for async and synchronous libraries to issue log messages.

- The program receiving the messages needs a tracing subscriber.

- Subscribers can print to the console, write to log files, write structured data to databases, send it to analytics.
- The default "fmt" provider prints to the console.


## Structured Logging with Tracing

- Changing log output to easily parsable JSON is easy. Change the subscriber builder.


## Instrumenting Spans

- The tracing system includes instrumentation.

- Enable span events in your output.

- Decorate functions to trace with #[instrument]

- You now have call-time information and parameters logged. You can change what it logged with parameters to the macro.

- You can also manually create spans inside your code. 


## Tokio-Console: It's like "htop" for async

- Install Tokio Console with cargo install tokio-console

- Subscribe to the console with:
console_subscriber::init();


## Open Telemetry & More

- With the tracing_opentelemetry crate, you can connect to an Open Telemetry system - and send your spans, trace points, etc.

- You can then view your distributed telemetry in the various Open Telemetry tools. Here's a screenshot of Aspecto.

<hr>

# Pinning

## What is Pinning?

- Rust's memory model strictly ensures that references must still exist, won't move, and won't be dropped while still in use.
- That's great for avoiding common memory bugs.
- It's tricky in a highly asynchronous environment, tasks may depend upon other tasks - which typically move around quite a bit.


- Pinning lets you tell Rust that a variable needs to stick around - in the same place - until you unpin it.
  - • A stream that relies upon another stream will typically pin its access to the previous stream.
  - • A select operation may need to pin entries for the same reason.
  - • Asynchronously calling yourself - recursion - requires pinning the iterations.


## Pinning and Select

- (Code from the Tokio examples)
- Of you want to create multiple futures as variables, and operate on them - you need to make sure that they will remain valid.
- Tokio's pin! macro makes this straightforward.

## Pinning and Recursion
[Recursion_async Code](async_recursion/async_recursion)

- Async recursion is difficult, because you need to pin the futures in turn.
The async_recursion crate offers an easy way.

- This won't compile:
```rs
async fn fib(n : u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fib(n-1).await + fib(n-2).await

    }
}
```

- Let the crate do its magic for you:
```rs
use async_recursion::async_recursion;

#[async_recursion]
async fn fib(n : u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fib(n-1).await + fib(n-2).await
    }
}
```

## Adapting a Stream In-Flight

- Tokio supports adapters - streams that read other streams, mutate them, and output the new stream.

- This can be a great way to modify data in-flight, and take advantage of stream self-pacing and yielding/awaiting.

- Unfortunately, using a parent stream requires some pinning gymnastics. Use the pin_project _lite crate to make it easier.

## Asyn Traits

- Traits can't - yet(it's being stabilized) - contain async functions by default.

- This won't compile:
```rs
trait MyTrait {
    async fn f() {}
}
```

- Examples taken from the async_trait crate documentation
- Using the async_trait crate does the hard work for you:

```rs
use async_trait::async_trait;

#[async_trait]
trait Advertisement {
    async fn run(&self);
}

struct Modal;

#[async_trait]
impl Advertisement for Modal {
    async fn run(&self) {
        self.render_fullscreen().await;
        for _ in 0..4u16 {
            remind_user_to_join_mailing_list().await;
        }
        self.hide_for_now().await;
    }
}

```



## Pinning Select

