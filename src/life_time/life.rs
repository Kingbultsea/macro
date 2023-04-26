fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

pub fn run_life() {
    let first = String::from("a");
    let second = String::from("a");
    let longest_num = longest(first.as_str(), second.as_str());

    println!("最大值为{}", longest_num);

    life_test();
}

pub fn life_test() {
    let string1 = String::from("a");

    {
        let string2 =  String::from("a");
        let longest_num = longest(string1.as_str(), string2.as_str());
        // longest_num的生命周期 和 string2一样，取最短

        println!("最大值为 2: {}", longest_num);
    }
}
