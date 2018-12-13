fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn demo_generic_func() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn demo_generic_struct() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// traits
// with default implementation
pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("default impl for summary");
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// use the default impl
impl Summary for Tweet {}

// item can be moved or referenced
pub fn notify(item: &impl Summary) {
    println!("borrow traits! {}", item.summarize());
}

fn demo_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("use traits interface: {}", tweet.summarize());
    notify(&tweet);
}

// life time
struct ImportantExcerpt<'a> {
    excerpt: &'a String,
}

impl<'b> ImportantExcerpt<'b> {
    fn level(&self) -> i32 {
        3
    }

    fn receipt(&self) -> &String {
        self.excerpt
    }
}

struct Gui<T: Copy> {
    x: T,
}

impl<T: Copy> Gui<T> {
    pub fn get_x(&self) -> T {
        self.x
    }
}

fn main() {
    demo_generic_func();
    demo_generic_struct();
    demo_trait();

    let r;
    {
        r = 5;
    }
    println!("r can be assigned if it's not initialized first {}", r);

    let gui = String::from("Guihao");
    let hao = ImportantExcerpt { excerpt: &gui };
    println!("ImportantExcerpt::level {}, {}", hao.level(), hao.receipt());

    let x: Gui<u32> = Gui { x: 10 };
    println!("struct with trait bounds {}", x.get_x());
}
