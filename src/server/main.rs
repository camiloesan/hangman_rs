mod dal;

use tonic::{transport::Server, Request, Response, Status};

use hangmancfg::user_manager_server::{UserManager, UserManagerServer};
use hangmancfg::{CredentialsRequest, IsSuccessReply, RowsAffectedReply, UserDataRequest};

pub mod hangmancfg {
    tonic::include_proto!("hangmancfg");
}

#[derive(Debug, Default)]
pub struct HangmanService {}

#[tonic::async_trait]
impl UserManager for HangmanService {
    async fn add_user(
        &self,
        _request: Request<UserDataRequest>,
    ) -> Result<Response<RowsAffectedReply>, Status> {
        let request = _request.into_inner();
        let username = request.username;
        let email = request.email;
        let password = request.password;

        let reply = hangmancfg::RowsAffectedReply {
            rows_affected: dal::user::add_user(username, email, password),
        };

        Ok(Response::new(reply))
    }

    async fn is_auth_valid(
        &self,
        _request: Request<CredentialsRequest>,
    ) -> Result<Response<IsSuccessReply>, Status> {
        let request = _request.into_inner();
        let username = request.username;
        let password = request.password;

        let reply = hangmancfg::IsSuccessReply {
            is_success: dal::user::is_auth_valid(username, password),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_service = HangmanService::default();

    Server::builder()
        .add_service(UserManagerServer::new(user_service))
        .serve(addr)
        .await?;
    Ok(())
}