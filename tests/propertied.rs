#[macro_use]
extern crate proptest;
#[macro_use]
extern crate spectral;

extern crate env_logger;
#[macro_use]
extern crate serde_derive;

extern crate jenkins_api;

use spectral::prelude::*;

use jenkins_api::build::Build;
use jenkins_api::job::Job;
use jenkins_api::JenkinsBuilder;
use std::{thread, time};

use std::sync::{Once, ONCE_INIT};

static INIT: Once = ONCE_INIT;

fn setup() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

static JENKINS_URL: &'static str = "http://localhost:8080";

proptest! {
    #[test]
    fn doesnt_crash_user(ref s in "\\PC*") {
        setup();
        let jenkins = JenkinsBuilder::new(JENKINS_URL)
            .with_user(&s, Some("password"))
            .build()
            .unwrap();
        jenkins.get_home();
    }
}

proptest! {
    #[test]
    fn doesnt_crash_url(ref s in "\\PC*") {
        setup();
        if let Ok(jenkins) = JenkinsBuilder::new(&s)
            .with_user("user", Some("password"))
            .build()
        {
            jenkins.get_home().unwrap();
        }
    }
}

proptest! {
    #[test]
    fn doesnt_crash_job_name(ref s in "\\PC*") {
        setup();
        let jenkins = JenkinsBuilder::new(JENKINS_URL)
            .with_user("user", Some("password"))
            .build()
            .unwrap();
        jenkins.get_job(s);
    }
}
