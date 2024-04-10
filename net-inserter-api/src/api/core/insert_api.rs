pub trait InsertAPI: net_core_api::core::api::API {
    type InsertEntity;
    fn into_insert_request(&self) -> crate::api::insert_request::InsertRequestDTO;
    fn into_insert_entity(request: Vec<u8>) -> Self::InsertEntity; 
}
