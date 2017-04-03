//! Module that provides abstraction over wallet functionality, as well as inegration wrappers for
//! operating various 3rd party software wallets (e.g. BitcoinCore, mSigna, Mist, etc)

// Submodules
pub mod bitcoin;
pub mod ethereum;

// Reexport availible wallets
pub use self::bitcoin::BitcoinCore;

/// Trait that specifies functionality common for every cryptocurrency wallet
trait Wallet {
    //fn assets() -> {}
}
