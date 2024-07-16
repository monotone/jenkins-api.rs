#![allow(dead_code)]
#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs,
    unused_extern_crates,
    unused_qualifications,
    unused_results
)]
#![allow(clippy::type_repetition_in_bounds)]

//! Bindings to [Jenkins JSON API](https://wiki.jenkins.io/display/JENKINS/Remote+access+API)
//!
//! # Example
//!
//! ```rust
//!
//! extern crate jenkins_api;
//!
//! use jenkins_api::JenkinsBuilder;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let jenkins = JenkinsBuilder::new("http://localhost:8080")
//!         .with_user("user", Some("password"))
//!         .build()?;
//!
//!     let job = jenkins.get_job("job name")?;
//!     let build = job.last_build.as_ref().unwrap().get_full_build(&jenkins)?;
//!
//!     println!(
//!         "last build for job {} at {} was {:?}",
//!         job.name, build.timestamp, build.result
//!     );
//!     Ok(())
//! }
//! ```
//!

mod client_internals;
pub use crate::client_internals::{Jenkins, JenkinsBuilder};
pub mod client;

#[macro_use]
pub mod helpers;

pub mod action;
pub mod build;
pub mod changeset;
pub mod home;
pub mod job;
pub mod nodes;
pub mod property;
pub mod queue;
pub mod scm;
pub mod user;
pub mod view;
