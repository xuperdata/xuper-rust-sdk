
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use grpc::ClientStubExt;

use crate::config;
use crate::protos::xchain_grpc;
use crate::protos::xendorser_grpc;


pub struct XChainClient {
    pub chain_name: String,

    pub endorser: xendorser_grpc::xendorserClient,

    pub xchain: xchain_grpc::XchainClient,
}

#[allow(dead_code)]
impl XChainClient {
    pub fn new(bcname: &String) -> Self {
        let host = config::CONFIG.read().unwrap().node.clone();
        let port = config::CONFIG.read().unwrap().endorse_port;
        let port_xchain = config::CONFIG.read().unwrap().node_port;

        //TODO: 设置超时，以及body大小
        let client_conf = Default::default();
        let client_endorser =
            xendorser_grpc::xendorserClient::new_plain(host.as_str(), port, client_conf)
                .expect("new connection");

        let client_conf = Default::default();
        let client_xchain =
            xchain_grpc::XchainClient::new_plain(host.as_str(), port_xchain, client_conf)
                .expect("new connection");

        XChainClient {
            chain_name: bcname.to_owned(),
            endorser: client_endorser,
            xchain: client_xchain,
        }
    }
}
