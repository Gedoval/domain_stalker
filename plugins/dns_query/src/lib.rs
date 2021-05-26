mod answer;

use plugins_core::{Arg, Help, InvocationError, Plugin, PluginRegistrar};
use std::str::FromStr;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use trust_dns_client::udp::UdpClientConnection;

plugins_core::export_plugin!(register);
const HOST_ARG: &str = "host";
const QUERY_TYPE_ARG: &str = "query_type";

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_function("query", Box::new(DnsQuery));
}

fn query(host: &str, record: &str) {
    println!("[*] Querying Record {} on {}", record, host);
    let name_server = "8.8.8.8:53".parse().unwrap();
    let conx = UdpClientConnection::new(name_server).unwrap();
    let client = SyncClient::new(conx);

    let name = Name::from_str(host).unwrap();
    let query_type = RecordType::from_str(record.to_ascii_uppercase().as_str()).unwrap();
    let response = client.query(&name, DNSClass::IN, query_type).unwrap();
    answer::parse_answers(response.answers(), query_type);
}

pub struct DnsQuery;

impl Plugin for DnsQuery {
    fn required_args(&self) -> Vec<&str> {
        return vec![HOST_ARG, QUERY_TYPE_ARG];
    }

    fn call(&self, args: std::collections::HashMap<String, String>) -> Result<(), InvocationError> {
        for arg in self.required_args() {
            if args.get(arg).is_none() {
                return Err(InvocationError::MissingArgument {
                    expected: String::from(arg),
                });
            }
        }

        query(
            args.get(HOST_ARG).unwrap(),
            args.get(QUERY_TYPE_ARG).unwrap(),
        );

        Ok(())
    }

    fn help(&self) -> Help {
        return Help {
            description: self.description(),
            args: vec![
                Arg {
                    name: HOST_ARG,
                    desc: "Host or IP to query"
                },
                Arg {
                    name: QUERY_TYPE_ARG,
                    desc: "Query type to use against the host or IP"
                }
            ]
        };
    }

    fn description(&self) -> &str {
        "Perform DNS queries to the specified domain.\nCurrently, the supported query types are A, NS, SOA, TXT, MX, AAAA, CNAME."
    }
}
