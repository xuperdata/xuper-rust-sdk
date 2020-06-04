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
* SDK目前需要迁移到TEE的部分主要是wallet.rs, 目前是加载文件的方式，可以改成私钥从参数传入, 把私钥保管到TEE内部。  @mesatee
* SDK在TEE内部调用mesatee sdk访问可信账本的KMS服务，这段还需要迁移。 @XuperChain

计划暴露的函数为:

* Ocall: crate::transfer::transfer, crate::contract::invoke/query_contract
* Ecall: crate::wallet::new/sign 

TEE内部发起数据链上执行流程为: 
1. TEE内部初始化Account（提前远程认证把秘钥放进去）；
2. Ocall调用crate::contract::invoke/query_contract；
> 1. 组装交易
> 2. 调用ecall sign 进行交易签名
> 3. 提交交易
