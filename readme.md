## xuper-rust-sdk

A Xuperchain SDK by rust, especially for TEE(Intel SGX/ARM TZ) application.

## Requirements

1. Xuperchain 3.7

    Clone xuperchain [source code](https://github.com/xuperchain/xuperchain/tree/v3.7) and follow the [instruction](https://github.com/xuperchain/xuperchain/wiki/3.-Getting-Started) to build a single-node or multi-node network.

## Function

- [x] load account
- [x] Transfer
- [x] Contract Invoke/Query
- [ ] balance

## Notices when serializing

In protos/xchain.rs and protos/xendorser.rs:
* Serialize enum as number: https://serde.rs/enum-number.html
* \#[serde(default)]
* crate::wallet::* 


## Test

1. Xuperchain configuraton

    You may need to modify the xuperchain configuration in file "conf/sdk.yaml" according to your xuperchain network.

2. Run tests
```
cargo test -- --test-threads 1
```

## Run in Intel SGX
Refer to [trusted-xuper-rust-sdk](https://github.com/xuperdata/trusted-xuper-rust-sdk)
