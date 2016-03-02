use std::sync::{Arc, RwLock};
use std::thread;
use std::option::Option;

use probe::status::StatusCache;
use std::process::Command; 
use std::time;

use hyper::client::Response;
use hyper::Client;
use hyper::error::Error;
use hyper::header::Connection;
use mio::{EventLoop, Handler};
use threadpool::ThreadPool;

pub fn start_client_driver(targets: Vec<String>,
                          http_probe_ms: u64,
                          status_cache: Arc<RwLock<StatusCache>>) {
    info!("Starting client driver");
    let mut event_loop = EventLoop::new().unwrap();
    let _ = event_loop.timeout_ms((), http_probe_ms);
    thread::spawn(move || {
        let _ = event_loop.run(&mut ClientHandler {
            targets: targets,
            http_probe_ms: http_probe_ms,
            status_cache: status_cache,
            thread_pool: ThreadPool::new(4),
        });
    });
}

struct ClientHandler {
    targets: Vec<String>,
    http_probe_ms: u64,
    status_cache: Arc<RwLock<StatusCache>>,
    thread_pool: ThreadPool,
}

impl Handler for ClientHandler {
    type Timeout = ();
    type Message = String;

    fn timeout(&mut self,
               event_loop: &mut EventLoop<ClientHandler>,
               _: ()) {
        info!("Probing all targets");
        let loop_channel = event_loop.channel();
        for target in self.targets.clone() {
            let _ = loop_channel.send(target);
        }
        let _ = event_loop.timeout_ms((), self.http_probe_ms);
    }

    fn notify(&mut self,
              _: &mut EventLoop<ClientHandler>,
              target_url: String) {
        let status_cache = self.status_cache.clone();
        self.thread_pool.execute(move || {
            info!("Probing target: [{}]", target_url);

	    // Call into python code.
            let res = Command::new("python").arg("/usr/bin/probe.py").arg(&target_url).output().unwrap_or_else(|e| {
	        panic!("failed to execute process: {}", e)
            });

	    // Get success / fail.
            info!("status: {}", res.status);
	    info!("stdout: {}", String::from_utf8_lossy(&res.stdout));
	    info!("stderr: {}", String::from_utf8_lossy(&res.stderr));
         
            // Obtain an exclusive write lock to the status cache.
            let mut status_cache = status_cache.write().unwrap();

            match res.status.code() {
                Some(0) => status_cache.reachable(target_url),
                None => status_cache.unreachable(target_url),
                Some(_) => status_cache.unreachable(target_url),
            }
        });
    }
}
