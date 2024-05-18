use gsdk::ext::sp_runtime::AccountId32;
use gsdk::metadata::runtime_types;
use gsdk::metadata::runtime_types::{pallet_balances, pallet_gear};
use gsdk::{Api, Event};
use gsdk::{Error, Result};
use hex_literal::hex;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting indexer");

    // let api = Api::new(Some("ws://localhost:9944")).await?;
    // let alice = api.clone().signer("//Alice", None)?;
    // let my_address = AccountId32::new(hex!["aea48c27a7f703a7f8acedf15b43e8fcbad0b7846e5fe32a0b2b75cb81d75306"]);
    // let tx = alice.calls.transfer_keep_alive(my_address, 1000000000000).await?;
    // let events = tx.fetch_events().await?;
    // for ev in events.iter() {
    //     let ev = ev?;
    //
    //     println!("{:?}", ev.event_metadata());
    // }

    // for event in api.events_of(&tx).await? {
    //     if let Event::Balances(pallet_balances::pallet::Event::Transfer { .. }) = event
    //     {
    //         println!("{:?}", event);
    //         break;
    //     }
    // }

    let api = Api::new(Some("ws://localhost:9944")).await?;
    let mut events = api.finalized_events().await?;

    while let Some(Ok(evs)) = events.next().await {
        println!("{:?}", api.gear_block_number(None).await?);
        for ev in &evs {
            if let Event::Gear(pallet_gear::pallet::Event::UserMessageSent { .. }) = ev {
                println!("{:?}", ev);
            }
        }
    }

    Ok(())
}
