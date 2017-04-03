//! Account structure represents user account and related identity information. In future we plan
//! to add support for self-soverign identity mangement (e.g. uPort)
use rcrypto::sha2::Sha256;
use ethkey::{Random, Generator, Public, Secret, Address, KeyPair};

//reexports
//pub use identity::Indentity;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub master_key : Public, // Master key that identifies account uniquely
    pub username : String, // Username used as login
    pub password_hash : Vec<u8>, // Hashed password used for basic auth
    pub email : Option<String>, // Used for weak authorization via Google Auth
    pub ethereum_address : Option<Address>, // Ethereum public address used to authorize user via EWT
}

#[derive(Serialize, Deserialize)]
pub struct AccountSecret {
    pub master_private_key : Secret, // Secret part of the account used to recreate account
}

impl Account {

    /// Constructor
    pub fn new( username: String, password: String ) -> (Account, AccountSecret) {
        let mut hashed_password = [0u8; 32];
        let keypair = Random.generate().unwrap();
        let mut hasher = Sha256::new();
        ( Account {
                master_key : keypair.public(),
                username : username,
                password_hash : hashed_password,
                email : "",
                ethereum_address : keypair.address(),
        },
        AccountSecret {
                master_private_key : keypair.secret(),
        }) // return tuple
    }

    /// Constructor for tests
    pub fn test() -> Account {
        Account::new( "test", "test" )
    }
}
