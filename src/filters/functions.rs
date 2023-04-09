use crate::fs_node::FSNode;

pub fn is_hidden(node: &FSNode) -> bool {
  node
    .path()
    .file_name()
    .map_or(false, |f| f.to_string_lossy().starts_with('.'))
}

pub const fn is_file(node: &FSNode) -> bool {
  matches!(*node, FSNode::File(_))
}

pub const fn is_symlink(node: &FSNode) -> bool {
  matches!(*node, FSNode::SymbolicLink(_))
}

pub fn filter_ext_exc(node: &FSNode, exts: &[&str]) -> bool {
  node
    .path()
    .extension()
    .map_or(false, |ext| exts.contains(&ext.to_string_lossy().as_ref()))
}