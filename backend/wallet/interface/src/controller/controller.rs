use crate::osaifu_wallet_v1::Wallet as PBWallet;
use crate::osaifu_wallet_v1::{CreateRequest, CreateResponse};
use crate::osaifu_wallet_v1::{DeleteRequest, DeleteResponse};
use crate::osaifu_wallet_v1::{GetRequest, GetResponse};
use anyhow::Result;
use derive_new::new;
use tonic::{Request, Response, Status};
use usecase::port::{
    CreateWalletInputData, CreateWalletOutputData, DeleteWalletInputData, DeleteWalletOutputData,
    GetWalletInputData, GetWalletOutputData, Port,
};

pub trait Controller {
    fn create(&self, request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status>;
    fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status>;
    fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status>;
}

#[derive(new)]
pub struct WalletController<Create, Get, Delete>
where
    Create: Port<CreateWalletInputData, CreateWalletOutputData>,
    Get: Port<GetWalletInputData, GetWalletOutputData>,
    Delete: Port<DeleteWalletInputData, DeleteWalletOutputData>,
{
    create_wallet: Create,
    get_wallet: Get,
    delete_wallet: Delete,
}

impl<Create, Get, Delete> Controller for WalletController<Create, Get, Delete>
where
    Create: Port<CreateWalletInputData, CreateWalletOutputData>,
    Get: Port<GetWalletInputData, GetWalletOutputData>,
    Delete: Port<DeleteWalletInputData, DeleteWalletOutputData>,
{
    fn create(&self, request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        let input = CreateWalletInputData::new(request.get_ref().owner.to_string());

        match self.create_wallet.handle(input) {
            Ok(output) => Ok(Response::new(CreateResponse {
                wallet: Some(PBWallet {
                    id: output.wallet.id().to_string(),
                    owner: "alice".to_string(),
                    balance: output.wallet.balance().to_string(),
                }),
            })),
            Err(_) => Err(Status::internal("error")),
        }
    }

    fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let input = GetWalletInputData::new(request.get_ref().id.to_string());

        match self.get_wallet.handle(input) {
            Ok(output) => Ok(Response::new(GetResponse {
                wallet: Some(PBWallet {
                    id: output.wallet.id().to_string(),
                    owner: "alice".to_string(),
                    balance: output.wallet.balance().to_string(),
                }),
            })),
            Err(_) => Err(Status::internal("error")),
        }
    }

    fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        let input = DeleteWalletInputData::new(request.get_ref().id.to_string());

        match self.delete_wallet.handle(input) {
            Ok(_) => Ok(Response::new(DeleteResponse {})),
            Err(_) => Err(Status::internal("error")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use domain::entity::*;
    use domain::vo::*;
    use usecase::port::*;

    fn new_wallet() -> Wallet {
        WalletBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Wallet>>().unwrap())
            .balance("2000".parse::<Money<JPY>>().unwrap())
            .build()
            .unwrap()
    }

    #[test]
    fn test_create_wallet_handle_ok() {
        let entity = new_wallet();

        let mut create = MockPort::<CreateWalletInputData, CreateWalletOutputData>::new();
        let mut get = MockPort::<GetWalletInputData, GetWalletOutputData>::new();
        let mut delete = MockPort::<DeleteWalletInputData, DeleteWalletOutputData>::new();
        create
            .expect_handle()
            .returning(|_| Ok(CreateWalletOutputData::new(new_wallet())));
        get.expect_handle()
            .returning(|_| Ok(GetWalletOutputData::new(new_wallet())));
        delete
            .expect_handle()
            .returning(|_| Ok(DeleteWalletOutputData::new()));
        let sut = WalletController::new(create, get, delete);

        assert_eq!(
            sut.create(Request::new(CreateRequest {
                owner: "alice".to_string(),
            }))
            .unwrap()
            .get_ref(),
            Response::new(CreateResponse {
                wallet: Some(PBWallet {
                    id: entity.id().to_string(),
                    owner: "alice".to_string(),
                    balance: entity.balance().to_string(),
                }),
            })
            .get_ref(),
        );
    }

    #[test]
    fn test_create_wallet_handle_err() {
        let _entity = new_wallet();

        let mut create = MockPort::<CreateWalletInputData, CreateWalletOutputData>::new();
        let mut get = MockPort::<GetWalletInputData, GetWalletOutputData>::new();
        let mut delete = MockPort::<DeleteWalletInputData, DeleteWalletOutputData>::new();
        create.expect_handle().returning(|_| bail!("error"));
        get.expect_handle().returning(|_| bail!("error"));
        delete.expect_handle().returning(|_| bail!("error"));
        let sut = WalletController::new(create, get, delete);

        assert!(sut
            .create(Request::new(CreateRequest {
                owner: "alice".to_string(),
            }))
            .is_err());
    }

    #[test]
    fn test_get_wallet_handle_ok() {
        let entity = new_wallet();

        let mut create = MockPort::<CreateWalletInputData, CreateWalletOutputData>::new();
        let mut get = MockPort::<GetWalletInputData, GetWalletOutputData>::new();
        let mut delete = MockPort::<DeleteWalletInputData, DeleteWalletOutputData>::new();
        create
            .expect_handle()
            .returning(|_| Ok(CreateWalletOutputData::new(new_wallet())));
        get.expect_handle()
            .returning(|_| Ok(GetWalletOutputData::new(new_wallet())));
        delete
            .expect_handle()
            .returning(|_| Ok(DeleteWalletOutputData::new()));
        let sut = WalletController::new(create, get, delete);

        assert_eq!(
            sut.get(Request::new(GetRequest {
                id: entity.id().to_string(),
            }))
            .unwrap()
            .get_ref(),
            Response::new(GetResponse {
                wallet: Some(PBWallet {
                    id: entity.id().to_string(),
                    owner: "alice".to_string(),
                    balance: entity.balance().to_string(),
                }),
            })
            .get_ref(),
        );
    }

    #[test]
    fn test_get_wallet_handle_err() {
        let entity = new_wallet();

        let mut create = MockPort::<CreateWalletInputData, CreateWalletOutputData>::new();
        let mut get = MockPort::<GetWalletInputData, GetWalletOutputData>::new();
        let mut delete = MockPort::<DeleteWalletInputData, DeleteWalletOutputData>::new();
        create.expect_handle().returning(|_| bail!("error"));
        get.expect_handle().returning(|_| bail!("error"));
        delete.expect_handle().returning(|_| bail!("error"));
        let sut = WalletController::new(create, get, delete);

        assert!(sut
            .get(Request::new(GetRequest {
                id: entity.id().to_string()
            }))
            .is_err());
    }
}
