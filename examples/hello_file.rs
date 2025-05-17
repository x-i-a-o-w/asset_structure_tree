// extern crate asset_structure_tree;

use asset_structure::*;
use std::fs;
fn main() {
    let _ = fs::create_dir("parent");
    let _ = fs::create_dir("parent/child");
    let _ = fs::create_dir("parent/child/childchild");
    let root = AssetStructure::new(fs::canonicalize(".").unwrap().join("parent"));
    assert_eq!(root.get_local("child").is_ok(), true);
    assert_eq!(root.get_global("child").is_ok(), true);
    assert_eq!(root.get_local("childchild").is_ok(), false);
    assert_eq!(root.get_global("childchild").is_ok(), true);
    fs::remove_dir_all("parent").unwrap();
}
