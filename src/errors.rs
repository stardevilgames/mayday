use thiserror::Error;

#[derive(Debug, Error)]
pub enum MaydayCrash {

}

pub type Result<T> = std::result::Result<T, MaydayCrash>;
