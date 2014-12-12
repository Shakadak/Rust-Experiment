//#[test]
//fn it_works()
fn main()
{
    let mut root = tree::Leaf::new_leaf::<uint>(50u);

    let mut random;

    for _ in range(1u, 11u)
    {
        random = std::rand::random::<uint>() % 100u;
        print!("{}, ", random);
        root.add_leaf::<uint>(random);
    }
    std::io::print("\n");

    root.display_sorted_leaves();
    std::io::print("\n");
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

        pub fn add_leaf<T>(&mut self, data: uint)
        {
            match  data >= self.data
            {
                true    =>
                {
                    match self.greater
                    {
                        Some(ref mut branch)    => branch.add_leaf::<T>(data),
                        None                    => self.greater = Some(Leaf::new_leaf::<T>(data)),
                    }
                },
                false   =>
                {
                    match self.lesser
                    {
                        Some(ref mut branch)    => branch.add_leaf::<T>(data),
                        None                    => self.lesser = Some(Leaf::new_leaf::<T>(data)),
                    }
                },
            }
        }

        pub fn display_sorted_leaves(self)
        {
            match self.lesser
            {
                Some(branch)    => branch.display_sorted_leaves(),
                None            => (),
            }

            print!("{}, ", self.data);

            match self.greater
            {
                Some(branch)    => branch.display_sorted_leaves(),
                None            => (),
            }
        }
    }
}
