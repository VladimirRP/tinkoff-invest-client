use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Currency {
    RUB,
    USD,
    EUR,
    GBP,
    HKD,
    CHF,
    JPY,
    CNY,
    TRY,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Operation {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    PartiallyFill,
    Fill,
    Cancelled,
    Replaced,
    PendingCancel,
    Rejected,
    PendingReplace,
    PendingNew,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum InstrumentType {
    Stock,
    Currency,
    Bond,
    Etf,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TradeStatus {
    NormalTrading,
    NotAvailableForTrading,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum BrokerAccountType {
    Tinkoff,
    TinkoffIis,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyPayload {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrdersPayload {
    pub order_id: String,
    pub figi: String,
    pub operation: Operation,
    pub status: OrderStatus,
    pub requested_lots: i64,
    pub executed_lots: i64,
    pub r#type: OrderType,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoneyAmount {
    pub currency: Currency,
    pub value: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderPayload {
    pub order_id: String,
    pub operation: Operation,
    pub status: OrderStatus,
    pub reject_reason: Option<String>,
    pub message: Option<String>,
    pub requested_lots: i32,
    pub executed_lots: i32,
    pub commission: Option<MoneyAmount>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrderPayload {
    pub order_id: String,
    pub operation: Operation,
    pub status: OrderStatus,
    pub reject_reason: Option<String>,
    pub message: Option<String>,
    pub requested_lots: i32,
    pub executed_lots: i32,
    pub commission: Option<MoneyAmount>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub figi: String,
    pub ticker: Option<String>,
    pub isin: Option<String>,
    pub instrument_type: InstrumentType,
    pub balance: f64,
    pub blocked: Option<f64>,
    pub lots: i32,
    pub expected_yield: Option<MoneyAmount>,
    pub average_position_price: Option<MoneyAmount>,
    pub average_position_price_no_nkd: Option<MoneyAmount>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioPayload {
    #[serde(default)]
    pub positions: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyPosition {
    pub currency: Currency,
    pub balance: f64,
    pub blocked: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioCurrenciesPayload {
    #[serde(default)]
    pub currencies: Vec<CurrencyPosition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookOrder {
    pub price: f64,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookPayload {
    pub figi: String,
    pub depth: i32,
    #[serde(default)]
    pub bids: Vec<OrderBookOrder>,
    #[serde(default)]
    pub asks: Vec<OrderBookOrder>,
    pub trade_status: TradeStatus,
    pub min_price_increment: f64,
    pub face_value: Option<f64>,
    pub last_price: Option<f64>,
    pub close_price: Option<f64>,
    pub limit_up: Option<f64>,
    pub limit_down: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum OperationStatus {
    Done,
    Decline,
    Progress,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperationTrade {
    pub trade_id: String,
    pub date: DateTime<Local>,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum OperationTypeWithCommission {
    Buy,
    BuyCard,
    Sell,
    BrokerCommission,
    ExchangeCommission,
    ServiceCommission,
    MarginCommission,
    OtherCommission,
    PayIn,
    PayOut,
    Tax,
    TaxLucre,
    TaxDividend,
    TaxCoupon,
    TaxBack,
    Repayment,
    PartRepayment,
    Coupon,
    Dividend,
    SecurityIn,
    SecurityOut,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperationItem {
    pub id: String,
    pub status: OperationStatus,
    #[serde(default)]
    pub trades: Vec<OperationTrade>,
    pub commission: Option<MoneyAmount>,
    pub currency: Currency,
    pub payment: f64,
    pub price: Option<f64>,
    pub quantity: Option<i32>,
    pub figi: Option<String>,
    pub instrument_type: Option<InstrumentType>,
    pub is_margin_call: bool,
    pub date: DateTime<Local>,
    pub operation_type: Option<OperationTypeWithCommission>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationsPayload {
    #[serde(default)]
    pub operations: Vec<OperationItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAccount {
    pub broker_account_type: BrokerAccountType,
    pub broker_account_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsPayload {
    #[serde(default)]
    pub accounts: Vec<UserAccount>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Interval {
    #[serde(rename = "1min")]
    _1min,
    #[serde(rename = "2min")]
    _2min,
    #[serde(rename = "3min")]
    _3min,
    #[serde(rename = "5min")]
    _5min,
    #[serde(rename = "10min")]
    _10min,
    #[serde(rename = "15min")]
    _15min,
    #[serde(rename = "30min")]
    _30min,
    Hour,
    Day,
    Week,
    Month,
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::_1min => write!(f, "1min"),
            Interval::_2min => write!(f, "2min"),
            Interval::_3min => write!(f, "3min"),
            Interval::_5min => write!(f, "5min"),
            Interval::_10min => write!(f, "10min"),
            Interval::_15min => write!(f, "15min"),
            Interval::_30min => write!(f, "30min"),
            Interval::Hour => write!(f, "Hour"),
            Interval::Day => write!(f, "Day"),
            Interval::Week => write!(f, "Week"),
            Interval::Month => write!(f, "Month"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candle {
    pub figi: String,
    pub interval: Interval,
    pub o: f64,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub v: i32,
    pub time: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CandlesPayload {
    pub figi: String,
    pub interval: Interval,
    #[serde(default)]
    pub candles: Vec<Candle>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchMarketInstrumentPayload {
    pub figi: String,
    pub ticker: String,
    pub isin: Option<String>,
    pub min_price_increment: Option<f64>,
    pub lot: i32,
    pub currency: Option<Currency>,
    pub name: String,
    pub r#type: InstrumentType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketInstrument {
    pub figi: String,
    pub ticker: String,
    pub isin: Option<String>,
    pub min_price_increment: Option<f64>,
    pub lot: i32,
    pub currency: Option<Currency>,
    pub name: String,
    pub r#type: InstrumentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketInstrumentListPayload {
    pub total: i32,
    #[serde(default)]
    pub instruments: Vec<MarketInstrument>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SandboxRequest {
    #[serde(rename_all = "camelCase")]
    Register {
        broker_account_type: BrokerAccountType,
    },
    SetCurrenciesBalance {
        currency: Currency,
        balance: f64,
    },
    SetPositionBalance {
        figi: String,
        balance: f64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SandboxAccount {
    pub broker_account_type: BrokerAccountType,
    pub broker_account_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum OrderRequest {
    MakeLimitOrder {
        operation: Operation,
        lots: i32,
        price: f64,
    },
    MakeMarketOrder {
        operation: Operation,
        lots: i32,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub tracking_id: String,
    pub status: Status,
    pub payload: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event")]
pub enum OutcomeEvent {
    #[serde(rename = "candle:subscribe")]
    CandleSubscribe {
        figi: String,
        interval: Interval,
        request_id: Option<String>,
    },
    #[serde(rename = "candle:unsubscribe")]
    CandleUnsubscribe {
        figi: String,
        interval: Interval,
        request_id: Option<String>,
    },
    #[serde(rename = "candle:unsubscribe")]
    OrderbookSubscribe {
        figi: String,
        depth: i32,
        request_id: Option<String>,
    },
    #[serde(rename = "orderbook:unsubscribe")]
    OrderbookUnsubscribe {
        figi: String,
        depth: i32,
        request_id: Option<String>,
    },
    #[serde(rename = "instrument_info:subscribe")]
    InstrumentInfoSubscribe {
        figi: String,
        request_id: Option<String>,
    },
    #[serde(rename = "instrument_info:unsubscribe")]
    InstrumentInfoUnsubscribe {
        figi: String,
        request_id: Option<String>,
    },
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    Ping(#[serde(default)] Vec<u8>),
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    Pong(#[serde(default)] Vec<u8>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event")]
pub enum IncomeEvent {
    #[serde(rename = "candle")]
    Candle {
        time: DateTime<Local>,
        payload: CandleEventPayload,
    },
    #[serde(rename = "orderbook")]
    OrderBook {
        time: DateTime<Local>,
        payload: OrderBookPayload,
    },
    #[serde(rename = "instrument_info")]
    InstrumentInfo {
        time: DateTime<Local>,
        payload: InstrumentInfoEventPayload,
    },
    #[serde(rename = "error")]
    Error {
        time: DateTime<Local>,
        payload: ErrorEventPayload,
    },
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    Binary(#[serde(default)] Vec<u8>),
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    Ping(#[serde(default)] Vec<u8>),
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    Pong(#[serde(default)] Vec<u8>),
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    Close,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CandleEventPayload {
    pub o: f64,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub v: f64,
    pub time: DateTime<Local>,
    pub interval: Interval,
    pub figi: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookEventPayload {
    pub figi: String,
    pub depth: i32,
    #[serde(default)]
    pub bids: Vec<(f64, f64)>,
    #[serde(default)]
    pub asks: Vec<(f64, f64)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentInfoEventPayload {
    pub trade_status: String,
    pub min_price_increment: f64,
    pub lot: f64,
    pub accrued_interest: Option<f64>,
    pub limit_up: Option<f64>,
    pub limit_down: Option<f64>,
    pub figi: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorEventPayload {
    pub error: String,
    pub request_id: Option<String>,
}
