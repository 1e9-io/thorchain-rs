# Rust API client for openapi

The Midgard Public API queries THORChain and any chains linked via the Bifröst and prepares information about the network to be readily available for public users. The API parses transaction event data from THORChain and stores them in a time-series database to make time-dependent queries easy. Midgard does not hold critical information. To interact with BEPSwap and Asgardex, users should query THORChain directly.

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0-oas3
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *http://127.0.0.1:8080*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**get_asset_info**](docs/DefaultApi.md#get_asset_info) | **get** /v1/assets | Get Asset Information
*DefaultApi* | [**get_health**](docs/DefaultApi.md#get_health) | **get** /v1/health | Get Health
*DefaultApi* | [**get_network_data**](docs/DefaultApi.md#get_network_data) | **get** /v1/network | Get Network Data
*DefaultApi* | [**get_nodes**](docs/DefaultApi.md#get_nodes) | **get** /v1/nodes | Get Node public keys
*DefaultApi* | [**get_pools**](docs/DefaultApi.md#get_pools) | **get** /v1/pools | Get Asset Pools
*DefaultApi* | [**get_pools_details**](docs/DefaultApi.md#get_pools_details) | **get** /v1/pools/detail | Get Pools Details
*DefaultApi* | [**get_stakers_address_and_asset_data**](docs/DefaultApi.md#get_stakers_address_and_asset_data) | **get** /v1/stakers/{address}/pools | Get Staker Pool Data
*DefaultApi* | [**get_stakers_address_data**](docs/DefaultApi.md#get_stakers_address_data) | **get** /v1/stakers/{address} | Get Staker Data
*DefaultApi* | [**get_stakers_data**](docs/DefaultApi.md#get_stakers_data) | **get** /v1/stakers | Get Stakers
*DefaultApi* | [**get_stats**](docs/DefaultApi.md#get_stats) | **get** /v1/stats | Get Global Stats
*DefaultApi* | [**get_thorchain_proxied_constants**](docs/DefaultApi.md#get_thorchain_proxied_constants) | **get** /v1/thorchain/constants | Get the Proxied THORChain Constants
*DefaultApi* | [**get_thorchain_proxied_endpoints**](docs/DefaultApi.md#get_thorchain_proxied_endpoints) | **get** /v1/thorchain/pool_addresses | Get the Proxied Pool Addresses
*DefaultApi* | [**get_thorchain_proxied_lastblock**](docs/DefaultApi.md#get_thorchain_proxied_lastblock) | **get** /v1/thorchain/lastblock | Get the Proxied THORChain Lastblock
*DefaultApi* | [**get_total_vol_changes**](docs/DefaultApi.md#get_total_vol_changes) | **get** /v1/history/total_volume | Get Total Volume Changes
*DefaultApi* | [**get_tx_details**](docs/DefaultApi.md#get_tx_details) | **get** /v1/txs | Get details of a tx by address, asset or tx-id
*DocumentationApi* | [**get_docs**](docs/DocumentationApi.md#get_docs) | **get** /v1/doc | Get Documents
*SpecificationApi* | [**get_swagger**](docs/SpecificationApi.md#get_swagger) | **get** /v1/swagger.json | Get Swagger


## Documentation For Models

 - [AssetDetail](docs/AssetDetail.md)
 - [BlockRewards](docs/BlockRewards.md)
 - [BondMetrics](docs/BondMetrics.md)
 - [Coin](docs/Coin.md)
 - [Error](docs/Error.md)
 - [Event](docs/Event.md)
 - [Gas](docs/Gas.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InlineResponse2001](docs/InlineResponse2001.md)
 - [NetworkInfo](docs/NetworkInfo.md)
 - [NodeKey](docs/NodeKey.md)
 - [Option](docs/Option.md)
 - [PoolDetail](docs/PoolDetail.md)
 - [StakersAddressData](docs/StakersAddressData.md)
 - [StakersAssetData](docs/StakersAssetData.md)
 - [StatsData](docs/StatsData.md)
 - [ThorchainBooleanConstants](docs/ThorchainBooleanConstants.md)
 - [ThorchainConstants](docs/ThorchainConstants.md)
 - [ThorchainEndpoint](docs/ThorchainEndpoint.md)
 - [ThorchainEndpoints](docs/ThorchainEndpoints.md)
 - [ThorchainInt64Constants](docs/ThorchainInt64Constants.md)
 - [ThorchainLastblock](docs/ThorchainLastblock.md)
 - [ThorchainStringConstants](docs/ThorchainStringConstants.md)
 - [TotalVolChanges](docs/TotalVolChanges.md)
 - [Tx](docs/Tx.md)
 - [TxDetails](docs/TxDetails.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

