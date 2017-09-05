use std::ffi::CString;

#[macro_use]
extern crate rustr;
pub mod export;
pub use rustr::*;
use rustr::rptr::RPtr;

extern crate errno;

#[macro_use]
mod utils;

mod bindings {
	#![allow(dead_code)]
	#![allow(non_snake_case)]
	#![allow(non_camel_case_types)]
	#![allow(non_upper_case_globals)]
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

	include!("binding_extras.rs");
}

mod graph;
use graph::Graph;

// #[rustr_export]
pub fn open_graph_internal(uri: CString, username: CString, password: CString) -> RResult<RPtr<Graph>> {
	let username = if username.to_bytes() == &[] { None } else { Some(username.as_c_str()) };
	let password = if password.to_bytes() == &[] { None } else { Some(password.as_c_str()) };
	Graph::open(&uri, username, password).map(Box::new).map(RPtr::new)
}
