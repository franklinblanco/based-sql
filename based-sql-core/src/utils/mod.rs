use std::{path::Path, net::Ipv4Addr};

/// For Internal use. Convenience function. 
/// TODO: return a result and return back that result to the user of this library?? 
pub fn as_datastore_file_path<'a>(path: &'a Path) -> String {
    let path_str: String = match path.to_str() {
        Some(path) => path,
        None => panic!("Invalid path supplied. Please make sure your path can go through std::path::Path.to_str()"),
    }.to_string();
    format!("file://{path_str}")
}

/// For Internal use. Convenience function. 
/// TODO: return a result and return back that result to the user of this library?? 
pub fn as_datastore_address<'a>(address: &Ipv4Addr) -> String {
    format!("tikv://{}", address.to_string())
}