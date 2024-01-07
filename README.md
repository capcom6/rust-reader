# Rust RSS Feed Reader

## Introduction

This Rust RSS Feed Reader is a command-line application that reads and displays content from RSS feeds provided by the user. With this tool, you can easily follow updates from your preferred news websites, blogs, podcasts, and more directly from your terminal.

For those unfamiliar with RSS, it's a standard for syndicating content updates from websites. To learn more about how RSS feeds work, check out this [wikipedia article](https://ru.wikipedia.org/wiki/RSS).

This is an open source project from [DevProjects](http://www.codementor.io/projects). Feedback and questions are welcome!

Find the project requirements here: [RSS feed reader in terminal](https://www.codementor.io/projects/tool/rss-feed-reader-in-terminal-atx32jp82q)

### Disclaimer

This Rust RSS Feed Reader is an educational project developed with the primary objective of practicing and improving Rust programming skills. It serves as a learning tool and a demonstration of coding practices in Rust.

Please note the following:

- The application is in a developmental stage and is not considered production-ready.
- It may lack features commonly expected in a full-fledged RSS feed reader.
- The project may contain bugs or issues as it is a platform for learning and experimentation.
- There is no guarantee of regular updates, feature enhancements, or bug fixes.
- The author is not responsible for any issues that arise from using this software for real-world applications.

## Features

- **Fetch RSS Feeds:** Input one or more RSS feed URLs to get the latest content updates.
- **Display Content Details:** The reader shows the title, description, and link of the original content.
- **Limit Feed Items:** Choose the number of items to display.

## Installation

To get started with the Rust RSS Feed Reader, follow these steps:

```shell
git clone https://github.com/capcom6/rust-reader.git
cd rust-reader
cargo build --release
```

## Usage

To use the RSS Feed Reader, execute the binary with the desired RSS feed URLs:

```shell
cargo run -- [OPTIONS] <URLS>...
```

Or if you're running the compiled binary directly:

```shell
./target/release/rust-reader [OPTIONS] <URLS>...
```

### Arguments

* `<URLS>...`: One or more URLs to the RSS feeds you want to read.

### Options

* `-c, --count <COUNT>`: Specify the total number of items to display.
* `-h, --help`: Print help information and usage instructions.
* `-V, --version`: Print the version information of the application.

### Examples

Reading from a single RSS feed:

```shell
cargo run -- "https://example.com/rss"
```

Reading from multiple RSS feeds with a limit on the number of items:

```shell
cargo run -- -c 5 "https://example.com/rss" "https://anotherexample.com/rss"
```

Example output:

```
  1: (2024-01-06 15:58:08.0 +00:00:00) Switching to a Flip Phone Helped Me Cut Down on My Smartphone Addiction
        Was it inconvenient? Yes. Did T9 texting drive me crazy? Definitely. Was it worth doing? Absolutely.
        https://www.nytimes.com/2024/01/06/technology/smartphone-addiction-flip-phone.html
```

## Roadmap

The following is a list of potential enhancements and features that are planned or under consideration for future versions of the Rust RSS Feed Reader. These improvements aim to extend functionality, improve user experience, and foster a more robust application.

- **Offline Reading:** Ability to save feed items for offline reading.
- **Full Text Retrieval:** Implement a feature to fetch the full text of an article when it's not provided in the RSS feed summary.
- **Feed Management:** Add support for adding, removing, and organizing feeds within the application.
- **Filtering Options:** Introduce filters to exclude or include items based on keywords, authors, or tags.
- **Search Functionality:** Implement a search feature to find specific articles within fetched feeds.
- **Multi-language Support:** Add support for multiple languages in the feed output.

This roadmap is subject to change, and new ideas are always welcome. If you have suggestions or want to contribute to any of these features, please feel free to open an issue or pull request on the project repository.

## Contributing

If you'd like to contribute to the Rust RSS Feed Reader, please follow these steps:

1. Fork this repository.
2. Create a new branch: git checkout -b <branch_name>.
3. Make your changes and commit them: git commit -m '<commit_message>'.
4. Push to your branch: git push origin <branch_name>.
5. Create a pull request.

For more information on creating a pull request, see [GitHub's documentation](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests).

## License

This project is open-source and available under the Apache License 2.0. For the full text of the license, see the [LICENSE](LICENSE) file in this repository.