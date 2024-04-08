// use tonic::{transport::Server, Request, Response, Status};

// use hangmancfg::user_manager_server::{UserManager, UserManagerServer};
// use hangmancfg::{UserDataRequest, SuccessReply};

// pub mod hangmancfg {
//     tonic::include_proto!("hangmancfg");
// }

// #[derive(Debug, Default)]
// pub struct HangmanService {}

// #[tonic::async_trait]
// impl UserManager for HangmanService {
//     async fn add_user(
//         &self,
//         _request: Request<UserDataRequest>,
//     ) -> Result<Response<SuccessReply>, Status> {

//         let reply = hangmancfg::SuccessReply {
//             is_success: true,
//         };

//         Ok(Response::new(reply))
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:50051".parse()?;
//     let greeter = HangmanService::default();

//     Server::builder()
//         .add_service(UserManagerServer::new(greeter))
//         .serve(addr)
//         .await?;
//     Ok(())
// }

fn main() {
    println!("Hello, world!");
}