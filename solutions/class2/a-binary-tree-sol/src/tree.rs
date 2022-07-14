use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem;
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
    fn min_node(mut cursor: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        loop {
            let left = cursor.deref().borrow().left.clone();

            if let Some(left) = left {
                cursor = left;
            } else {
                break cursor;
            }
        }
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
/// let tree = Tree::new();
///
///
/// ```
struct Tree<T>
where
    T: Ord,
{
    root: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
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

    /// Check if the tree contains the value.
    pub fn contains(&self, value: &T) -> bool {
        if let Some(root) = self.root.clone() {
            let mut cursor = root;

            loop {
                let child = match value.cmp(&cursor.deref().borrow().value) {
                    Ordering::Less => cursor.deref().borrow().left.clone(),
                    Ordering::Greater => cursor.deref().borrow().right.clone(),
                    Ordering::Equal => break true,
                };

                if let Some(child) = child {
                    cursor = child;
                } else {
                    break false;
                }
            }
        } else {
            false
        }
    }

    /// Insert the value into the tree.
    /// If there was no equal value in the tree, it returns `true`.
    /// Otherwise, it returns `false`.
    pub fn insert(&mut self, value: T) -> bool {
        self.len += 1;
        if let Some(root) = self.root.clone() {
            let mut cursor = root;

            loop {
                let mut c = cursor.deref().borrow_mut();
                let child = match value.cmp(&c.value) {
                    Ordering::Less => &mut c.left,
                    Ordering::Greater => &mut c.right,
                    Ordering::Equal => break false,
                };

                if let Some(child) = child {
                    let child = child.clone();
                    drop(c);
                    cursor = child;
                } else {
                    child.replace(Node::with_parent(value, Rc::downgrade(&cursor)));
                    break true;
                }
            }
        } else {
            self.root = Some(Node::new(value));
            true
        }
    }

    /// Remove the value from the tree.
    /// If there is an equal value in the tree, it returns `true`.
    /// Otherwise, it returns `false`.
    pub fn remove(&mut self, value: &T) -> bool {
        let res = if let Some(root) = self.root.clone() {
            let mut cursor = root;

            loop {
                let mut c = cursor.deref().borrow_mut();
                let child = match value.cmp(&c.value) {
                    Ordering::Less => c.left.clone(),
                    Ordering::Greater => c.right.clone(),
                    Ordering::Equal => {
                        if let Some(right) = c.right.clone() {
                            let mut right = right;

                            loop {
                                let r = right.deref().borrow().right.clone();

                                if let Some(r) = r {
                                    right = r;
                                } else {
                                    break;
                                }
                            }

                            mem::swap(&mut c.value, &mut right.deref().borrow_mut().value);
                            drop(c);
                            cursor = right;
                        } else {
                            drop(c);
                        }

                        let mut c = cursor.deref().borrow_mut();

                        if let Some(parent) = c.parent() {
                            let l = c.left.take();
                            if c.is_left_child().unwrap() {
                                parent.deref().borrow_mut().left = l.clone();
                            } else {
                                parent.deref().borrow_mut().right = l.clone();
                            }

                            if let Some(l) = l {
                                l.deref()
                                    .borrow_mut()
                                    .parent
                                    .replace(Rc::downgrade(&parent));
                            }
                        } else {
                            self.root = cursor.deref().borrow().left.clone();
                        }
                        break true;
                    }
                };

                if let Some(child) = child {
                    drop(c);
                    cursor = child;
                } else {
                    break false;
                }
            }
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

struct Iter<'a, T>
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
            let res = unsafe { &*(&node.deref().borrow().value as *const T) };
            let mut cursor = node;
            self.len -= 1;

            let right = cursor.deref().borrow().right.clone();
            if let Some(right) = right {
                self.node.replace(Node::min_node(right));
            } else {
                loop {
                    let l = cursor.deref().borrow().is_left_child();
                    if let Some(false) = l {
                        let p = cursor.deref().borrow().parent().unwrap();
                        cursor = p;
                    } else {
                        break;
                    }
                }

                if let Some(true) = cursor.deref().borrow().is_left_child() {
                    self.node.replace(cursor.deref().borrow().parent().unwrap());
                } else {
                    self.node = None;
                }
            }

            Some(res)
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
