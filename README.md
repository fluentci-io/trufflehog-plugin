# Trufflehog Plugin

[![ci](https://github.com/fluentci-io/trufflehog-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/trufflehog-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [trufflehog](https://github.com/trufflesecurity/trufflehog).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm trufflehog setup
```

## Functions

| Name       | Description                                  |
| ---------- | -------------------------------------------- |
| setup      | Installs a specific version of trufflehog.   |
| git        | Find credentials in git repositories         |
| github     | Find credentials in GitHub repositories.     |
| gitlab     | Find credentials in GitLab repositories.     |
| filesystem | Find credentials in a filesystem.            |
| s3         | Find credentials in S3 buckets.              |
| gcs        | Find credentials in GCS buckets              |
| syslog     | Scan syslog                                  |
| circleci   | Scan CircleCI                                |
| docker     | Scan Docker Image                            |
| travisci   | Scan TravisCI                                |
| postman    | Scan Postman                                 |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/trufflehog@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: trufflehog
    args: |
      setup
- name: Show trufflehog version
  run: |
    type trufflehog
    trufflehog --version
```
