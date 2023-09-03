use anyhow::{anyhow, Result};

#[allow(dead_code)]
pub fn assert_result<T: PartialEq>(expected: T, actual: T) -> Result<()> {
    if expected.eq(&actual) {
        Ok(())
    } else {
        let type_name = std::any::type_name::<T>();
        Err(anyhow!("assert equals for type {}", type_name))
    }
}

pub fn assert_result_msg<T: PartialEq>(expected: T, actual: T, msg: String) -> Result<()> {
    if expected.eq(&actual) {
        Ok(())
    } else {
        let type_name = std::any::type_name::<T>();
        Err(anyhow!(msg))
    }
}
