
extern crate sappers;
extern crate url;

use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use url::form_urlencoded;


use sappers::{Request, Result, Key};

pub type QueryMap = HashMap<String, Vec<String>>;

pub struct QueryParams;
impl Key for QueryParams { type Value = QueryMap; }

pub fn process(req: &mut Request) -> Result<()> {
    
    let query_string = req.query_string().clone();
    match query_string {
        Some(ref query_string) => {
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
            
            req.get_ext_mut().insert::<QueryParams>(deduplicated);
        },
        None => {
            // do nothing
        }
    }
    
    
    
    Ok(())
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
