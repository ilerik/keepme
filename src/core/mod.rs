//! Core server logic and definitions
//!
//! # Motivation
//!
//! This particular software is intended to provide means of authentificated and secure
//! control for all kinds of digital assets.
//!
//! # General design
//!
//! ## Identity managment, user accounts and authorization
//! ## Information flow and capability based security model
//! ## Triple entry bookeeping, and data storage design

use futures::{Future, Stream};
use futures::future;
use hyper::method::Method::{Delete, Get, Post, Options};
use hyper::header::{Headers, ContentType, ContentLength};
use unicase::UniCase;
use hyper::header::{AccessControlAllowOrigin, AccessControlAllowCredentials, AccessControlAllowHeaders, AccessControlAllowMethods};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::server::{Handler, Server, Request, Response};
use hyper::status::{StatusCode};
use hyper::uri::RequestUri::AbsolutePath;
use net2::unix::UnixTcpBuilderExt;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use serde_json::Value as JSONValue;
use serde_json;

use std::thread;
use std::path::Path;
use std::net::SocketAddr;
use std::sync::Arc;
use std::io::{Read, Write};
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use std::str;
use std::io;

// TODO: move to identity =======================

// rustc_serialize
use rustc_serialize::json::{Json, ToJson};

// hyper
use hyper::header::{Authorization, Bearer};

// jwt
use std::default::Default;
use crypto::sha2::Sha256;
use jwt::{ Header, Registered, Token };

// =============================================

// Identity managment and authnetification middlware
// use identity::Identity;
// use identity::{authorize_request};

// Connectors to external services

// Submodules
pub mod identity; // Identity managment, auth and access levels related functions and types
pub mod wallets; // Wallets managment

// Constants
static AUTH_SECRET: &'static str = "some_secret_key___";
static USER_EMAIL: &'static str = "test";
static USER_PASSWORD: &'static str = "test";
static CORS_TRUSTED_ORIGIN: &'static str = "http://localhost:4000"; // Frontend app origin

pub enum 2FAType {
    Weak,
    Strong,
}

#[derive(Serialize, Deserialize)]
struct UserLogin {
    email: String,
    password: String,
    2FA_type: Option< 2FAType >,
}

/// Async HTTP server implementing all app logic
pub struct KeepmeCore {
    core : Arc<Core>, //
    pub config_dir : Path, //
    admin_account : identity::Account,
    pub BTC_wallet : wallets::BitcoinCore,
    pub client_bitmex : BitmexClient,
}

/// Class settings and constructors
impl KeepmeCore {

    ///Constructor
    pub fn new<P: AsRef<Path>>(path: &P) -> KeepmeCore {
        // Load config from given dir

        // Return unstance of perimeter core server
        KeepmeCore {
            config_dir : path,
            core : Arc::new(Core::new().unwrap()),
            admin_account : identity::Account::test(),
            BTC_wallet : wallets::BitcoinCoreWallet::new(),
        }
    }

    /// Helper function that reads request body into String
    fn read_to_string(mut req: Request) -> io::Result<String> {
        let mut s = String::new();
        try!(req.read_to_string(&mut s));
        Ok(s)
    }
}

/// Macro definitions for specification of API endpoints


/// Server request handling logic
impl Handler for KeepmeCore {

    /// Handle HTTP requests made to the server.
    fn handle(&self, mut req: Request, mut res: Response) {
        // Reactor from tokio-core
        println!("{:?}", req.uri);
        let mut core = Core::new().unwrap();

        // Headers
        let mut headers = Headers::new();
        headers.set( AccessControlAllowOrigin::Any ); // CORS
        headers.set( ContentType::json() );
        *res.headers_mut() = headers;

        // Authorize request
        //let auth_result = core.run().unwrap();

        // Deconstruct request before processing
        let mut request_body = String::new();
        req.read_to_string(&mut request_body).unwrap();
        println!("{:?}", request_body);

        // Match url
        *res.status_mut() = StatusCode::Ok;
        match req.uri {
            AbsolutePath(ref path) => match (&req.method, &path[..]) {
                (&Get, "/") => {
                    res.send(br#"{"status":{"name":"Keep.me API","version":"0.1.0"}}"#).unwrap();
                    return;
                },
                (&Get, "/account") => {
                    res.send(br#"{"account":{"name":"admin"}}"#).unwrap();
                    return;
                },
                (&Options, "/account") => {
                    res.headers_mut().set( AccessControlAllowOrigin::Any );
                    res.headers_mut().set(
                        AccessControlAllowHeaders(vec![
                            UniCase("accept".to_owned()),
                            UniCase("content-type".to_owned()),
                        ])
                    );
                    res.headers_mut().set(
                        AccessControlAllowMethods(vec![
                            Get,
                            Post,
                        ])
                    );
                    res.send(b"").unwrap();
                    return;
                },
                (&Options, "/api/auth/login") => {
                    res.headers_mut().set( AccessControlAllowOrigin::Value(CORS_TRUSTED_ORIGIN.to_owned()) );
                    res.headers_mut().set( AccessControlAllowCredentials );
                    res.headers_mut().set(
                        AccessControlAllowHeaders(vec![
                            UniCase("accept".to_owned()),
                            UniCase("content-type".to_owned()),
                        ])
                    );

                    res.headers_mut().set(
                        AccessControlAllowMethods(vec![
                            Get,
                            Post,
                        ])
                    );
                    res.send(b"").unwrap();
                    return;
                }
                (&Post, "/api/auth/login") => {
                    // Login request

                    // Accept a JSON string that corresponds to the User struct
                    let credentials: UserLogin = serde_json::from_str(&request_body).unwrap();

                    // Get the email and password
                    let email = credentials.email;
                    let password = credentials.password;

                    // Simple password checker
                    let mut response_body = String::new();
                    if (email == USER_EMAIL.to_string()) && (password == USER_PASSWORD.to_string()) {

                        let header: Header = Default::default();

                        // For the example, we just have one claim
                        // You would also want iss, exp, iat etc
                        let claims = Registered {
                            sub: Some(email.into()),
                            ..Default::default()
                        };

                        let token = Token::new(header, claims);

                        // Sign the token
                        let jwt = token.signed(AUTH_SECRET.as_bytes(), Sha256::new()).unwrap();
                        response_body = format!("{{\"token\": \"{}\"}}", jwt).to_string();

                    } else {
                        //Authorization failed
                        *res.status_mut() = StatusCode::Forbidden;
                        res.send(br#"{"error" : "Authorization failed."}"#);
                        return;
                    }

                    res.headers_mut().set( AccessControlAllowOrigin::Value(CORS_TRUSTED_ORIGIN.to_owned()) );
                    res.headers_mut().set( AccessControlAllowCredentials );
                    res.send(&response_body.as_bytes()).unwrap();
                    //res.send(br#"{status:"JWT login disabled"#).unwrap();
                    return;
                },
                _ => {
                    //Check for possible redirects
                    *res.status_mut() = StatusCode::NotFound;
                    return;
                }
                },
            _ => {
                *res.status_mut() = StatusCode::NotFound;
                return;
            }
        }
    }
}


// Server start logic
impl KeepmeCore {
    // Nothing fancy in here, we start an instance of our server with one reactor.
    pub fn start_server<P: AsRef<Path>>(addr: &str, path: P) {
        let server = Server::http(addr).unwrap().handle(PerimeterCore::new(&path)).unwrap();
        println!("Listening on http://{} with 1 thread.", addr);
    }
}
