#![cfg(test)]

use sealed_test::prelude::*;

use crate::{BinaryGrabber, BinaryManifest};

#[sealed_test]
fn test_sealed_test() {
    pretty_env_logger::init();

    let manifest = BinaryManifest::tailwind();

    let grabber = BinaryGrabber::new(manifest, []);
    grabber.run().unwrap();
}
