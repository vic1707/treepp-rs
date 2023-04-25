/* Built in imports */
use core::cmp;
/* Crate imports */
use crate::fs_node::FSNodeRes;

// TODO: write tests for these functions

pub fn name(node1: &FSNodeRes, node2: &FSNodeRes) -> cmp::Ordering {
  let path1 = node1.as_ref().map_or_else(|_| None, |n| Some(n.path()));
  let path2 = node2.as_ref().map_or_else(|_| None, |n| Some(n.path()));
  match (path1, path2) {
    (Some(p1), Some(p2)) => p1.cmp(p2),
    (Some(_), None) => cmp::Ordering::Less,
    (None, Some(_)) => cmp::Ordering::Greater,
    (None, None) => cmp::Ordering::Equal,
  }
}

pub fn size(node1: &FSNodeRes, node2: &FSNodeRes) -> cmp::Ordering {
  let size1 = node1.as_ref().map_or_else(|_| None, |n| Some(n.size()));
  let size2 = node2.as_ref().map_or_else(|_| None, |n| Some(n.size()));
  match (size1, size2) {
    (Some(s1), Some(s2)) => s1.cmp(s2),
    (Some(_), None) => cmp::Ordering::Less,
    (None, Some(_)) => cmp::Ordering::Greater,
    (None, None) => cmp::Ordering::Equal,
  }
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

pub fn modified_date(node1: &FSNodeRes, node2: &FSNodeRes) -> cmp::Ordering {
  let date1 = node1
    .as_ref()
    .map_or_else(|_| None, |n| Some(n.modified_date()));
  let date2 = node2
    .as_ref()
    .map_or_else(|_| None, |n| Some(n.modified_date()));
  match (date1, date2) {
    (Some(d1), Some(d2)) => d1.cmp(d2),
    (Some(_), None) => cmp::Ordering::Less,
    (None, Some(_)) => cmp::Ordering::Greater,
    (None, None) => cmp::Ordering::Equal,
  }
}
