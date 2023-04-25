/* Built in imports */
use core::cmp;
/* Crate imports */
use crate::fs_node::{FSNode, FSNodeRes};

// TODO: write tests for these functions

pub fn name(node1: &FSNodeRes, node2: &FSNodeRes) -> cmp::Ordering {
  let path1 = node1.as_ref().ok().map(FSNode::path);
  let path2 = node2.as_ref().ok().map(FSNode::path);
  path1.cmp(&path2)
}

pub fn size(node1: &FSNodeRes, node2: &FSNodeRes) -> cmp::Ordering {
  let size1 = node1.as_ref().ok().map(FSNode::size);
  let size2 = node2.as_ref().ok().map(FSNode::size);
  size1.cmp(&size2)
}

pub fn extension(node1: &FSNodeRes, node2: &FSNodeRes) -> cmp::Ordering {
  let ext1 = node1
    .as_ref()
    .map_or_else(|_| None, |n| n.path().extension());
  let ext2 = node2
    .as_ref()
    .map_or_else(|_| None, |n| n.path().extension());
  ext1.cmp(&ext2)
}

pub fn modified_date(node1: &FSNodeRes, node2: &FSNodeRes) -> cmp::Ordering {
  let date1 = node1.as_ref().ok().map(FSNode::modified_date);
  let date2 = node2.as_ref().ok().map(FSNode::modified_date);
  date1.cmp(&date2)
}
