# Temples

Temples is a framework for building onchain applications. It is written in the Rust language.It consists of two parts, one for Storage and one for World.

## Schema
A fully chained ORM. 

To make it easier to build on-chain applications, we ditched the original storage model implemented in Rust and replaced it with a runtime-defined storage system.
Schema enables you to do new things on-chain that you didn't know you could do before. As an example of its power, Schema allows you to add new tables at runtime without having to redeploy contracts or perform upgrades.
Schema emits events for every change in state. This allows the client application to stay updated on each block and enables the indexer to rebuild the state on the chain. Never write a contract event again and remove large chunks of client-side code and bombard RPCs with state function calls at load time.

```rust
#[derive(StorageValue, Encode, Decode, Default, Debug)]
pub struct Counter {
    pub value: u128,
}

Counter.get();
Counter.set(Counter { value: 10 });
Counter.muate(|v| { *v = 100 });


#[derive(StorageMap, Encode, Decode, Default, Debug)]
pub struct CMap {
    #[key]
    pub key: u128,
    pub value: u128,
}

CMap.get(0);
CMap.set(0, 10);
CMap.muate(0, |v| { *v = 100 });
```
## NexCore

The goal of NexCore is to make it easy for developers to build and extend on-chain applications. To enable this new way of building on-chain, we created NexCore: a smart contract framework built on top of the Schema.

