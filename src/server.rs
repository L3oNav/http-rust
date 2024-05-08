use core::panic;
use std::{
    fs,
    io::prelude::*, 
    net::{TcpStream, TcpListener},
    sync::Arc,
    
};

mod handlers;
mod http;
mod request;
mod routing;

use http::HttpMethod;
use request::Request;
use routing::{Route, Router};

use crate::threading::ThreadPool;


const DIR_FLAG: &str = "--directory";

pub struct Server {
    listener: TcpListener,
    router: Arc<Router>,
    pool: ThreadPool,
    cfg: Arc<Cfg>,
}

struct Cfg {
    files_dir: Option<String>,
}

// TODO: add shutdown
impl Server {

    pub fn setup(addr: &str, pool_size: usize, args: &[String]) -> Server {
        let listener = TcpListener::bind(addr).unwrap();
        let router = Arc::new(setup_router());
        let cfg = Arc::new(setup_cfg(&args));
        let pool = ThreadPool::new(pool_size);

        Server {
            listener,
            router,
            cfg,
            pool,
        }
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router = Arc::clone(&self.router);
                    let cfg = Arc::clone(&self.cfg);
                    self.pool.execute(move || {
                        Server::handle_connection( stream, &router, &cfg);
                    }) 
                },
    
                Err(e) => {
                    println!("stream error: {}", e);
                },
            }
        }
    }

    fn handle_connection(
        mut stream: TcpStream,
        router: &Router,
        cfg: &Cfg,
    ) {
        match Request::parse(&stream) {
            Ok(request) => {
                let handler = router.get_handler(&request);
    
                let response = handler(&cfg, &request);
    
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            },
            Err(err) => {
                eprintln!("Problem parsing request: {}", err);
            }
        }
    }
}


fn setup_cfg(args: &[String]) -> Cfg {
    let files_dir = setup_directory(&args);
    
    Cfg {files_dir}
}

fn setup_router() -> Router {
    Router::new(vec![
        Route::new("/", HttpMethod::GET, handlers::handle_200),
        Route::new("/echo/", HttpMethod::GET, handlers::handle_echo),
        Route::new("/user-agent", HttpMethod::GET, handlers::handle_user_agent),
        Route::new("/files/", HttpMethod::GET, handlers::handle_get_file),
        Route::new("/files/", HttpMethod::POST, handlers::handle_post_file),
    ])
}

fn setup_directory(args: &[String]) -> Option<String>{
    let dir_flag_index = args.iter()
        .position(|arg| arg == DIR_FLAG);

    match dir_flag_index {
        Some(dir_flag_index) => {
            let path = args.get(dir_flag_index + 1);
            match path {
                Some(path) => {
                    let path = path.clone();
                    fs::create_dir_all(&path)
                        .expect(&format!("Can't create directory at {}", &path));
                    return Some(path)
                },
                None => {
                    panic!("No `directory` argument provided for --directory")
                }
            };
        },
        None => return None
    }
}
