# Final Project: Building a Multithreaded Web Server

Following the tutorial from The Rust Book's last chapter: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

## TL;DR
We create our own Thread Pool implementation. When creating the pool we create one Multi-producer, single-consumer FIFO queue. The pool keeps the sender channel and each of our workers recieves a clone of the recieve channel. We allow the threads to each have a clone of the recieve channel by wrapping it in a thread-safe smart pointer (`Arc`). Additionally, each thread only gets one job by using a `Mutex`. 

Summing it up, every connection the server recives, we execute it using the thread pool. The `execute` method will complete our `handle_connection` function on a worker thread. 
Lastly the `handle_connection` simply handles the incoming `TCPStream` request and will write our response to the same `TCPStream` instance.