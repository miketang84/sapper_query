
extern crate sapper;
extern crate url;

use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use url::form_urlencoded;


use sapper::{Request, Result, Key};

pub type QueryMap = HashMap<String, Vec<String>>;

pub struct QueryParams;
impl Key for QueryParams { type Value = QueryMap; }

pub fn process(req: &mut Request) -> Result<()> {
    
    if req.query().is_none() {
        return Ok(());
    }
    
    let query_string = req.query().unwrap().to_owned();

    let query_iter = form_urlencoded::parse(query_string.as_bytes());

    let mut deduplicated: QueryMap = HashMap::new();
    for (key, val) in query_iter {
        match deduplicated.entry(key.into_owned()) {
            // Already a Vec here, push onto it
            Occupied(entry) => { entry.into_mut().push(val.into_owned()); },

            // No value, create a one-element Vec.
            Vacant(entry) => { entry.insert(vec![val.into_owned()]); },
        };
    }
    
    req.ext_mut().insert::<QueryParams>(deduplicated);

    
    Ok(())
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
