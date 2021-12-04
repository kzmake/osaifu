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

    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        println!("{:?}", request); // TODO: logger を実装して println! を削除する

        self.controller.list(request)
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
        let reflection = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(interface::osaifu_wallet_v1::FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();

        Server::builder()
            .add_service(WalletServiceServer::new(self))
            .add_service(reflection)
            .serve(addr)
            .await?;

        Ok(())
    }
}
