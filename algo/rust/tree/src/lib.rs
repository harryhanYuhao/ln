#[macro_use]
extern crate downcast_rs;
use downcast_rs::DowncastSync;

use rand::prelude::*;
use std::fmt;

use std::sync::Arc;

pub trait Tree: fmt::Debug + fmt::Display + DowncastSync {
    fn rand(level: usize) -> Arc<dyn Tree>
    where
        Self: Sized;

    fn to_nice_str(&self) -> String;
}
impl_downcast!(sync Tree);

#[derive(Debug)]
pub struct BinaryTree {
    pub value: Option<i64>,
    pub left: Option<Arc<BinaryTree>>,
    pub right: Option<Arc<BinaryTree>>,
}

impl fmt::Display for BinaryTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BinaryTree {{ value: {:?}, left: {:?}, right: {:?} }}",
            self.value, self.left, self.right
        )
    }
}

impl Tree for BinaryTree {
    fn rand(level: usize) -> Arc<dyn Tree> {
        if level == 0 {
            return Arc::new(BinaryTree {
                value: Some(random::<i64>() % 100),
                left: None,
                right: None,
            });
        }

        fn generate_random(level: usize) -> Option<Arc<BinaryTree>> {
            let res: Option<Arc<BinaryTree>>;
            if (random::<usize>() % 100) <= 10 {
                res = None;
            } else {
                res = Some(BinaryTree::rand(level).downcast_arc().unwrap());
            }
            res
        }

        let left = generate_random(level - 1);
        let right = generate_random(level - 1);

        Arc::new(BinaryTree {
            value: Some(random::<i64>() % 100),
            left,
            right,
        })
    }

    fn to_nice_str(&self) -> String {
        fn auxiliary(node: &BinaryTree) -> Vec<String> {
            let mut res: Vec<String> = Vec::new();

            match node.value {
                Some(v) => {
                    res.push(format!("({})", v.to_string()));
                }
                None => res.push(String::from("NaN")),
            }

            if node.left.is_some() {
                let left_strs = auxiliary(node.left.as_ref().unwrap());
                for i in 0..left_strs.len() {
                    let mut tmp;
                    if i == 0 {
                        tmp = String::from(" |-");
                    } else {
                        if node.right.is_some() {
                            tmp = String::from(" | ");
                        } else {
                            tmp = String::from("   ");
                        }
                    }
                    tmp.push_str(&left_strs[i]);
                    res.push(tmp);
                }
            }
            if node.right.is_some() {
                let right_strs = auxiliary(node.right.as_ref().unwrap());
                for i in 0..right_strs.len() {
                    let mut tmp;
                    if i == 0 {
                        tmp = String::from(" |-");
                    } else {
                        tmp = String::from("   ");
                    }
                    tmp.push_str(&right_strs[i]);
                    res.push(tmp);
                }
            }
            res
        }
        let strings = auxiliary(self);

        let mut res: String = String::new();
        for i in strings.iter() {
            res.push_str(i);
            res.push_str(&String::from("\n"));
        }
        if res.len() > 0 {
            res.pop(); // remove last newline
        }
        res
    }
}
