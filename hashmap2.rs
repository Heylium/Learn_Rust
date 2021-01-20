use std::collections::HashMap;

pub fn main() {
    let mut scores=HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),30);

    let team_name=String::from("Blue");
    let score=scores.get(&team_name);   //使用get()方法获取hashmap的内容（通过键获得值），最好使用引用&传递，get()返回的是一个迭代器，所以最好用match方法处理返回的迭代器

    match score {
        Some(s)=>println!("{}",s),
        None=>println!("team not exist!"),
    };

    for(key,value)in &scores{   //for循环即可遍历hashmap，（key，value）使用模式匹配获得值
        println!("{}: {}",key,value);
    }

}
