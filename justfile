_:
    @just --list

# setup required dependencies for developing trt on macos
[macos]
setup:
    brew install markdownlint-cli
    brew install yamlfmt
    brew install taplo

# format the project
format:
    cargo fmt --all
    markdownlint --fix ./README.md .github/README.md
    yamlfmt .github/workflows/*
    taplo fmt
    just --fmt
