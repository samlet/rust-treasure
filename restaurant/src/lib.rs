/*
我们定义了一个公有结构体 back_of_house:Breakfast，其中有一个公有字段 toast 和私有字段 seasonal_fruit。
这个例子模拟的情况是，在一家餐馆中，顾客可以选择随餐附赠的面包类型，
但是厨师会根据季节和库存情况来决定随餐搭配的水果。餐馆可用的水果变化是很快的，所以顾客不能选择水果，
甚至无法看到他们将会得到什么水果。
 */
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

/*
https://kaisery.gitbooks.io/trpl-zh-cn/content/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
 */
