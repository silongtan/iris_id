# Iris Identification

A cloud project to identify iris species. 

## Model Construction

Using the dataset from UCI Machine Learning Repository, we can construct a model to identify iris species locally using Rust Linfa.

## Usage

First, install Rust and Cargo using the following command:

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

Then, clone the repository and run the following command:

```cargo run --release```

## Todo

- [ ] S3 data upload
- [ ] AWS SageMaker + S3
- [ ] AWS SageMaker

## References

* [Rust](https://www.rust-lang.org/)
* [Iris Data Set](https://archive.ics.uci.edu/ml/datasets/iris)
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)