use std::{fmt::Display, mem::{size_of, swap}};

// funciona como um pointer para a struct Node<T> (Tree::Empty ~ NULL)
enum Tree<K,V> {
    Empty,
    With(Box<Node<K,V>>),
}

struct Node<K,V> {
    key: K,
    value: V,
    ltree: Tree<K,V>,
    rtree: Tree<K,V>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Inserted {
    None,
    Leaf,
    Left,
    Right,
}

impl<K:Ord,V> Tree<K,V> {
    fn add(&mut self, key: K, value: V) -> (Inserted,Inserted) {
        use std::cmp::Ordering::*;
        if let Tree::With(ref mut node) = *self {
            let inserted = match key.cmp(&node.key) {
                Less => (Inserted::Left, node.ltree.add(key, value).0),
                Greater => (Inserted::Right, node.rtree.add(key, value).0),
                Equal => (Inserted::None, Inserted::None),
            };
            let diff = node.ltree.level_max() - node.rtree.level_max();
            if diff.abs() > 1 {
                match inserted {
                    (Inserted::Left, Inserted::Left) => self.rot_rsingle(),
                    (Inserted::Left, Inserted::Right) => self.rot_rdouble(),
                    (Inserted::Right, Inserted::Right) => self.rot_lsingle(),
                    (Inserted::Right, Inserted::Left) => self.rot_ldouble(),
                    _ => {},
                }
            }
            inserted
        } else {
            *self = Tree::With(Box::new(Node {
                key,
                value,
                ltree: Tree::Empty,
                rtree: Tree::Empty,
            }));
            (Inserted::Leaf,Inserted::None)
        }
    }
    fn get(&self, key: K) -> Option<&V> {
        use std::cmp::Ordering::*;
        if let Tree::With(ref node) = *self {
            match key.cmp(&node.key) {
                Equal => Some(&node.value),
                Less => node.ltree.get(key),
                Greater => node.rtree.get(key),
            }
        } else {
            None
        }
    }
    fn del(&mut self, key: K) -> Self {
        use std::cmp::Ordering::*;
        let mut result = Tree::Empty;
        if let Tree::With(ref mut node) = *self {
            match key.cmp(&node.key) {
                Equal => {
                    swap(self, &mut result);
                },
                Less => {
                    return node.ltree.del(key);
                },
                Greater => {
                    return node.rtree.del(key);
                },
            }
        }
        result
    }
    fn level_max(&self) -> isize {
        if let Tree::With(ref node) = *self {
            let l = node.ltree.level_max();
            let r = node.rtree.level_max();
            std::cmp::max(l, r) + 1
        } else {
            0
        }
    }
    fn level_min(&self) -> isize {
        if let Tree::With(ref node) = *self {
            let l = node.ltree.level_max();
            let r = node.rtree.level_max();
            std::cmp::min(l, r) + 1
        } else {
            0
        }
    }
    
    fn rot_lsingle(&mut self) {
        let mut a = Tree::Empty;
        let mut b = Tree::Empty;
        let mut c = Tree::Empty;
        swap(&mut a, self);
        if let Tree::With(ref mut anode) = a {
            swap(&mut b, &mut anode.rtree);
            if let Tree::With(ref mut bnode) = b {
                swap(&mut c, &mut bnode.ltree);
                anode.rtree = c;
                bnode.ltree = a;
                *self = b;
                return;
            }
            swap(&mut b, &mut anode.rtree);
        }
        swap(&mut a, self);
    }
    fn rot_rsingle(&mut self) {
        let mut a = Tree::Empty;
        let mut b = Tree::Empty;
        let mut c = Tree::Empty;
        swap(&mut a, self);
        if let Tree::With(ref mut anode) = a {
            swap(&mut b, &mut anode.ltree);
            if let Tree::With(ref mut bnode) = b {
                swap(&mut c, &mut bnode.rtree);
                anode.ltree = c;
                bnode.rtree = a;
                *self = b;
                return;
            }
            swap(&mut b, &mut anode.ltree);
        }
        swap(&mut a, self);
    }
    fn rot_ldouble(&mut self) {
        if let Tree::With(ref mut node) = *self {
            node.rtree.rot_rsingle();
            self.rot_lsingle();
        }
    }
    fn rot_rdouble(&mut self) {
        if let Tree::With(ref mut node) = *self {
            node.ltree.rot_lsingle();
            self.rot_rsingle();
        }
    }
}

impl<K:Display,V:Display> Tree<K,V> {
    fn print(&self) {
        self.print_deep(0);
    }
    fn print_deep(&self, level: usize) {
        if let Tree::With(ref node) = *self {
            for _ in 0..level {
                print!("    ");
            }
            println!("{}: {}", node.key, node.value);
            node.ltree.print_deep(level + 1);
            node.rtree.print_deep(level + 1);
        }
    }
}

use rand::{Rng, thread_rng};

fn main() {
    println!("tree: {} bytes", size_of::<Tree<String,String>>());
    println!("node: {} bytes", size_of::<Node<String,String>>());
    
    let mut tree = Tree::Empty;
    let mut rng = thread_rng();
    let mut vec = Vec::new();
    
    for _ in 0..100 {
        let r = rng.gen_range(-100..=100);
        tree.add(r, r);
        vec.push(r);
    }
    
    println!("{} | {}", tree.level_min(), tree.level_max());
    tree.print();
    
    for i in &vec {
        print!("{}, ", *i);
        assert_eq!(*i, *tree.get(*i).unwrap());
    }
    println!("ok!");
    
    for _ in 0..10 {
        let r = rng.gen_range(-100..=100);
        println!("deleting: {}", r);
        let d = tree.del(r);
        d.print();
    }
    /*
    println!("test!");
    
    let mut test = Tree::Empty;
    
    test.add(0, 0);
    test.add(-1, -1);
    test.add(4, 4);
    test.add(2, 2);
    test.add(6, 6);
    test.add(1, 1);
    test.add(3, 3);
    test.add(5, 5);
    test.add(7, 7);
    
    test.print();
    test.rot_rsingle();
    test.print();
    */
}
