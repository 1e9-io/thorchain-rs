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
pub struct StatsData {
    /// Daily active users (unique addresses interacting)
    #[serde(rename = "dailyActiveUsers", skip_serializing_if = "Option::is_none")]
    pub daily_active_users: Option<String>,
    /// Monthly active users
    #[serde(rename = "monthlyActiveUsers", skip_serializing_if = "Option::is_none")]
    pub monthly_active_users: Option<String>,
    /// Total unique swappers & stakers
    #[serde(rename = "totalUsers", skip_serializing_if = "Option::is_none")]
    pub total_users: Option<String>,
    /// Daily transactions
    #[serde(rename = "dailyTx", skip_serializing_if = "Option::is_none")]
    pub daily_tx: Option<String>,
    /// Monthly transactions
    #[serde(rename = "monthlyTx", skip_serializing_if = "Option::is_none")]
    pub monthly_tx: Option<String>,
    /// Total transactions
    #[serde(rename = "totalTx", skip_serializing_if = "Option::is_none")]
    pub total_tx: Option<String>,
    /// Total (in RUNE Value) of all assets swapped in 24hrs
    #[serde(rename = "totalVolume24hr", skip_serializing_if = "Option::is_none")]
    pub total_volume24hr: Option<String>,
    /// Total (in RUNE Value) of all assets swapped since start.
    #[serde(rename = "totalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<String>,
    /// Total staked (in RUNE Value).
    #[serde(rename = "totalStaked", skip_serializing_if = "Option::is_none")]
    pub total_staked: Option<String>,
    /// Total RUNE balances
    #[serde(rename = "totalDepth", skip_serializing_if = "Option::is_none")]
    pub total_depth: Option<String>,
    /// Total earned (in RUNE Value).
    #[serde(rename = "totalEarned", skip_serializing_if = "Option::is_none")]
    pub total_earned: Option<String>,
    /// Number of active pools
    #[serde(rename = "poolCount", skip_serializing_if = "Option::is_none")]
    pub pool_count: Option<String>,
    /// Total buying transactions
    #[serde(rename = "totalAssetBuys", skip_serializing_if = "Option::is_none")]
    pub total_asset_buys: Option<String>,
    /// Total selling transactions
    #[serde(rename = "totalAssetSells", skip_serializing_if = "Option::is_none")]
    pub total_asset_sells: Option<String>,
    /// Total staking transactions
    #[serde(rename = "totalStakeTx", skip_serializing_if = "Option::is_none")]
    pub total_stake_tx: Option<String>,
    /// Total withdrawing transactions
    #[serde(rename = "totalWithdrawTx", skip_serializing_if = "Option::is_none")]
    pub total_withdraw_tx: Option<String>,
}

impl StatsData {
    pub fn new() -> StatsData {
        StatsData {
            daily_active_users: None,
            monthly_active_users: None,
            total_users: None,
            daily_tx: None,
            monthly_tx: None,
            total_tx: None,
            total_volume24hr: None,
            total_volume: None,
            total_staked: None,
            total_depth: None,
            total_earned: None,
            pool_count: None,
            total_asset_buys: None,
            total_asset_sells: None,
            total_stake_tx: None,
            total_withdraw_tx: None,
        }
    }
}


