//#[test]
//fn it_works()
fn main()
{
    let mut root = tree::Leaf
    {
        greater: None,
        lesser:  None,
        data:    "Green Leaf"
    };
    println!("{}", root.data);

    let yellow_leaf = box tree::Leaf {greater: None, lesser: None, data: "Yellow Leaf"};
    let blue_leaf = box tree::Leaf {greater: None, lesser: None, data: "Blue Leaf"};
    let black_leaf = box tree::Leaf {greater: None, lesser: None, data: "Black Leaf"};

    root.add_leaf(yellow_leaf);
    root.add_leaf(black_leaf);
    root.add_leaf(blue_leaf);
}

//#[allow(dead_code)]
mod tree
{
    pub struct Leaf
    {
        pub greater:    Option<Box<Leaf>>,
        pub lesser:     Option<Box<Leaf>>,
        pub data:       &'static str,
    }

    impl Leaf
    {
        pub fn add_leaf(&mut self, green: Box<Leaf>)
        {
            match  green.data >= self.data
            {
                true    =>
                {
                    match self.greater
                    {
                        Some(ref mut trunk) => trunk.add_leaf(green),
                        None        => self.greater = Some(green),
                    }
                },
                false   =>
                {
                    match self.lesser
                    {
                        Some(ref mut trunk) => trunk.add_leaf(green),
                        None        => self.lesser = Some(green),
                    }
                },
            }
        }
    }
}
