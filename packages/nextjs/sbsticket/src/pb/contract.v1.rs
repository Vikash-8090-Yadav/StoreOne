// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub sbsticket_approvals: ::prost::alloc::vec::Vec<SbsticketApproval>,
    #[prost(message, repeated, tag="2")]
    pub sbsticket_approval_for_alls: ::prost::alloc::vec::Vec<SbsticketApprovalForAll>,
    #[prost(message, repeated, tag="3")]
    pub sbsticket_batch_metadata_updates: ::prost::alloc::vec::Vec<SbsticketBatchMetadataUpdate>,
    #[prost(message, repeated, tag="4")]
    pub sbsticket_chai_boughts: ::prost::alloc::vec::Vec<SbsticketChaiBought>,
    #[prost(message, repeated, tag="5")]
    pub sbsticket_donations: ::prost::alloc::vec::Vec<SbsticketDonation>,
    #[prost(message, repeated, tag="6")]
    pub sbsticket_listing_price_updateds: ::prost::alloc::vec::Vec<SbsticketListingPriceUpdated>,
    #[prost(message, repeated, tag="7")]
    pub sbsticket_market_item_createds: ::prost::alloc::vec::Vec<SbsticketMarketItemCreated>,
    #[prost(message, repeated, tag="8")]
    pub sbsticket_market_sales: ::prost::alloc::vec::Vec<SbsticketMarketSale>,
    #[prost(message, repeated, tag="9")]
    pub sbsticket_metadata_updates: ::prost::alloc::vec::Vec<SbsticketMetadataUpdate>,
    #[prost(message, repeated, tag="10")]
    pub sbsticket_token_items: ::prost::alloc::vec::Vec<SbsticketTokenItem>,
    #[prost(message, repeated, tag="11")]
    pub sbsticket_token_resolds: ::prost::alloc::vec::Vec<SbsticketTokenResold>,
    #[prost(message, repeated, tag="12")]
    pub sbsticket_transfers: ::prost::alloc::vec::Vec<SbsticketTransfer>,
    #[prost(message, repeated, tag="13")]
    pub sbsticket_update_data: ::prost::alloc::vec::Vec<SbsticketUpdateData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketApproval {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub approved: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketApprovalForAll {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub operator: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="7")]
    pub approved: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketBatchMetadataUpdate {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub u_from_token_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub u_to_token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketChaiBought {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="8")]
    pub buyer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketDonation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketListingPriceUpdated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub new_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketMarketItemCreated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub seller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub price: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub sold: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketMarketSale {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub buyer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub seller: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketMetadataUpdate {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub u_token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketTokenItem {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub token_uri: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub new_token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketTokenResold {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub seller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketTransfer {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SbsticketUpdateData {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub func: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub txn: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="8")]
    pub buyer: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
