use async_stream::stream;
use futures::future::join_all;
use futures::stream::Stream;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration, Instant};

pub struct RepositorySearchResult {
    pub url: String,
}

pub async fn new<'a>(
    query: &'a str,
    driver: WebDriver,
) -> impl Stream<Item = RepositorySearchResult> + 'a {
    stream! {
        // Navigate to the GitHub search page
        driver.goto("https://github.com/search?type=repositories").await.unwrap();

        // Find the search input and enter the query
        let search_input = driver.find(By::Css("input[aria-label='Search GitHub']")).await.unwrap();
        search_input.send_keys(query).await.unwrap();
        search_input.send_keys("\u{E007}").await.unwrap(); // Unicode for Enter key

        // Wait for the search results to load
        driver.find(By::Css("div[data-testid='results-list']")).await.unwrap();

        loop {
            let repos = extract_repositories(&driver).await;
            for repo in repos {
                yield repo;
            }

            // Logic to navigate to the next page, if necessary
            if let Some(next_button) = driver.find_all(By::Css("a[aria-label='Next Page']")).await.unwrap().first() {
                next_button.click().await.unwrap();
                sleep(Duration::from_secs(2)).await; // wait for the page to load
            } else {
                break;
            }
        }
    }
}

async fn extract_repositories(driver: &WebDriver) -> Vec<RepositorySearchResult> {
    let repo_elements = driver
        .find_all(By::Css("div[data-testid='results-list'] > div"))
        .await
        .unwrap();

    let futures = repo_elements.iter().map(|elem| async {
        let repo_url = extract_repository_url(elem).await;
        RepositorySearchResult { url: repo_url }
    });

    join_all(futures).await // Await all futures here
}

async fn extract_repository_url(elem: &WebElement) -> String {
    // More generic targeting of the <a> tag within 'search-title' div
    let a_tag = elem
        .find(By::Css("div.search-title > a"))
        .await
        .expect("Failed to find <a> tag within 'search-title' div");
    let href = a_tag
        .attr("href")
        .await
        .expect("Failed to get href attribute")
        .expect("Href attribute is missing");

    // Prepending the base URL if href is a relative path
    format!("https://github.com{}", href)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::pin_mut; // Required for pin_mut!
    use futures::StreamExt; // Needed for .next() on streams
    use tokio;

    async fn setup_driver() -> WebDriver {
        let caps = DesiredCapabilities::chrome();
        WebDriver::new("http://127.0.0.1:4444", caps).await.unwrap()
    }

    #[tokio::test]
    async fn test_extract_repository_url() {
        let driver = setup_driver().await;
        driver
            .goto("https://github.com/search?q=rust-lang")
            .await
            .expect("Failed to navigate to GitHub search");

        let mut found = false;
        while !found {
            // Wait for the search results to be visible
            let results_list = driver
                .find(By::Css("div[data-testid='results-list']"))
                .await
                .expect("Search results did not load");

            // Attempt to find the repository URL within the current page
            if let Ok(elem) = results_list.find(By::Css("div.search-title > a")).await {
                found = true;
                let url = extract_repository_url(&elem).await;
                assert!(
                    url.contains("https://github.com/rust-lang/"),
                    "URL does not match expected pattern: {}",
                    url
                );
            } else {
                // If not found, attempt to click the next page button
                if let Ok(next_button) = driver.find(By::Css("a[aria-label='Next Page']")).await {
                    next_button
                        .click()
                        .await
                        .expect("Failed to navigate to the next page");
                    sleep(Duration::from_secs(2)).await; // Wait for the next page to load
                } else {
                    // If the next page button is not found, break the loop
                    break;
                }
            }
        }

        assert!(
            found,
            "Failed to find the test repository URL within the paginated search results."
        );

        driver.quit().await.expect("Failed to quit WebDriver");
    }

    #[tokio::test]
    async fn test_extract_repositories() {
        let driver = setup_driver().await;
        driver
            .goto("https://github.com/search?q=rust-lang")
            .await
            .unwrap();
        let repos = extract_repositories(&driver).await;
        assert!(!repos.is_empty());
        driver.quit().await.unwrap(); // Ensure the WebDriver quits after the test
    }

    #[tokio::test]
    async fn test_new() {
        let driver = setup_driver().await;
        let repo_stream = new("rust-lang", driver.clone()).await;

        pin_mut!(repo_stream); // Pin the stream before using it

        if let Some(repo_result) = repo_stream.next().await {
            assert!(!repo_result.url.is_empty());
        } else {
            panic!("No repositories found");
        }
        driver.quit().await.unwrap(); // Ensure the WebDriver quits after the test
    }
}
