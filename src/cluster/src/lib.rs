//! Functionality for installing, managing, and deleting Fluvio clusters.
//!
//! The primary use of this crate is to install Fluvio clusters on
//! Kubernetes using a [`ClusterInstaller`], which provides a fluid
//! interface for cluster specification.
//!
//! # Example
//!
//! To install a basic Fluvio cluster, just do the following:
//!
//! ```no_run
//! use fluvio_cluster::ClusterInstaller;
//! let installer = ClusterInstaller::new().build().unwrap();
//! fluvio_future::task::run_block_on(installer.install_fluvio()).unwrap();
//! ```
//!
//! [`ClusterInstaller`]: ./struct.ClusterInstaller.html

#![warn(missing_docs)]

mod helm;
mod install;
mod error;
mod check;

pub use install::ClusterInstaller;
pub use install::ClusterInstallerBuilder;
pub use error::ClusterError;
pub use check::_check_load_balancer_status;

const VERSION: &str = include_str!("VERSION");
