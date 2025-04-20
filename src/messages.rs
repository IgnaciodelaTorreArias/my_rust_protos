pub(crate) mod my_package {
    include!(concat!(env!("OUT_DIR"), "/my_package.messages.rs"));
}
