use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::any::Any;

use rustr::*;
use value::ValueRef;
use bindings::*;
use errno::{errno, Errno};

pub struct QueryResult<'a> {
    inner: *mut neo4j_result_t,
    len: usize,
    phantom: PhantomData<&'a ()>,
}

impl<'a> QueryResult<'a> {
    fn from_c_ty(value: *mut neo4j_result_t, len: usize) -> QueryResult<'a> {
        unsafe {
            neo4j_retain(value);
        }
        QueryResult {
            inner: value,
            len: len,
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn index<'b: 'a>(&'b self, idx: u32) -> ValueRef<'b> {
        unsafe {
            ValueRef::from_c_ty(neo4j_result_field(self.inner, idx))
        }
    }
}

impl<'a> Drop for QueryResult<'a> {
    fn drop(&mut self) {
        unsafe {
            neo4j_release(self.inner);
        }
    }
}

#[allow(dead_code)] // for param_store Drop
pub struct ResultStream<'a> {
    pub(crate) inner: *mut neo4j_result_stream_t,
    param_store: Option<Box<Any>>,
    query: CString,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ResultStream<'a> {
    pub(crate) unsafe fn from_c_ty(value: *mut neo4j_result_stream_t, query: CString, store: Option<Box<Any>>) -> ResultStream<'a> {
        ResultStream {
            inner: value,
            phantom: PhantomData,
            query: query,
            param_store: store,
        }
    }

    pub fn nfields(&self) -> u32 {
        unsafe { neo4j_nfields(self.inner) }
    }

    pub fn fieldname(&self, i: u32) -> RResult<&CStr> {
        unsafe {
            if i > self.nfields() {
                stop!("Tried to get fieldname of nonexistant field")
            }
            let ptr = neo4j_fieldname(self.inner, i);
            if ptr.is_null() {
                stop!("Failed to get fieldname: {}", errno())
            }
            Ok(CStr::from_ptr(ptr))
        }
    }
}

impl<'a> Iterator for ResultStream<'a> {
    type Item = RResult<QueryResult<'a>>;

    fn next(&mut self) -> Option<RResult<QueryResult<'a>>> {
        unsafe {
            let res = neo4j_fetch_next(self.inner);
            if res.is_null() {
                let err = neo4j_check_failure(self.inner);
                if err != 0 {
                    if err == NEO4J_STATEMENT_EVALUATION_FAILED {
                        stop!("Neo4j statement evaluation failed: {}",
                            CStr::from_ptr((*neo4j_failure_details(self.inner)).message).to_string_lossy());
                    } else {
                        stop!("Neo4j query failed: {}", Errno(err));
                    }
                }
                return None;
            }
            Some(Ok(QueryResult::from_c_ty(res, neo4j_nfields(self.inner) as _)))
        }
    }
}
