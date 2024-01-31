pub mod search;

use thirtyfour::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("GitHub API rate limit exceeded")]
    RateLimitError,
}

pub async fn check_for_rate_limit(driver: &WebDriver) -> Result<(), Error> {
    let rate_limit_detected = driver
        .find(By::XPath("//*[contains(text(), 'Whoa there!')]"))
        .await
        .is_ok();

    if rate_limit_detected {
        Err(Error::RateLimitError)
    } else {
        Ok(())
    }
}
