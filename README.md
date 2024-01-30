# GitHub Scraper

`GitHub Scraper` is a versatile Rust library specifically designed for extracting information from GitHub repositories. This library provides a straightforward way to access GitHub data programmatically, without using the official API. It's particularly useful for developers and researchers who need to gather detailed data from GitHub for analysis, research, or development purposes.

## Features

- Efficiently access a wide range of data from GitHub repositories.
- Ideal for extracting detailed repository information such as statistics, contributions, issue data, etc.
- Simplified interface, making it easy to integrate into Rust applications.

## Installation

`GitHub Scraper` requires a Selenium WebDriver instance for functioning, as it uses browser automation for scraping data. Make sure you have Selenium set up in your environment. To add `GitHub Scraper` to your Rust project, include it in your `Cargo.toml` file:

```toml
[dependencies]
github_scraper = "0.1.0"
thirtyfour = "0.1.0"
```

## Usage

Integrate `GitHub Scraper` into your Rust application with ease. Here's a quick example to illustrate its usage:

```rust
use github_scraper::GitHub;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let driver = WebDriver::new("http://localhost:4444", &DesiredCapabilities::chrome()).await?;

    let github = GitHub::new(driver);
    // Example: Retrieve data from a specific GitHub repository
    let repo_data = github.repository("user/repository_name").unwrap();
    println!("{:?}", repo_data);

    Ok(())
}
```

Ensure you have the appropriate WebDriver server (like ChromeDriver or GeckoDriver) running at the specified URL.

## Dependencies

Key dependencies of `GitHub Scraper` include:
- `thirtyfour` for Selenium WebDriver support.
- Additional Rust crates for HTTP requests, JSON parsing, and more.

## License

`GitHub Scraper` is released under the GNU Lesser General Public License v3.0 or later (LGPLv3 or later). For more information, please refer to the [LICENSE](./LICENSE) file in this repository.

## Disclaimer

This library is provided "as is", with no warranty. Users must adhere to GitHub's terms of service when using `GitHub Scraper`. The authors are not liable for any misuse or damages arising from the use of this library.
