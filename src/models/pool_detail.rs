/*
 * Midgard Public API
 *
 * The Midgard Public API queries THORChain and any chains linked via the Bifröst and prepares information about the network to be readily available for public users. The API parses transaction event data from THORChain and stores them in a time-series database to make time-dependent queries easy. Midgard does not hold critical information. To interact with BEPSwap and Asgardex, users should query THORChain directly.
 *
 * The version of the OpenAPI document: 1.0.0-oas3
 * Contact: devs@thorchain.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolDetail {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Price of Asset (in RUNE).
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Total Asset staked
    #[serde(rename = "assetStakedTotal", skip_serializing_if = "Option::is_none")]
    pub asset_staked_total: Option<String>,
    /// Total RUNE staked
    #[serde(rename = "runeStakedTotal", skip_serializing_if = "Option::is_none")]
    pub rune_staked_total: Option<String>,
    /// Rune value staked Total
    #[serde(rename = "poolStakedTotal", skip_serializing_if = "Option::is_none")]
    pub pool_staked_total: Option<String>,
    /// Total current Asset balance
    #[serde(rename = "assetDepth", skip_serializing_if = "Option::is_none")]
    pub asset_depth: Option<String>,
    /// Total current Rune balance
    #[serde(rename = "runeDepth", skip_serializing_if = "Option::is_none")]
    pub rune_depth: Option<String>,
    /// Total depth of both sides (in RUNE)
    #[serde(rename = "poolDepth", skip_serializing_if = "Option::is_none")]
    pub pool_depth: Option<String>,
    /// Total pool units outstanding
    #[serde(rename = "poolUnits", skip_serializing_if = "Option::is_none")]
    pub pool_units: Option<String>,
    /// Total Asset sell volume (ASSET>RUNE) (in RUNE).
    #[serde(rename = "sellVolume", skip_serializing_if = "Option::is_none")]
    pub sell_volume: Option<String>,
    /// Total Asset buy volume (RUNE->ASSET) (in Asset)
    #[serde(rename = "buyVolume", skip_serializing_if = "Option::is_none")]
    pub buy_volume: Option<String>,
    /// Two-way volume of all-time (in RUNE)
    #[serde(rename = "poolVolume", skip_serializing_if = "Option::is_none")]
    pub pool_volume: Option<String>,
    /// Two-way volume in 24hrs (in RUNE)
    #[serde(rename = "poolVolume24hr", skip_serializing_if = "Option::is_none")]
    pub pool_volume24hr: Option<String>,
    /// Average Asset sell transaction size (ASSET>RUNE) (in RUNE)
    #[serde(rename = "sellTxAverage", skip_serializing_if = "Option::is_none")]
    pub sell_tx_average: Option<String>,
    /// Average Asset buy transaction size for (RUNE->ASSET) (in ASSET)
    #[serde(rename = "buyTxAverage", skip_serializing_if = "Option::is_none")]
    pub buy_tx_average: Option<String>,
    /// Average pool transaction
    #[serde(rename = "poolTxAverage", skip_serializing_if = "Option::is_none")]
    pub pool_tx_average: Option<String>,
    /// Average trade slip for ASSET->RUNE in %
    #[serde(rename = "sellSlipAverage", skip_serializing_if = "Option::is_none")]
    pub sell_slip_average: Option<String>,
    /// Average trade slip for RUNE->ASSET in %
    #[serde(rename = "buySlipAverage", skip_serializing_if = "Option::is_none")]
    pub buy_slip_average: Option<String>,
    /// Average pool slip
    #[serde(rename = "poolSlipAverage", skip_serializing_if = "Option::is_none")]
    pub pool_slip_average: Option<String>,
    /// Average buy Asset fee size for ASSET->RUNE (in RUNE)
    #[serde(rename = "sellFeeAverage", skip_serializing_if = "Option::is_none")]
    pub sell_fee_average: Option<String>,
    /// Average sell Asset fee size for RUNE->ASSET (in ASSET)
    #[serde(rename = "buyFeeAverage", skip_serializing_if = "Option::is_none")]
    pub buy_fee_average: Option<String>,
    /// Average pool fee
    #[serde(rename = "poolFeeAverage", skip_serializing_if = "Option::is_none")]
    pub pool_fee_average: Option<String>,
    /// Total fees (in RUNE)
    #[serde(rename = "sellFeesTotal", skip_serializing_if = "Option::is_none")]
    pub sell_fees_total: Option<String>,
    /// Total fees (in Asset)
    #[serde(rename = "buyFeesTotal", skip_serializing_if = "Option::is_none")]
    pub buy_fees_total: Option<String>,
    /// Total fees
    #[serde(rename = "poolFeesTotal", skip_serializing_if = "Option::is_none")]
    pub pool_fees_total: Option<String>,
    /// Number of ASSET->RUNE transactions
    #[serde(rename = "sellAssetCount", skip_serializing_if = "Option::is_none")]
    pub sell_asset_count: Option<String>,
    /// Number of RUNE->ASSET transactions
    #[serde(rename = "buyAssetCount", skip_serializing_if = "Option::is_none")]
    pub buy_asset_count: Option<String>,
    /// Number of swapping transactions in the pool (buys and sells)
    #[serde(rename = "swappingTxCount", skip_serializing_if = "Option::is_none")]
    pub swapping_tx_count: Option<String>,
    /// Number of unique swappers interacting with pool
    #[serde(rename = "swappersCount", skip_serializing_if = "Option::is_none")]
    pub swappers_count: Option<String>,
    /// Number of stake transactions
    #[serde(rename = "stakeTxCount", skip_serializing_if = "Option::is_none")]
    pub stake_tx_count: Option<String>,
    /// Number of withdraw transactions
    #[serde(rename = "withdrawTxCount", skip_serializing_if = "Option::is_none")]
    pub withdraw_tx_count: Option<String>,
    /// Number of stake & withdraw transactions
    #[serde(rename = "stakingTxCount", skip_serializing_if = "Option::is_none")]
    pub staking_tx_count: Option<String>,
    /// Number of unique stakers
    #[serde(rename = "stakersCount", skip_serializing_if = "Option::is_none")]
    pub stakers_count: Option<String>,
    /// Asset return on investment
    #[serde(rename = "assetROI", skip_serializing_if = "Option::is_none")]
    pub asset_roi: Option<String>,
    /// RUNE return on investment
    #[serde(rename = "runeROI", skip_serializing_if = "Option::is_none")]
    pub rune_roi: Option<String>,
    /// Pool ROI (average of RUNE and Asset ROI)
    #[serde(rename = "poolROI", skip_serializing_if = "Option::is_none")]
    pub pool_roi: Option<String>,
    /// Pool ROI over 12 months
    #[serde(rename = "poolROI12", skip_serializing_if = "Option::is_none")]
    pub pool_roi12: Option<String>,
}

impl PoolDetail {
    pub fn new() -> PoolDetail {
        PoolDetail {
            asset: None,
            status: None,
            price: None,
            asset_staked_total: None,
            rune_staked_total: None,
            pool_staked_total: None,
            asset_depth: None,
            rune_depth: None,
            pool_depth: None,
            pool_units: None,
            sell_volume: None,
            buy_volume: None,
            pool_volume: None,
            pool_volume24hr: None,
            sell_tx_average: None,
            buy_tx_average: None,
            pool_tx_average: None,
            sell_slip_average: None,
            buy_slip_average: None,
            pool_slip_average: None,
            sell_fee_average: None,
            buy_fee_average: None,
            pool_fee_average: None,
            sell_fees_total: None,
            buy_fees_total: None,
            pool_fees_total: None,
            sell_asset_count: None,
            buy_asset_count: None,
            swapping_tx_count: None,
            swappers_count: None,
            stake_tx_count: None,
            withdraw_tx_count: None,
            staking_tx_count: None,
            stakers_count: None,
            asset_roi: None,
            rune_roi: None,
            pool_roi: None,
            pool_roi12: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "bootstrapped")]
    Bootstrapped,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

