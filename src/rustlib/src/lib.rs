use std::ffi::CString;

#[macro_use]
extern crate rustr;
pub mod export;
pub use rustr::*;
use rustr::rptr::RPtr;

#[macro_use]
extern crate neo4j;
use neo4j::{Graph, Value};

// #[rustr_export]
pub fn open_graph_internal(uri: CString, username: CString, password: CString) -> RResult<RPtr<Graph>> {
    let username = if username.to_bytes() == &[] { None } else { Some(username.as_c_str()) };
    let password = if password.to_bytes() == &[] { None } else { Some(password.as_c_str()) };
    Graph::open(&uri, username, password).map(Box::new).map(RPtr::new)
}

// #[rustr_export]
pub fn query_graph_internal(graph: RPtr<Graph>, query: CString, params: Value) -> RResult<RList> {
    let mut graph = { graph };
    let graph = graph.get()?;
    let result_stream = graph.query(query, params)?;
    let nfields = result_stream.nfields();
    let mut fieldnames = CharVec::alloc(nfields as _);
    for i in 0..nfields {
        let s = result_stream.fieldname(i)?;
        let s = match s.to_str() {
            Ok(x) => x,
            Err(_) => stop!("Invalid UTF-8 in Neo4J field name: {:?}", s.to_bytes()),
        };
        fieldnames.set(i as _, s)?;
    }
    let results = result_stream.collect::<RResult<Vec<_>>>()?;
    let mut out = RList::alloc(nfields as _);
    out.set_name(&fieldnames)?;
    for y in 0..nfields {
        let mut data = RList::alloc(results.len());
        for (x, result) in results.iter().enumerate() {
            let field = result.index(y);
            data.set(x, field.intor()?)?;
        }
        out.set(y as _, data.intor()?)?;
    }
    out.as_data_frame()?;
    Ok(out)
}
