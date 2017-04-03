//! Bitcoin wallets integration module

use futures::{Future, Stream};
use tokio_core::reactor::Core;
use std::str;
use std::io::Read;
use std::sync::Arc;

// serialization
use serde_json;
use serde_json::Value as JSONValue;

// hyper
use hyper::method::{Post};
use hyper::header::{Authorization, Basic};

use error::Error as PerimeterError;

// Account and user identity
pub use identity::account::Account;

// COnstants TO DO move to secure storage
static RPC_USER: &'static [u8] = b"perimeter"; // rpcauth password
static RPC_PASSWORD: &'static [u8] = b"payEm"; // rpcauth password
static RPC_BASIC_AUTH: &'static [u8] = b"cGVyaW1ldGVyOlBheUVt"; // rpcauth

/// Bitcoin Core wallet json RPC request general form
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletRPCRequest {
    jsonrpc : f64,
    id : Option<String>,
    method : String,
    params : Vec<JSONValue>,
}

impl WalletRPCRequest {

    /// Constructor
    pub fn new( method : String, params: Vec<JSONValue>, id : Option<String>) -> WalletRPCRequest {
        WalletRPCRequest {
            jsonrpc : 1.0,
            id : id,
            method : method,
            params : params,
        }
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletRPCError {
    code : u64,
    message : String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletRPCResponse {
    result : JSONValue,
    error : WalletRPCError,
    id : Option<String>,
}

#[derive(Debug)]
pub struct BitcoinCoreWallet {
    node_url : Uri,
    rpc_user : String,
    rpc_pass : String,
    reactor : Arc<Core>,
}

impl BitcoinCoreWallet {

    /// Constructor with specified node RPC-JSON interface endpoint url and auth credentials
    pub fn new(url : &str) -> BitcoinCore {
        BitcoinCore{
            node_url : hyper::Url::parse(url).unwrap(),
            rpc_user : RPC_USER.to_string(),
            rpc_pass : RPC_PASSWORD.to_string(),
        }
    }

    ///Execute single json rpc request and return future with response
    pub fn execute_json_rpc( rpc_request : WalletRPCRequest ) -> impl Future<Item = WalletRPCResponse, Error = PerimeterError> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        // Make request to server
        let method = Post;
        let path = url.clone().path().to_string();
        let mut headers = Headers::new();
        let mut data = serde_json::to_string(&rpc_request).unwrap();

        //Set basic headers
        headers.set(Accept::json());
        headers.set(ContentLength(data.len() as u64));
        headers.set(ContentType::json());

        // Authorization magic
        headers.set(Authorization::Basic(RPC_BASIC_AUTH));

        let mut body = String::new();
        {
            let mut request = client.request(method, url);
            request = request.headers(headers);
            request = request.body( data.as_bytes() );

            // Obtain response
            let mut response = request.send().unwrap();
            response.read_to_string(&mut body).unwrap();
            println!("{}", response.status);
            println!("{}", response.headers);
        }
        let v : WalletRPCResponse = serde_json::from_str(&body).unwrap();

        println!("{}", serde_json::to_string_pretty(&v).unwrap());

        futures::future::ok(v)
    }

    /// The getnewaddress RPC returns a new Bitcoin address for receiving payments.
    /// If an account is specified, payments received with the address will be credited to that account.
    // pub fn getnewaddress( account : Account ) {
    //     execute_json_rpc( WalletRPCRequest{ method : "getnewaddress", params : "[ ]", ...} )
    // }
}
