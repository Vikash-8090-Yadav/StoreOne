mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const SBSTICKET_TRACKED_CONTRACT: [u8; 20] = hex!("cfc29195eb33f4e7de75a0eb0ab6fb120946a3b2");

fn map_sbsticket_events(blk: &eth::Block, events: &mut contract::Events) {
    events.sbsticket_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::SbsticketApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            owner: event.owner,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_approval_for_alls.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::ApprovalForAll::match_and_decode(log) {
                        return Some(contract::SbsticketApprovalForAll {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            approved: event.approved,
                            operator: event.operator,
                            owner: event.owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_batch_metadata_updates.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::BatchMetadataUpdate::match_and_decode(log) {
                        return Some(contract::SbsticketBatchMetadataUpdate {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_from_token_id: event.u_from_token_id.to_string(),
                            u_to_token_id: event.u_to_token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_chai_boughts.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::ChaiBought::match_and_decode(log) {
                        return Some(contract::SbsticketChaiBought {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            buyer: event.buyer,
                            message: event.message,
                            name: event.name,
                            timestamp: event.timestamp.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_donations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::Donation::match_and_decode(log) {
                        return Some(contract::SbsticketDonation {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            from: event.from,
                            message: event.message,
                            name: event.name,
                            timestamp: event.timestamp.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_listing_price_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::ListingPriceUpdated::match_and_decode(log) {
                        return Some(contract::SbsticketListingPriceUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_price: event.new_price.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_market_item_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::MarketItemCreated::match_and_decode(log) {
                        return Some(contract::SbsticketMarketItemCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            owner: event.owner,
                            price: event.price.to_string(),
                            seller: event.seller,
                            sold: event.sold,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_market_sales.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::MarketSale::match_and_decode(log) {
                        return Some(contract::SbsticketMarketSale {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            buyer: event.buyer,
                            price: event.price.to_string(),
                            seller: event.seller,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_metadata_updates.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::MetadataUpdate::match_and_decode(log) {
                        return Some(contract::SbsticketMetadataUpdate {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_token_id: event.u_token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_token_items.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::TokenItem::match_and_decode(log) {
                        return Some(contract::SbsticketTokenItem {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_token_id: event.new_token_id.to_string(),
                            price: event.price.to_string(),
                            token_uri: event.token_uri,
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_token_resolds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::TokenResold::match_and_decode(log) {
                        return Some(contract::SbsticketTokenResold {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            owner: event.owner,
                            price: event.price.to_string(),
                            seller: event.seller,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.sbsticket_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SBSTICKET_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sbsticket_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::SbsticketTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                            token_id: event.token_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

}

fn graph_sbsticket_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.sbsticket_approvals.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_approval", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("approved", Hex(&evt.approved).to_string())
            .set("owner", Hex(&evt.owner).to_string())
            .set("token_id", BigDecimal::from_str(&evt.token_id).unwrap());
    });
    events.sbsticket_approval_for_alls.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_approval_for_all", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("approved", evt.approved)
            .set("operator", Hex(&evt.operator).to_string())
            .set("owner", Hex(&evt.owner).to_string());
    });
    events.sbsticket_batch_metadata_updates.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_batch_metadata_update", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_from_token_id", BigDecimal::from_str(&evt.u_from_token_id).unwrap())
            .set("u_to_token_id", BigDecimal::from_str(&evt.u_to_token_id).unwrap());
    });
    events.sbsticket_chai_boughts.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_chai_bought", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("buyer", Hex(&evt.buyer).to_string())
            .set("message", &evt.message)
            .set("name", &evt.name)
            .set("timestamp", BigDecimal::from_str(&evt.timestamp).unwrap());
    });
    events.sbsticket_donations.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_donation", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("from", Hex(&evt.from).to_string())
            .set("message", &evt.message)
            .set("name", &evt.name)
            .set("timestamp", BigDecimal::from_str(&evt.timestamp).unwrap());
    });
    events.sbsticket_listing_price_updateds.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_listing_price_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_price", BigDecimal::from_str(&evt.new_price).unwrap());
    });
    events.sbsticket_market_item_createds.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_market_item_created", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("price", BigDecimal::from_str(&evt.price).unwrap())
            .set("seller", Hex(&evt.seller).to_string())
            .set("sold", evt.sold)
            .set("token_id", BigDecimal::from_str(&evt.token_id).unwrap());
    });
    events.sbsticket_market_sales.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_market_sale", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("buyer", Hex(&evt.buyer).to_string())
            .set("price", BigDecimal::from_str(&evt.price).unwrap())
            .set("seller", Hex(&evt.seller).to_string())
            .set("token_id", BigDecimal::from_str(&evt.token_id).unwrap());
    });
    events.sbsticket_metadata_updates.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_metadata_update", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("u_token_id", BigDecimal::from_str(&evt.u_token_id).unwrap());
    });
    events.sbsticket_token_items.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_token_item", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_token_id", BigDecimal::from_str(&evt.new_token_id).unwrap())
            .set("price", BigDecimal::from_str(&evt.price).unwrap())
            .set("token_uri", &evt.token_uri);
    });
    events.sbsticket_token_resolds.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_token_resold", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("owner", Hex(&evt.owner).to_string())
            .set("price", BigDecimal::from_str(&evt.price).unwrap())
            .set("seller", Hex(&evt.seller).to_string())
            .set("token_id", BigDecimal::from_str(&evt.token_id).unwrap());
    });
    events.sbsticket_transfers.iter().for_each(|evt| {
        tables
            .create_row("sbsticket_transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("token_id", BigDecimal::from_str(&evt.token_id).unwrap());
    });
    
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_sbsticket_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_sbsticket_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}

