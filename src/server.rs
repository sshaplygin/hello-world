use tonic::{transport::Server, Request, Response, Status};

// use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloResponse, HelloRequest};
use hello_world::greeter_server::{Greeter, GreeterServer};

pub mod hello_world {
    tonic::include_proto!("hello_world");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self, 
        request: Request<HelloRequest>, 
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Get a request: {:?}", request);

        let reply = hello_world::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr = "[::1]:8080".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
