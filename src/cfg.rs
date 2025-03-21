use crate::*;
use std::{
    sync::Mutex,
    thread::{JoinHandle, spawn},
    time::Instant,
};

#[test]
fn test_readme_text() {
    let mut request_builder = RequestBuilder::new().host("127.0.0.1").port(60000).build();
    request_builder
        .send("udp send".as_bytes())
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {:?}", e));
    let mut request_builder = RequestBuilder::new().host("127.0.0.1").port(60000).build();
    request_builder
        .send("udp send".as_bytes())
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {:?}", e));
}

#[test]
fn test_readme_binary() {
    let mut request_builder = RequestBuilder::new().host("127.0.0.1").port(60000).build();
    request_builder
        .send("udp send".as_bytes())
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.binary());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {:?}", e));
}

#[test]
fn test_thread_request() {
    let num_threads: i32 = 10;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let request_builder: Arc<Mutex<BoxRequestTrait>> = Arc::new(Mutex::new(
        RequestBuilder::new()
            .host("127.0.0.1")
            .port(60000)
            .timeout(10)
            .buffer(1_024_000)
            .build(),
    ));
    for _ in 0..num_threads {
        let request_builder: Arc<
            Mutex<
                Box<
                    dyn RequestTrait<
                        RequestResult = Result<
                            Box<dyn ResponseTrait<OutputText = String, OutputBinary = Vec<u8>>>,
                            RequestError,
                        >,
                    >,
                >,
            >,
        > = Arc::clone(&request_builder);
        let handle: JoinHandle<()> = spawn(move || {
            let mut request_builder = request_builder.lock().unwrap();
            let start_time: Instant = Instant::now();
            match request_builder.send("test".as_bytes()) {
                Ok(response) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    println!("{:?}", duration);
                    let response_text = response.text();
                    println!("ResponseTrait => {}", response_text);
                }
                Err(e) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    println!("{:?}", duration);
                    println!("Error => {}", e);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
