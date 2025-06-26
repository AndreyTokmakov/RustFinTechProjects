
#[path = "Networking/web_socket_binance.rs"] pub mod binance_web_sockets;


fn main() 
{
    binance_web_sockets::test_all();
}
