use std::io::{self, BufRead};

struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    priority: i32,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut root: Option<Box<Node>> = None;

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        match parts[0] {
            "insert" => {
                let key: i32 = parts[1].parse().unwrap();
                let priority: i32 = parts[2].parse().unwrap();
                root = insert(root, key, priority);
            }
            "find" => {
                let key: i32 = parts[1].parse().unwrap();
                if find(&root, key) {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
            "delete" => {
                let key: i32 = parts[1].parse().unwrap();
                root = delete(root, key);
            }
            "print" => {
                inorder(&root);
                println!();
                preorder(&root);
                println!();
            }
            _ => {}
        }
    }
}

fn right_rotate(mut node: Box<Node>) -> Box<Node> {
    let mut left = node.left.take().unwrap();
    node.left = left.right.take();
    left.right = Some(node);
    left
}

fn left_rotate(mut node: Box<Node>) -> Box<Node> {
    let mut right = node.right.take().unwrap();
    node.right = right.left.take();
    right.left = Some(node);
    right
}

fn insert(t: Option<Box<Node>>, key: i32, priority: i32) -> Option<Box<Node>> {
    match t {
        None => Some(Box::new(Node { key, priority, left: None, right: None })),
        Some(mut node) => {
            if key == node.key {
                return Some(node);
            }
            if key < node.key {
                node.left = insert(node.left, key, priority);
                if let Some(ref left) = node.left {
                    if node.priority < left.priority {
                        return Some(right_rotate(node));
                    }
                }
            } else {
                node.right = insert(node.right, key, priority);
                if let Some(ref right) = node.right {
                    if node.priority < right.priority {
                        return Some(left_rotate(node));
                    }
                }
            }
            Some(node)
        }
    }
}

fn find(t: &Option<Box<Node>>, key: i32) -> bool {
    let mut cur = t;
    while let Some(node) = cur {
        if key == node.key {
            return true;
        } else if key < node.key {
            cur = &node.left;
        } else {
            cur = &node.right;
        }
    }
    false
}

fn delete(t: Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
    match t {
        None => None,
        Some(mut node) => {
            if key < node.key {
                node.left = delete(node.left, key);
                Some(node)
            } else if key > node.key {
                node.right = delete(node.right, key);
                Some(node)
            } else {
                delete_node(node, key)
            }
        }
    }
}

fn delete_node(mut node: Box<Node>, key: i32) -> Option<Box<Node>> {
    match (node.left.is_none(), node.right.is_none()) {
        (true, true) => None,
        (true, false) => {
            let mut rotated = left_rotate(node);
            rotated.left = delete(rotated.left.take(), key);
            Some(rotated)
        }
        (false, true) => {
            let mut rotated = right_rotate(node);
            rotated.right = delete(rotated.right.take(), key);
            Some(rotated)
        }
        (false, false) => {
            if node.left.as_ref().unwrap().priority > node.right.as_ref().unwrap().priority {
                let mut rotated = right_rotate(node);
                rotated.right = delete(rotated.right.take(), key);
                Some(rotated)
            } else {
                let mut rotated = left_rotate(node);
                rotated.left = delete(rotated.left.take(), key);
                Some(rotated)
            }
        }
    }
}

fn inorder(t: &Option<Box<Node>>) {
    if let Some(node) = t {
        inorder(&node.left);
        print!(" {}", node.key);
        inorder(&node.right);
    }
}

fn preorder(t: &Option<Box<Node>>) {
    if let Some(node) = t {
        print!(" {}", node.key);
        preorder(&node.left);
        preorder(&node.right);
    }
}
