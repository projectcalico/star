use std::io::Read;
use std::sync::{Arc, RwLock};
use std::thread;

use collect::resource::{Resource, ResourceStore, Response};

use hyper::client::Response as HttpResponse;
use hyper::Client;
use hyper::error::Error;
//use hyper::header::Connection;
use hyper::http::RawStatus;
use mio::{EventLoop, Handler};
use rustc_serialize::json::Json;
use threadpool::ThreadPool;
use std::process::Command; 

pub fn start_client_driver(http_request_ms: u64,
                           resource_store: Arc<RwLock<ResourceStore>>) {
    info!("Starting client driver");
    let mut event_loop = EventLoop::new().unwrap();
    let _ = event_loop.timeout_ms((), http_request_ms);
    thread::spawn(move || {
        let _ = event_loop.run(&mut ClientHandler {
            http_request_ms: http_request_ms,
            resource_store: resource_store,
            thread_pool: ThreadPool::new(4),
        });
    });
}

struct ClientHandler {
    http_request_ms: u64,
    resource_store: Arc<RwLock<ResourceStore>>,
    thread_pool: ThreadPool,
}

impl Handler for ClientHandler {
    type Timeout = ();
    type Message = Resource;

    fn timeout(&mut self,
               event_loop: &mut EventLoop<ClientHandler>,
               _: ()) {
        info!("Fetching all resources");
        let loop_channel = event_loop.channel();
        for resource in self.resource_store.read().unwrap().resources() {
            let _ = loop_channel.send(resource);
        }
        let _ = event_loop.timeout_ms((), self.http_request_ms);
    }

    fn notify(&mut self,
              _: &mut EventLoop<ClientHandler>,
              resource: Resource) {
        let resource_store = self.resource_store.clone();
        self.thread_pool.execute(move || {
            info!("Fetching resource: [{}]", &resource.url);

            let client = Client::new();

	    // Call into python code.
            let res = Command::new("python").arg("/usr/bin/probe.py").arg(&resource.url).output().unwrap_or_else(|e| {
	        panic!("failed to execute process: {}", e)
            });

	    // Get success / fail.
            info!("status: {}", res.status);
	    info!("stdout: {}", String::from_utf8_lossy(&res.stdout));
	    info!("stderr: {}", String::from_utf8_lossy(&res.stderr));

            //let response_result: Result<HttpResponse, Error> =
            //    client.get(&resource.url)
            //        .header(Connection::close())
            //        .send();

            // Obtain an exclusive write lock to the status cache.
            let mut resource_store = resource_store.write().unwrap();

            match res.status.code() {
                Some(0) => {
                    //let body = &mut String::new();
                    //http_response.read_to_string(body).unwrap();
                    
                    let mut s = String::new();
		    let r = String::from_utf8(res.stdout);
                    s = r.unwrap_or(s);
		    //let body: &str = &s[..]; // Full slice of string.
                    let body: &str = &*s;
                    let body_json = Json::from_str(body);

                    if let Err(parse_error) = body_json {
                        let error_str = format!("{}", parse_error);
                        warn!("Failed to parse response body as JSON: [{}]",
                            error_str);
                        resource_store.save_response(resource, None);
                        return;
                    }

                    //let &RawStatus(status_code, _) =
                    //    http_response.status_raw();

                    let response = Response {
                        url: resource.url.clone(),
                        status_code: 0, //status_code,
                        json: body_json.unwrap(),
                    };
                    resource_store.save_response(resource, Some(response));
                },
                Some(_) => resource_store.save_response(resource, None),
                None => resource_store.save_response(resource, None),
            }
        });
    }
}

