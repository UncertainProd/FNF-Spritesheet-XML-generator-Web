pub mod growingpacker;

mod helpers {
    // Taken from https://gist.github.com/LPeter1997/f18c78de6cea1810bc755fe39ddfd77d
    // Rust version of the algorithm written by (@LPeter1997)[https://github.com/LPeter1997]
    // with a few modifications from myself

    // Note that the problem is the 2 dimensional version of the Bin Packing
    // problem, which is essentially NP-hard. The algorithm used is a best-effort
    // algorithm based on: https://codeincomplete.com/posts/bin-packing/.

    use std::collections::HashMap;
    use std::hash::Hash;
    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::algorithms::PackingRectangle;
    use crate::utils::PackError;

    /// The packer algorithm itself.
    pub fn bin_pack<
        // Heuristic function,
        T: Hash + Eq,
        FH: Fn(&PackingRectangle<T>) -> i32
    >(to_pack: impl Iterator<Item = PackingRectangle<T>>, heuristic_fn: FH) -> Result<PackOutput<T>, PackError> {
        let mut to_pack: Vec<_> = to_pack.collect();
        to_pack.sort_by(|a, b| heuristic_fn(b).cmp(&heuristic_fn(a)));

        let (w, h) = to_pack.first().map(|i| (i.width, i.height)).unwrap_or((0, 0));
        let mut packer = Packer::new(w, h);

        let mut items = HashMap::new();

        for e in to_pack {
            let (w, h) = (e.width, e.height);
            let k = e.id;
            let mut rect = packer.fit(w, h)?;
            rect.width = w;
            rect.height = h;
            items.insert(k, rect);
        }

        let width = packer.root.borrow().width;
        let height = packer.root.borrow().height;
        Ok(PackOutput{ width, height, items })
    }

    /// Returned by the packing operation to summate the results.
    pub struct PackOutput<K> {
        /// The required width to fit in every entry.
        width: u32,
        /// The required height to fit in every entry.
        height: u32,
        /// The map from the entry key to it's fit rectangle.
        pub items: HashMap<K, Rect>,
    }

    /// Represents a section in the packing that has been positioned.
    #[derive(Debug, Clone, Copy)]
    pub struct Rect {
        /// The x position of the upper-left corner of the rectangle.
        pub x: u32,
        /// The y position of the upper-left corner of the rectangle.
        pub y: u32,
        /// The width of the rectangle.
        pub width: u32,
        /// The height of the rectangle.
        pub height: u32,
    }

    impl <K> PackOutput<K> {
        /// Returns the required width to fit in every entry.
        pub fn width(&self) -> u32 { self.width }
        /// Returns the required height to fit in every entry.
        pub fn height(&self) -> u32 { self.height }
    }

    impl <'a, K> IntoIterator for &'a PackOutput<K> {
        type Item = (&'a K, &'a Rect);
        type IntoIter = std::collections::hash_map::Iter<'a, K, Rect>;

        fn into_iter(self) -> Self::IntoIter { self.items.iter() }
    }

    impl <K> IntoIterator for PackOutput<K> {
        type Item = (K, Rect);
        type IntoIter = std::collections::hash_map::IntoIter<K, Rect>;

        fn into_iter(self) -> Self::IntoIter { self.items.into_iter() }
    }

    /// The backing data-structure to the packing algorithm.
    struct Packer {
        /// The root node of the packer.
        root: Rc<RefCell<Node>>,
    }

    impl Packer {
        /// Creates an empty packer.
        fn new(w: u32, h: u32) -> Self {
            Self{
                root: Rc::new(RefCell::new(Node::new(0, 0, w, h))),
            }
        }

        /// Tries to fit in a block.
        fn fit(&mut self, w: u32, h: u32) -> Result<Rect, PackError> {
            let node = if let Some(node) = self.find_node(&self.root, w, h) {
                    self.split_node(&node, w, h)
                }
                else {
                    self.grow_node(w, h)?
                };
            let node = node.borrow();
            Ok(Rect{
                x: node.x,
                y: node.y,
                width: node.width,
                height: node.height,
            })
        }

