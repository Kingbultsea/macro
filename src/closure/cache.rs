struct Cacher<'a, T, V>
where
    T: Fn(u32) -> &'a V,
{
    query: T,
    value: Option<&'a V>,
}

impl<'a, T, V> Cacher<'a, T, V>
where
    T: Fn(u32) -> &'a V,
{
    fn new(query: T) -> Cacher<'a, T, V> {
        Cacher {
            query,
            value: None,
        }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: u32) -> &V {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(&v);
                v
            }
        }
    }
}

pub fn run_cache() {
    let defalut = String::from("aasdasda");
    let mut cache = Cacher{ query: |q: u32| {
        // todo 去查询数据库啥的
        &defalut
    }, value: None };

    println!("默认查询值{}", cache.value(1))
}