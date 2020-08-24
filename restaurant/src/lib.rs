mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 指定的 super 起始的 server_order 路径，来调用 server_order 函数
        super::serve_order();
    }

    fn cook_order() {}
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/*
https://kaisery.gitbooks.io/trpl-zh-cn/content/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
 */
