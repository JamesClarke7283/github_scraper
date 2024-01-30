# TODO List for GitHub Scraper

## Overall Project Setup
- [x] Set up the basic Rust project structure.
- [x] Initialize git repository.
- [x] Write initial README.md.
- [x] Create LICENSE file.
- [x] Set up Selenium WebDriver dependency.

## Development
- [ ] Implement scraping functionality.

  ### Search Functionality
  - [ ] Implement plain text search.
  - [ ] Implement search with GitHub filters.

  ### Repository Information
  - [ ] Scrape main page data (stars, forks, license, description, topics).
  - [ ] Scrape language usage percentages.
  - [ ] Retrieve HTTPS and SSH clone URLs.

  ### Issues
  - [ ] Scrape issues from repositories.

  ### Pull Requests
  - [ ] Scrape pull requests from repositories.

  ### Releases
  - [ ] Scrape release information for repositories.

  ### Contributors
  - [ ] List contributors with profile links.
  - [ ] Extract contributors' contributions details (lines added, removed, number of commits).

## Testing
- [ ] Write unit tests for each functionality.
- [ ] Perform integration testing.

## Documentation
- [ ] Document each function in the code.
- [ ] Update README.md with usage examples.
- [ ] Create comprehensive library documentation.

## Deployment
- [ ] Finalize all features and testing.
- [ ] Publish the crate to crates.io.
