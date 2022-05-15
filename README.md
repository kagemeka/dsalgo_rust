# DsAlgo Rust

Data Structures and Algorithms for Rust.

[![Crates.io][crates-badge]][crates-url]
[![Github pages][gh-pages-badge]][gh-pages-url]
[![MIT licensed][mit-badge]][mit-url]
[![CI][actions-badge]][actions-url]
[![pre-commit][pre-commit-badge]][pre-commit-url]

[crates-badge]: https://img.shields.io/crates/v/dsalgo.svg
[crates-url]: https://crates.io/crates/dsalgo
[gh-pages-badge]: https://github.com/kagemeka/dsalgo_rust/actions/workflows/pages/pages-build-deployment/badge.svg
[gh-pages-url]: https://kagemeka.github.io/dsalgo_rust
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/kagemeka/dsalgo_rust/blob/main/LICENSE
[docs-url]: https://docs.rs/dsalgo
[actions-badge]: https://github.com/kagemeka/dsalgo_rust/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/kagemeka/dsalgo_rust/actions/workflows/rust.yml
[pre-commit-badge]: https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white
[pre-commit-url]: https://github.com/pre-commit/pre-commit

## module naming rule

* basically,
  + <theme>_<algorithm/data_structure>_<ext>.rs
    - if theme is trivial, it can be omitted.
    - for example, `floyd_warshall` is a algorithm only used for shortest path.
    - extentional impl section should be separeted with absolute core API.

## project rules

* do not introduce additional directory depth for this project.
* instead, split files into small pieces, and connect them by `use` keyword.
* run ./script/ci.sh before commit and push.
* by defining abstract algebra and precice traits, preserve strict `DRY` principle and beautiful codes.
