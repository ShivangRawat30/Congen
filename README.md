# Congen

Generate perfect Contract Info and code headers every time.

## Build

You need Rust and Cargo installed on your machine. See the installation guide
[here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Then clone the repo and install the CLI globally like this:

```sh
cargo install --git https://github.com/ShivangRawat30/Congen.git.
```

## Usage

### Contract-info
```sh
congen contract-info "A,B,C,D"
```

```sh
///////////////////////////
/**
* @title A
* @author B
* @notice C
* @dev D
*
*/
///////////////////////////
```

### Code-Headers
```sh
congen header Hello Everyone
```

```sh
/*//////////////////////////////////////////////////////////////
                        HELLO EVERYONE
//////////////////////////////////////////////////////////////*/
```

