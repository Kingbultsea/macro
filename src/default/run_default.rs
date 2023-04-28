use std::collections::HashMap;
use std::hash::Hash;

// 默认值
#[derive(Default, Debug)]
struct Person {
    name: String,
    age: u8,
}

// 该方法有借用问题
// fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
// where
//     K: Clone + Eq + Hash,
//     V: Default,
// {
       // 第一次借用
//     match map.get_mut(&key) {
//         Some(value) => value,
//         None => {
//             // 第二次借用 插入默认值 
//             map.insert(key.clone(), V::default());
//             map.get_mut(&key).unwrap()
//         }
//     }
// }

fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
where
    K: Clone + Eq + Hash,
    V: Default,
{
    map.entry(key).or_default()
}

pub fn run_default() {
    // let mut people = HashMap::new();
    let mut people = HashMap::<&str, Person, _>::new();

    // 使用 `get_default` 函数获取 `Person` 的可变引用
    let person:&mut Person = get_default(&mut people, "Alice");

    // 通过可变引用修改结构体实例的字段
    person.name = "Alice".to_string();
    person.age = 30;

    println!("{:?}", person);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 一定要约束b，不然默认和self一样为a
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
