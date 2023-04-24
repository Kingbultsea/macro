fn lagest<T: PartialOrd>(list: &[T]) -> &T {
    let mut lagest_num = &list[0];

    for item in list.iter() {
        if item > lagest_num {
            lagest_num = item;
        }
    }

    lagest_num
}

fn lagest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut lagest_index = 0_usize;

    for i in 0..list.len() {
        if list[i] > list[lagest_index] {
            lagest_index = i;
        }
    }

    &list[lagest_index]
}

pub fn run_latest() {
    let list = vec![1,2,3,4];

    let latest_num = lagest(&list);

    let largest_num_2 = lagest2(&list);

    println!("最大数为： {}", latest_num);
    print!("最大数为：{}", largest_num_2);
}