mod tree
{
    extern crate core;
    use self::Tree::
    {
        Leaf,
        Node
    };

    pub enum Tree<T>
    {
        Leaf,
        Node
            (
                Box<Tree<T>>,
                T,
                Box<Tree<T>>
            )
    }

    //###########################################

    impl<T: Ord+core::fmt::Show> Tree<T>
    {
        pub fn new_tree() -> Tree<T>
        {
            Leaf
        }

        //#######################################

        pub fn insert_value(&mut self, unplaced: T)
        {
            match *self
            {
                Leaf    => *self = Node(box Leaf, unplaced, box Leaf),
                Node(ref mut lesser_branch, ref data, ref mut greater_branch)   => if unplaced < *data
                {
                    lesser_branch.insert_value(unplaced);
                }
                else if unplaced > *data
                {
                    greater_branch.insert_value(unplaced);
                }
            }
        }

        //#######################################

        pub fn print_in_order(&self)
        {
            match *self
            {
                Leaf                                                    => return,
                Node(ref lesser_branch, ref data, ref greater_branch)   =>
                {
                    lesser_branch.print_in_order();
                    print!("{}, ", *data);
                    greater_branch.print_in_order();
                }
            }
        }
    }
}

        //###########################################

fn main()
{
    let mut root: tree::Tree<uint> = tree::Tree::new_tree();

    let mut random;

    print!("Elements generated : ");
    for _ in range(1u, 11u)
    {
        random = std::rand::random::<uint>() % 100u;
        print!("{}, ", random);
        root.insert_value(random);
    }
    std::io::print("\n");

    print!("Sorted elements : ");
    root.print_in_order();
    println!("");
}

        //###########################################

//#[test]
//fn it_works()
/*fn main()
{
    let mut root = tree::Leaf::new_leaf::<uint>(50u);

    let mut random;

    print!("Elements generated : ");
    for _ in range(1u, 11u)
    {
        random = std::rand::random::<uint>() % 100u;
        print!("{}, ", random);
        root.add_leaf::<uint>(random);
    }
    std::io::print("\n");

    print!("In-order tree traversal : ");
    root.display_sorted_leaves();
    std::io::print("\n");

    println!("Depth of the tree : {} levels", root.count_levels());
    root.display_tree();
}

//#[allow(dead_code)]
mod tree
{
    pub enum Leaf<T>
    {
        Node (
            greater:    Box<Leaf<T>>>,
            lesser:     Option<Box<Leaf<T>>>,
            data:       T,
            ),
            None,
    }

    impl Leaf<uint>
    {
        pub fn new_leaf<Uint>(data: uint) -> Box<Leaf<uint>>
        {
            box Leaf
            {
                greater:    None,
                lesser:     None,
                data:       data
            }
        }

        //###########################################

        pub fn add_leaf<Uint>(&mut self, data: uint)
        {
            match  data >= self.data
            {
                true    =>
                {
                    match self.greater
                    {
                        Some(ref mut branch)    => branch.add_leaf::<uint>(data),
                        None                    => self.greater = Some(Leaf::new_leaf::<uint>(data)),
                    }
                },
                false   =>
                {
                    match self.lesser
                    {
                        Some(ref mut branch)    => branch.add_leaf::<uint>(data),
                        None                    => self.lesser = Some(Leaf::new_leaf::<uint>(data)),
                    }
                },
            }
        }

        //###########################################

        pub fn display_sorted_leaves(&self)
        {
            match self.lesser
            {
                Some(ref branch)    => branch.display_sorted_leaves(),
                None            => (),
            }

            print!("{}, ", self.data);

            match self.greater
            {
                Some(ref branch)    => branch.display_sorted_leaves(),
                None            => (),
            }
        }

        //###########################################

        pub fn display_tree(&self)
        {
            let depth = self.count_levels();

            for level in range(1u, depth + 1)
            {
                println!("{}| {:^30}", depth - level, self.format_level(depth - level, depth));
            }
        }

        //###########################################

        pub fn count_levels(&self) -> uint
        {
            let lesser = match self.lesser
            {
                None                => 1u,
                Some(ref branch)    => 1u + branch.count_levels(),
            };

            let greater = match self.greater
            {
                None                => 1u,
                Some(ref branch)    => 1u + branch.count_levels(),
            };

            if lesser > greater
            {
                lesser
            }
            else
            {
                greater
            }
        }

        //###########################################

        fn format_level(&self, level: uint, depth: uint) -> String
        {
            if level == 0
            {
                format!("{:<2}", self.data)
            }
            else
            {
                let lesser = match self.lesser
                {
                    Some(ref branch)    => branch.format_level(level - 1, depth),
                    None                => format!("      "),
                };

                let greater = match self.greater
                {
                    Some(ref branch)    => branch.format_level(level - 1, depth),
                    None                => format!("      "),
                };

                format!("{:<2}{}{:<2}", lesser, String::from_char((level) * 2u, ' '), greater)
            }
        }
    }
}*/
