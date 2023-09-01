use async_trait::async_trait;


#[derive(Debug)]
pub struct PID {
    pub unique_id: String,
    pub node_id: String
}


#[async_trait]
pub trait GenServer {
    async fn call(
        state: &mut Vec<u8>,
        message: &Vec<u8>,
        call_id: &String,
        from: &PID
    );
}

