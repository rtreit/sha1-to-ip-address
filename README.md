# SHA1 to IP Address

This project is a Rust application that finds an IP address whose SHA1 hash matches a given hash. The program utilizes parallel processing to speed up the search using the `rayon` library.

## Usage

### Prerequisites

- Rust and Cargo installed. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository or download the source code.

2. Navigate to the project directory:

```sh
cd sha1_ip_finder
```

3. Build the project in release mode:

```sh
cargo build --release
```

### Running the Program
To run the program, you need to provide the hash to check as a command-line argument. The program will print the matching IP address, the total number of hashes checked, and the time elapsed during the search.

Example:
```sh
./target/release/sha1_ip_finder <hash_to_check>
```

To run the program, you need to provide the hash to check as a command-line argument. The program will print the matching IP address, the total number of hashes checked, and the time elapsed during the search.

Output:
```sh
Hash to check: a12c8a141cf35dd84a43183ffd8a12d200d5892f
Found IP: 173.231.209.34
Matching IP found: 173.231.209.34
Total hashes checked: 3767705046
Time elapsed: 1088.1075607s
```

### Implementation details
* The program uses the sha1 crate for computing SHA1 hashes.
* The rayon crate is used to parallelize the computation across all available CPU cores.
* The program uses Arc and Mutex from the std::sync module to safely share state between threads.
* The elapsed time is measured using the std::time::Instant module
