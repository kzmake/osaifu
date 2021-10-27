use anyhow::Result;
use infrastructure::grpc::Service;
use infrastructure::postgres::DbPool;
use infrastructure::postgres::WalletRepository;
use infrastructure::ulid::IdRepository;
use interface::controller::WalletController;
use std::env;
use usecase::interactor::CreateWalletInteractor;
use usecase::interactor::GetWalletInteractor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("WALLET_DATABASE_URL").expect("WALLET_DATABASE_URL must be set");

    let connections = DbPool::new(&database_url);

    let id_repository = IdRepository::default();
    let wallet_repository = WalletRepository::new(connections.clone());
    let create = CreateWalletInteractor::new(id_repository, wallet_repository.clone());
    let get = GetWalletInteractor::new(wallet_repository.clone());
    let controller = WalletController::new(create, get);
    let service = Service::new(controller);

    let addr = "0.0.0.0:50051".parse()?;

    println!("service listening on {}", addr);

    connections.init()?;
    service.serve(addr).await?;

    Ok(())
}
