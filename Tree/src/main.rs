//#[test]
//fn it_works()
fn main()
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
    pub struct Leaf<T>
    {
        pub greater:    Option<Box<Leaf<T>>>,
        pub lesser:     Option<Box<Leaf<T>>>,
        pub data:       T,
    }

    impl Leaf<uint>
    {
        pub fn new_leaf<uint>(data: uint) -> Box<Leaf<uint>>
        {
            box Leaf
            {
                greater:    None,
                lesser:     None,
                data:       data
            }
        }

        //###########################################

        pub fn add_leaf<uint>(&mut self, data: uint)
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
            let mut level;

            for depth in range(self.count_levels() - 1u, 0u)
            {
                println!("Depth : {}", depth);
                level = self.get_level(depth);
                match level
                {
                    Some(to_print)  => println!("{}| {}", depth, to_print),
                    None            => break,
                }
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

        fn get_level(&self, depth: uint) -> Option<String>
        {
            if depth == 0
            {
                Some(format!("{}", self.data))
            }
            else
            {
                let lesser = match self.lesser
                {
                    Some(ref branch)    => branch.get_level(depth - 1),
                    None                => None,
                };

                let greater = match self.greater
                {
                    Some(ref branch)    => branch.get_level(depth - 1),
                    None                => None,
                };

                match (lesser, greater)
                {
                    (Some(s1), Some(s2))    => Some(format!("({}|{})", s1, s2)),
                    (Some(s), None)         => Some(format!("({}|)", s)),
                    (None, Some(s))         => Some(format!("(|{})", s)),
                    (None, None)            => None,
                }
            }
        }
    }
}
