# MangaBuddy Source for Aidoku

## Overview
MangaBuddy is a custom source for the Aidoku app, designed to fetch and display manga content from the Mangabuddy website. This project includes all necessary files and configurations to build and deploy the source.

## Project Structure
```
en.mangabuddy
├── src
│   ├── lib.rs        # Main library code for MangaBuddy source
│   ├── parser.rs     # Content parsing logic
│   ├── helper.rs     # Utility functions
│   └── utils.rs      # Additional helper functions
├── public
│   └── index.min.json # Metadata for the source
├── Cargo.toml        # Rust project configuration
├── aidoku.toml       # Aidoku-specific configuration
├── .gitignore        # Git ignore rules
└── README.md         # Project documentation
```

## Installation
To set up the MangaBuddy source, follow these steps:

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/en.mangabuddy.git
   cd en.mangabuddy
   ```

2. **Install dependencies:**
   Ensure you have Rust and Cargo installed. Then run:
   ```bash
   cargo build
   ```

3. **Initialize Aidoku:**
   If you haven't already, install Aidoku CLI:
   ```bash
   cargo install --git https://github.com/Aidoku/aidoku-rs aidoku-cli
   aidoku init
   ```

4. **Build the source:**
   From the project root, run:
   ```bash
   aidoku build
   ```

## Usage
To use the MangaBuddy source in the Aidoku app:

1. Open Aidoku and navigate to **Sources**.
2. Select **Add custom source**.
3. Paste the following URL:
   ```
   https://follyofmine.github.io/Mangabuddy-aidoku/index.min.json
   ```
4. Add the MangaBuddy source.

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for details.