        /// Finds the first fitting node, or none in the tree.
        fn find_node(&self, root: &Rc<RefCell<Node>>, w: u32, h: u32) -> Option<Rc<RefCell<Node>>> {
            let node = root.borrow();
            if node.occupied {
                let r = self.find_node(&node.right.as_ref().unwrap(), w, h);
                if r.is_some() {
                    return r;
                }
                self.find_node(&node.down.as_ref().unwrap(), w, h)
            }
            else if w <= node.width && h <= node.height {
                Some(root.clone())
            }
            else {
                None
            }
        }

        /// Splits and occupies the node.
        fn split_node(&mut self, node: &Rc<RefCell<Node>>, w: u32, h: u32) -> Rc<RefCell<Node>> {
            let mut bnode = node.borrow_mut();
            bnode.occupied = true;
            bnode.down = Some(Rc::new(RefCell::new(Node::new(bnode.x, bnode.y + h, bnode.width, bnode.height - h))));
            bnode.right = Some(Rc::new(RefCell::new(Node::new(bnode.x + w, bnode.y, bnode.width - w, h))));
            node.clone()
        }

        /// Grows the node in size and tries to remain close to a square.
        fn grow_node(&mut self, w: u32, h: u32) -> Result<Rc<RefCell<Node>>, PackError> {
            let root_w = self.root.borrow().width;
            let root_h = self.root.borrow().height;

            let can_down = w <= root_w;
            let can_right = h <= root_h;

            let should_right = can_right && (root_h > (root_w + w));
            let should_down = can_down && (root_w > (root_h + h));

            if should_right {
                Ok(self.grow_right(w, h))
            }
            else if should_down {
                Ok(self.grow_down(w, h))
            }
            else if can_right {
                Ok(self.grow_right(w, h))
            }
            else if can_down {
                Ok(self.grow_down(w, h))
            }
            else {
                // panic!("Invalid sorting!");
                Err(PackError)
            }
        }

        /// Grows a node to the right.
        fn grow_right(&mut self, w: u32, h: u32) -> Rc<RefCell<Node>> {
            let root_w = self.root.borrow().width;
            let root_h = self.root.borrow().height;

            let mut root = Node::new(0, 0, root_w + w, root_h);
            root.occupied = true;
            root.down = Some(self.root.clone());
            root.right = Some(Rc::new(RefCell::new(Node::new(root_w, 0, w, root_h))));
            self.root = Rc::new(RefCell::new(root));

            let node = self.find_node(&self.root, w, h).expect("Invalid sorting!");
            self.split_node(&node, w, h)
        }

        /// Grows a node to down.
        fn grow_down(&mut self, w: u32, h: u32) -> Rc<RefCell<Node>> {
            let root_w = self.root.borrow().width;
            let root_h = self.root.borrow().height;

            let mut root = Node::new(0, 0, root_w, root_h + h);
            root.occupied = true;
            root.right = Some(self.root.clone());
            root.down = Some(Rc::new(RefCell::new(Node::new(0, root_h, root_w, h))));
            self.root = Rc::new(RefCell::new(root));

            let node = self.find_node(&self.root, w, h).expect("Invalid sorting!");
            self.split_node(&node, w, h)
        }
    }

    /// A helper structure to represent a node in the packer.
    struct Node {
        /// Is this node occupied by other entries, or free to fill.
        occupied: bool,
        /// The x position of the upper-left corner of this node.
        x: u32,
        /// The y position of the upper-left corner of this node.
        y: u32,
        /// The width of this node.
        width: u32,
        /// The height of this node.
        height: u32,
        /// The node below this.
        down: Option<Rc<RefCell<Node>>>,
        /// The node right to this.
        right: Option<Rc<RefCell<Node>>>,
    }

    impl Node {
        /// Creates an empty node with the given upper-left corner and size.
        fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
            Self{
                occupied: false,
                x, y, width, height,
                down: None,
                right: None,
            }
        }
    }

}