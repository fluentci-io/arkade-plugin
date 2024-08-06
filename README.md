# Arkade Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/arkade)](https://pkg.fluentci.io/arkade)
[![ci](https://github.com/fluentci-io/arkade-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/arkade-plugin/actions/workflows/ci.yml)

Install tools and Kubernetes applications with [Arkade](http://github.com/alexellis/arkade).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm arkade setup
```

## Functions

| Name    | Description                                |
| ------- | ------------------------------------------ |
| setup   | Install arkade                             |
| chart   | Chart utilites                             |
| get     | The get command downloads a tool           |
| install | Isntall Kubernertss app                    |
| oci     | oci apps                                   |
| system  | System apps                                |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/arkade@v0.1.0?wasm=1", "setup", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: arkade
    args: |
      get kind
- name: Show kind version
  run: |
    type arkade
    type kind
    kind version
```
