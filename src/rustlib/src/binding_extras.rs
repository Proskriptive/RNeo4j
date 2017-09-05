use std::ffi::CString;

use rustr::*;

// Defined as a macro
pub fn neo4j_type(value: neo4j_value_t) -> neo4j_type_t {
    return value._type;
}

impl IntoR for neo4j_value_t {
	fn intor(&self) -> RResult<SEXP> {
        unsafe {
            let ty = neo4j_type(*self);
            if ty == NEO4J_NULL {
                Ok(rstatic::rnull())
            } else if ty == NEO4J_BOOL {
                neo4j_bool_value(*self).intor()
            } else if ty == NEO4J_INT {
                neo4j_int_value(*self).intor()
            } else if ty == NEO4J_FLOAT {
                neo4j_float_value(*self).intor()
            } else if ty == NEO4J_STRING {
                let len = neo4j_string_length(*self) as usize + 1;
                let mut buf = vec![0u8; len];
                neo4j_string_value(*self, buf.as_mut_ptr() as *mut i8, len);
                CString::from_vec_unchecked(buf).intor()
            } else {
                stop!("Cannot convert Neo4j type to R type: {}", ty);
            }
        }
	}
}
