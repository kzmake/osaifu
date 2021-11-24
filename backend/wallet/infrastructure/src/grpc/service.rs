use anyhow::Result;
use derive_new::new;
use interface::controller::Controller;
use interface::osaifu_wallet_v1::wallet_service_server::{WalletService, WalletServiceServer};
use interface::osaifu_wallet_v1::{CreateRequest, CreateResponse};
use interface::osaifu_wallet_v1::{DeleteRequest, DeleteResponse};
use interface::osaifu_wallet_v1::{GetRequest, GetResponse};
use interface::osaifu_wallet_v1::{ListRequest, ListResponse};
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

#[derive(new)]
pub struct Service<C>
where
    C: Controller + std::marker::Sync + std::marker::Send,
{
    controller: C,
}

#[tonic::async_trait]
impl<C> WalletService for Service<C>
where
    C: Controller + std::marker::Sync + std::marker::Send + 'static,
{
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        println!("{:?}", request); // TODO: logger を実装して println! を削除する

        self.controller.create(request)
    }

    async fn list(&self, _: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        println!("{:?}", request); // TODO: logger を実装して println! を削除する

        self.controller.get(request)
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        println!("{:?}", request); // TODO: logger を実装して println! を削除する

        self.controller.delete(request)
    }
}

impl<C> Service<C>
where
    C: Controller + std::marker::Sync + std::marker::Send + 'static,
{
    pub async fn serve(self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        Server::builder()
            .add_service(WalletServiceServer::new(self))
            .serve(addr)
            .await?;

        Ok(())
    }
}
