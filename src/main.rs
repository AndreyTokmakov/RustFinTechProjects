
#[path = "Networking/binance_ws_client_1.rs"] pub mod binance_ws_client_1;
#[path = "Networking/binance_ws_client_2.rs"] pub mod binance_ws_client_2;
#[path = "Networking/okx_ws_client.rs"] pub mod okx_ws_client;


fn main() 
{
    // binance_ws_client_1::test_all();
    // binance_ws_client_2::test_all();
    okx_ws_client::test_all();
}
