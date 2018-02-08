pub mod client;
pub mod network;



#[cfg(test)]
mod tests {
    use super::client; // super 回退一级 调用client模块
    #[test]
    fn it_works() {
        client::connect();
    }
}
