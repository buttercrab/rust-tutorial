#![allow(unused)]

use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::{Rc, Weak};

/// Node struct of tree
struct Node<T>
where
    T: Ord,
{
    value: T,
    parent: Option<Weak<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            parent: None,
            left: None,
            right: None,
        }))
    }

    fn with_parent(value: T, parent: Weak<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            parent: Some(parent),
            left: None,
            right: None,
        }))
    }

    /// Minimum node starting from cursor
    fn min_node(_cursor: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        todo!()
    }

    /// Upgraded parent node.
    /// `None` if the node has no parent.
    #[inline]
    fn parent(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.parent.as_ref().and_then(|p| p.upgrade())
    }

    /// Check if the node is a left child.
    /// `None` if the node has no parent.
    #[inline]
    fn is_left_child(&self) -> Option<bool> {
        self.parent().and_then(|p| {
            p.deref()
                .borrow()
                .left
                .as_ref()
                .map(|v| v.deref().borrow().value == self.value)
        })
    }
}

impl<T> Debug for Node<T>
where
    T: Ord + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("Node");
        builder.field("value", &self.value);
        builder.field("left", &self.left);
        builder.field("right", &self.right);
        builder.finish()
    }
}

impl<T> Clone for Node<T>
where
    T: Ord + Clone,
{
    fn clone(&self) -> Self {
        Node {
            value: self.value.clone(),
            parent: self.parent.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

/// Binary Search Tree
///
/// ## Example
///
/// ```
/// use a_binary_tree::tree::Tree;
///
/// let tree = Tree::new();
/// ```
pub struct Tree<T>
where
    T: Ord,
{
    root: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T: Ord> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Tree<T>
where
    T: Ord,
{
    pub fn new() -> Tree<T> {
        Tree { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Check if the tree contains the value.
    pub fn contains(&self, _value: &T) -> bool {
        if let Some(_root) = self.root.clone() {
            todo!()
        } else {
            false
        }
    }

    /// Insert the value into the tree.
    /// If there was no equal value in the tree, it returns `true`.
    /// Otherwise, it returns `false`.
    pub fn insert(&mut self, value: T) -> bool {
        self.len += 1;
        if let Some(_root) = self.root.clone() {
            todo!()
        } else {
            self.root = Some(Node::new(value));
            true
        }
    }

    /// Remove the value from the tree.
    /// If there is an equal value in the tree, it returns `true`.
    /// Otherwise, it returns `false`.
    pub fn remove(&mut self, _value: &T) -> bool {
        let res = if let Some(_root) = self.root.clone() {
            todo!()
        } else {
            false
        };

        self.len -= res as usize;
        res
    }

    /// Iterator Generator
    pub fn iter(&self) -> Iter<T> {
        Iter {
            node: self.root.clone().map(|r| Node::min_node(r)),
            len: self.len,
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Tree<T>
where
    T: Ord + Clone,
{
    fn clone(&self) -> Self {
        Tree {
            root: self.root.clone(),
            len: self.len,
        }
    }
}

pub struct Iter<'a, T>
where
    T: Ord,
{
    node: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.clone() {
            // Need an unbound lifetime to get 'a
            let _res = unsafe { &*(&node.deref().borrow().value as *const T) };
            self.len -= 1;

            todo!()

            // uncomment this
            // Some(res)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

#[cfg(test)]
mod test {
    use crate::tree::Tree;

    #[test]
    fn insert_test() {
        let mut tree = Tree::new();

        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);

        assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec![1, 3, 5, 7]);
    }

    #[test]
    fn remove_test() {
        let mut tree = Tree::new();

        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.remove(&7);

        assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec![1, 3, 5]);
    }
}
