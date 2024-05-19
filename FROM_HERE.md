# rust example coverage

## [FROM HERE](https://www.reddit.com/r/rust/comments/j0ribi/rust_code_coverage_in_vscode/)

## cargo tarpaulin --out Lcov --skip-clean --ignore-tests

## [user settings](https://users.rust-lang.org/t/linux-vsc-rust-analyzer-cargo-check-run-on-every-change/44232)

"rust-analyzer.checkOnSave.enable": false

[follow up video](https://www.youtube.com/watch?v=JEgrkmEAHjA)

[Speed up Rust CI pipelines that use Tarpaulin](https://identeco.de/en/blog/speed-up-rust-ci-pipelines-that-use-tarpaulin/)

## cargo tarpaulin --out Lcov --skip-clean

cargo tarpaulin --target-dir target/tarpaulin-target/ --skip-clean

cargo tarpaulin --target-dir target/tarpaulin-target/ --skip-clean --out Lcov

## run one test

cargo tarpaulin --ignore-tests --target-dir target/tarpaulin-target/ --skip-clean --out Lcov  -- --nocapture test::num_test_smaller
