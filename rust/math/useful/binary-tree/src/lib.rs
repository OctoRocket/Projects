#[derive(Debug, PartialEq)]
struct BinaryTree {
    val: usize,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    fn new_node(
        val: usize,
        left: Option<BinaryTree>,
        right: Option<BinaryTree>,
    ) -> BinaryTree {
        let mut node = BinaryTree {
            val,
            left: None,
            right: None,
        };

        if let Some(left) = left {
            node.left = Some(Box::new(left));
        }
        if let Some(right) = right {
            node.right = Some(Box::new(right))
        }

        node
    }

    fn change_children(mut self, children: (Option<BinaryTree>, Option<BinaryTree>)) -> Self{
        if let Some(left) = children.0 {
            self.left = Some(Box::new(left));
        } else {
            self.left = None;
        }
        if let Some(right) = children.1 {
            self.right = Some(Box::new(right));
        } else {
            self.right = None;
        }

        self
    }

    fn construct_tree(mut nodes: Vec<BinaryTree>) -> Vec<BinaryTree> {
        match nodes.len() {
            1 => return nodes,
            2 => nodes[1] = nodes[1].change_children((Some(nodes[0]), None)),
            3 => nodes[1] = nodes[1].change_children((Some(nodes[0]), Some(nodes[2]))),
            _ => {
                let new_nodes = nodes[..].chunks_exact(4).map(|chunk| [chunk[1].change_children((Some(chunk[0]), Some(chunk[2]))), chunk[3]]);
                nodes = new_nodes.collect::<Vec<BinaryTree>>().append(new_nodes.remainder)
            }
        }

        nodes
    }

    pub fn new(input: Vec<usize>) -> BinaryTree {
        let binary_tree = input
            .iter()
            .map(
                |v| BinaryTree::new_node(
                    *v, 
                    None, 
                    None
                )
            )
            .collect::<Vec<BinaryTree>>();
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_node() {
        assert_eq!(
            BinaryTree::new_node(
                1,
                Some(BinaryTree::new_node(2, None, None)),
                Some(BinaryTree::new_node(3, None, None))
            ),
            BinaryTree{
                val: 1,
                left: Some(Box::new(BinaryTree {
                    val: 2,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTree {
                    val: 3,
                    left: None,
                    right: None,
                }))}
            )
    }
}
