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
```toml
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

### Nix

`ziggurat-core`, and its derivatives, make use of Nix to provide reproducable and declarative benefits to our CI/CD pipeline. These are detailed below:

#### Development Environment

A development environment is provided with all the necessary dependencies and a few utilities that allows for running CI checks locally. This is used extensively in the check and lint workflow to guarantee that a local CI pass will result in a remote one as well.

The interactive development shell can be entered via this command:
```fish
nix develop
```

##### Extending

The `ziggurat-core` development environment is made to be a lightweight solution with all the necessities, while at the same remaining extendable enough to account for per-implementation specifics. As such, the set of core utilities can be extended at will by modifying the projects `flake.nix` file, like so:

You can define additional CI scripts by creating an attribute set with the custom commands:
```nix
scripts = {
  <name> = <command>;
} // ziggurat-core.scripts;
```
*Note: this only affects local CI scripts for now. Manual invocation in the workflow is still needed.*

You can also define additional dependencies by listing them in the `buildInputs` list:

```nix
devShells.default = pkgs.mkShell {
  # Enter additional build dependencies here.
  buildInputs = [ ]
    ...
};
```

#### Ziggurat Template

A Nix template is also provided to make it easy to bootstrap future projects and implementations. Running the following command, from the new root directory, will instantiate a bare-bones, but extendable, `flake.nix` file for working with Ziggurat:
```fish
nix flake init -t github:runziggurat/ziggurat-core
```
