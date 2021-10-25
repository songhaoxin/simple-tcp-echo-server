use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

// 处理TCP连接中的输入数据流
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512]; //创建一个buf用于保存输入的数据流
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?; //获取TCP连接的输入数据流到buf中
        if bytes_read == 0 { //如果输入数据流的长度为0，直接返回该函数
            return Ok(());
        }

        stream.write(&buf[..bytes_read])?; //把接收到的数据回写到输入流中echo到客户端，如果发生错误传递该函数并返回
        thread::sleep(time::Duration::from_secs(1 as u64)); //休眠1秒中
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?; //创建TCP服务并监听127.0.0.1:8080
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new(); // 创建一个vec用于保存线程句柄

    // 对于每一个连接过来的TCP请求，创建一个单独的线程调用handle_client方法进行echo回声响应
    for stream in listener.incoming() {
        let stream = stream.expect("failed!"); //获取连接的流，当输入流错误时默认用"failed"替代
        //开启新线程，单独处理每一个TCP连接过来的输入流
        let handle = thread::spawn(move || {
            //调用handle_client处理输入数据，当t发生错误时，打印错误信息到标准错误输出窗口
            handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        thread_vec.push(handle); //把线程句柄加入到vec中
    }

    //让各线程可以异步地执行，且主线程阻塞以等待所有线程完成
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(()) //main函数的返回值
}
