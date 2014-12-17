#![allow(dead_code)]
#![feature(unboxed_closures)]
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

    impl<T: Ord+core::fmt::Show+Copy> Tree<T>
    {
        pub fn new() -> Tree<T>
        {
            Leaf
        }

        //#######################################

        pub fn insert_value(&mut self, unplaced: T)
        {
            match *self
            {
                Leaf                                    => *self = Node(box Leaf, unplaced, box Leaf),
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
                    print!(" {},", *data);
                    greater_branch.print_in_order();
                }
            }
        }

        //#######################################

        pub fn count_levels(&self) -> uint
        {
            match *self
            {
                Leaf                                            => 0,
                Node(ref lesser_branch, _, ref greater_branch)  =>
                {
                    let lesser_levels = 1 + lesser_branch.count_levels();
                    let greater_levels = 1 + greater_branch.count_levels();
                    if lesser_levels > greater_levels
                    {
                        lesser_levels
                    }
                    else
                    {
                        greater_levels
                    }
                }
            }
        }

        //#######################################

        pub fn format_levels(&self)
        {
            let max_level = self.count_levels();

            let size_of_element = self.estimated_size_of_printed_element();

            let size_of_level = ((1u << max_level) - 1u) * size_of_element;
            
            let greatermost_margin = (1 << (max_level - self.greatermost()) ) - 1u;
            
            let lessermost_margin = (1 << (max_level - self.lessermost()) ) - 1u;
            println!("Lessermost margin : {}. Greatermost_margin : {}.", lessermost_margin, greatermost_margin);

            let mut formatted_levels: Vec<String> =
                Vec::from_elem
                (
                    max_level, 
                    String::with_capacity(size_of_level)
                );
            self.fill_formatted_levels(1,
                                      max_level,
                                      lessermost_margin,
                                      size_of_element,
                                      &mut formatted_levels,
                                      1);

            let separator = String::from_char(size_of_level - (lessermost_margin + greatermost_margin) * size_of_element, '-');
            for level in formatted_levels.iter()
            {
                println!("{}", level);
                println!("{}", separator);
            }
        }

        //#######################################

        fn greatermost(&self) -> uint
        {
            match *self
            {
                Leaf                => 0,
                Node(_, _, ref g)   => 1 + g.greatermost()
            }
        }

        //#######################################

        fn lessermost(&self) -> uint
        {
            match *self
            {
                Leaf                => 0,
                Node(ref l, _, _)   => 1 + l.lessermost()
            }
        }

        //#######################################

        fn estimated_size_of_printed_element(&self) -> uint
        {
            (
                format!("{}",
                        self
                        .get_max()
                        .unwrap())
            ).len()
        }

        //#######################################

        fn fill_formatted_levels(&self,
                                current_level:      uint,
                                max_level:          uint,
                                lessermost:         uint,
                                size_of_element:    uint,
                                vector:             &mut Vec<String>,
                                position_in_level:  uint)
        {
            match *self
            {
                Leaf                                                    =>  return,
                Node(ref lesser_branch, ref data, ref greater_branch)   =>
                {
                    lesser_branch.fill_formatted_levels(current_level + 1,
                                                       max_level,
                                                       lessermost,
                                                       size_of_element,
                                                       vector,
                                                       position_in_level * 2 - 1);
                    {
                        let ref mut formatted_level = vector.as_mut_slice()[current_level - 1];

                        let len = formatted_level.len();
                        let margin = (1 << (max_level - current_level) ) - 1;
                        let inbetween_space = (margin << 1) + 2;
                        let to_current_node = inbetween_space * (position_in_level - 1);
                        let how_much_to_fill = (margin + to_current_node - lessermost) * size_of_element - len;

                        formatted_level.grow(how_much_to_fill, ' ');
                        formatted_level.push_str((format!("{}", *data)).as_slice());
                    }

                    greater_branch.fill_formatted_levels(current_level + 1,
                                                        max_level,
                                                        lessermost,
                                                        size_of_element,
                                                        vector,
                                                        position_in_level * 2);
                }
            }
        }

        //#######################################

        pub fn get_max(&self) -> Option<T>
        {
            match *self
            {
                Leaf                                    => None,
                Node(_, ref data, ref greater_branch)   => match greater_branch.get_max()
                {
                    Some(result)    => Some(result),
                    None            => Some(*data)
                }
            }
        }

        //#######################################

        pub fn get_min(&self) -> Option<T>
        {
            match *self
            {
                Leaf                                    => None,
                Node(ref lesser_branch, ref data, _)    => match lesser_branch.get_min()
                {
                    Some(result)    => Some(result),
                    None            => Some(*data)
                }
            }
        }
    }
}

        //###########################################

fn main()
{
    let mut root: tree::Tree<uint> = tree::Tree::new();

    let mut random;

    print!("Elements generated :");
    for _ in range(1u, 16u)
    {
        random = std::rand::random::<uint>() % 100u;
        print!(" {},", random);
        root.insert_value(random);
    }
    std::io::print("\n");

    print!("Sorted elements :");
    root.print_in_order();
    println!("\nNumber of level : {}", root.count_levels());
    println!("Max is : {}", match root.get_max()
             {
                 Some(value)    => format!("{}", value),
                 None           => format!("Error, the tree may have never been used.")
             }
            );
    println!("Min is : {}", match root.get_min()
             {
                 Some(value)    => format!("{}", value),
                 None           => format!("Error, the tree may have never been used.")
             }
            );
    root.format_levels();
}
