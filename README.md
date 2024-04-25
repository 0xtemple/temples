# Temples

Temples is a framework for building onchain applications. It is written in the Rust language.It consists of two parts, one for Schema and one for NexCore.

## Schema
A fully chained ORM. 

To make it easier to build on-chain applications, we ditched the original storage model implemented in Rust and replaced it with a runtime-defined storage system.
Schema enables you to do new things on-chain that you didn't know you could do before. As an example of its power, Schema allows you to add new tables at runtime without having to redeploy contracts or perform upgrades.
Schema emits events for every change in state. This allows the client application to stay updated on each block and enables the indexer to rebuild the state on the chain. Never write a contract event again and remove large chunks of client-side code and bombard RPCs with state function calls at load time.

## NexCore

The goal of NexCore is to make it easy for developers to build and extend on-chain applications. To enable this new way of building on-chain, we created NexCore: a smart contract framework built on top of the Schema.
<img width="1289" alt="image" src="https://github.com/0xtemple/temples/assets/111047493/075f8e21-4cf5-45a8-b2e4-7200c88a058f">



