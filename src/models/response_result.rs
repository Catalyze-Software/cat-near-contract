use near_sdk::near;

use crate::error::GenericError;

#[derive(Clone, Debug)]
#[near(serializers = ["json"])]
pub enum ResponseResult<T> {
    Ok(T),
    Err(GenericError),
}
