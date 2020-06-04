## xuper-rust-sdk

A Xuperchain SDK by rust, especially for TEE(Intel SGX/ARM TZ) application.

## Requirements

XuperChain 3.7

## Function

- [x] load account
- [x] Transfer
- [x] Contract Invoke/Query
- [ ] balance

## Notices when serializing

In protos/xchain.rs and protos/xendorser.rs:
* Serialize enum as number: https://serde.rs/enum-number.html
* #[serde(default)]
* crate::wallet::* 


## Test
```
cargo test -- --test-threads 1
```

## 迁移TEE
* SDK目前需要迁移到TEE的部分主要是wallet.rs, 把私钥保管到TEE内部。 
* SDK在TEE内部调用mesatee sdk访问可信账本的KMS服务，这段还需要迁移。 @XuperChain

