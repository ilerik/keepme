//! Identity managment subsystem and related concepts implementation module

use futures::{Future, Stream};
use futures::future;

// crypto
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;

// hyper
use hyper::header;
use hyper::header::{Authorization, Bearer};
//
// // jwt
// use std::default::Default;
// use crypto::sha2::Sha256;
// use jwt::{
//     Header,
//     Registered,
//     Token,
// };

//serialization
//use serde_json;

pub mod account;

// reexports
pub use error::Error as Error; // Error type
pub use self::account::Account; // Permanent user account


/// Auth request sent by user
pub struct AuthentificationRequest {
    email : String,
    password : String,
}

/// Levels of authorization
pub enum AuthorizationLevel {
    Basic,
    Weak,
    Strong
}

/// Token that serves as a proof of identity and is issued for a limited time
pub struct AuthorizationToken {

}

/// Main class that abstracts identity concept
pub struct Identity {
    pub account : Account,
    pub auth_level : AuthorizationLevel,
}

impl Identity {

    /// Constructor
    pub fn new( account : Account ) -> Identity {
      Identity {
        account : account,
        auth_level : AuthorizationLevel::Basic,
      }
    }

    /// Try to produce identity from request
    pub fn authentificate( auth_request : AuthentificationRequest ) -> impl Future<Item = Identity, Error = PerimeterError> {
        match (&auth_request.email as &str, &auth_request.password as &str) {
            ( "admin", "admin" ) => future::ok(Identity::new(Account::admin())),
            (_ , _) => future::err(PerimeterError::AuthentificationError),
        }
    }

}

#[cfg(test)]
mod tests {
    use exchanges::bitmex::auth::generate_signature;
    use std::str;

    //
    #[test]
    fn identity_test() {
        assert!(true);
    }

}
