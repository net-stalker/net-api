use std::error::Error;

use crate::api::insert_request::InsertRequestDTO;

pub trait InsertAPI: net_core_api::core::api::API + Sized {
    fn into_insert_request(self) -> crate::api::insert_request::InsertRequestDTO;
    fn into_insert_entity(request: InsertRequestDTO) -> Result<Self, Box<dyn Error + Send + Sync>>; 
}
