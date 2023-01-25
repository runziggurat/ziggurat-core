[![dependency status](https://deps.rs/repo/github/runziggurat/ziggurat-core/status.svg)](https://deps.rs/repo/github/runziggurat/ziggurat-core)

# ziggurat-core package

*Note:* This project is a work in progress.

ziggurat-core is a set of packages that provides basic functionality for all Ziggurat projects. Each subset of 

## Prerequisites

Ziggurat is written in stable Rust; you can install the Rust toolchain by following the official instructions [here](https://www.rust-lang.org/learn/get-started).

## Getting started

ziggurat-core is intended to be used within the other ziggurat projects, providing it the common feature subset. ziggurat-core is workspace, which is divided into sub-libraries which implement specific features withing one area.

Currently, the following sub-libraries are available:
```
ziggurat-core-geoip - GeoIP functionality
ziggurat-core-metrics - provides metrics functionality
ziggurat-core-utils - provides general purpose utilities
```

To use it, simply include it in your Cargo.toml file:
```
[dependencies]
ziggurat-core-metrics = { git = "https://github.com/runziggurat/ziggurat-core" }
```

You can use git tags or branches to specify exact verion of each library you wish to use.

## Development

### Versioning

ziggurat-core use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/runziggurat/ziggurat-core). 
Each sub-package is versioned individually.

### Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Commits should follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

Please make sure to update tests as appropriate.