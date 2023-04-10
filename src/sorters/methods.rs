use core::cmp;
use std::path::PathBuf;

use crate::fs_node::{FSNode, FSNodeRes};

pub fn name(node: &FSNodeRes) -> PathBuf {
  node
    .as_ref()
    .map_or_else(|_| PathBuf::new(), |n| n.path().clone())
}

pub fn size(node: &FSNodeRes) -> i128 {
  node.as_ref().map_or_else(|_| 0, FSNode::size)
}

pub fn extension(node1: &FSNodeRes, node2: &FSNodeRes) -> cmp::Ordering {
  let ext1 = node1
    .as_ref()
    .map_or_else(|_| None, |n| n.path().extension());
  let ext2 = node2
    .as_ref()
    .map_or_else(|_| None, |n| n.path().extension());
  match (ext1, ext2) {
    (Some(e1), Some(e2)) => e1.cmp(e2),
    (Some(_), None) => cmp::Ordering::Less,
    (None, Some(_)) => cmp::Ordering::Greater,
    (None, None) => cmp::Ordering::Equal,
  }
}
