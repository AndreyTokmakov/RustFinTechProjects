use crypto_ws_client::{BinanceSpotWSClient, BinanceWSClient, WSClient};


#[tokio::main]
async fn get_trades()
{
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::task::spawn(async move {
        let symbols: Vec<String> = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];
        let ws_client = BinanceSpotWSClient::new(tx, None).await;
        ws_client.subscribe_trade(&symbols).await;
        // run for 5 seconds
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), ws_client.run()).await;
        ws_client.close().await;
    });

    // let mut messages = Vec::new();
    for msg in rx {
        println!("{}", msg);
        // messages.push(msg);
    }
}

#[tokio::main]
async fn get_bbo()
{
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::task::spawn(async move {
        let symbols: Vec<String> = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];
        let ws_client = BinanceSpotWSClient::new(tx, None).await;
        ws_client.subscribe_bbo(&symbols).await;
        // run for 5 seconds
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), ws_client.run()).await;
        ws_client.close().await;
    });

    // let mut messages = Vec::new();
    for msg in rx {
        println!("{}", msg);
        // messages.push(msg);
    }
}

#[tokio::main]
async fn get_ticker()
{
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::task::spawn(async move {
        let symbols: Vec<String> = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];
        let ws_client = BinanceSpotWSClient::new(tx, None).await;
        ws_client.subscribe_ticker(&symbols).await;
        // run for 5 seconds
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), ws_client.run()).await;
        ws_client.close().await;
    });

    for msg in rx {
        println!("{}", msg);
    }
}




// https://crates.io/crates/crypto-ws-client
pub fn test_all()
{
    // get_trades();
    // get_bbo();
    get_ticker();
}
