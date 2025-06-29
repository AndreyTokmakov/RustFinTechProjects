
use crypto_ws_client::{ OkxWSClient, WSClient};

#[tokio::main]
async fn get_trades()
{
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::task::spawn(async move {
        let symbols: Vec<String> = vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()];
        let ws_okx_client: OkxWSClient = OkxWSClient::new(tx, None).await;
        ws_okx_client.subscribe_trade(&symbols).await;
        // run for 5 seconds
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), ws_okx_client.run()).await;
        ws_okx_client.close().await;
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
        let symbols: Vec<String> = vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()];
        let ws_okx_client: OkxWSClient = OkxWSClient::new(tx, None).await;
        ws_okx_client.subscribe_bbo(&symbols).await;
        // run for 5 seconds
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), ws_okx_client.run()).await;
        ws_okx_client.close().await;
    });

    // let mut messages = Vec::new();
    for msg in rx {
        println!("{}", msg);
        // messages.push(msg);
    }
}

pub fn test_all()
{
    // get_trades();
    get_bbo();
}
