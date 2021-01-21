use std::collections::HashMap;

pub fn main() {
 let text="Hello world wonderful world";
    let mut map =HashMap::new();
    for word in text.split_whitespace() {
        let count =map.entry(word).or_insert(0);    //entry会判断目标key是否存在，如果目标key不存在，or_insert方法会插入这个key，并插入value值0，此时hashmap中就有了目标键值对，同时or_insert会返回一个对该value值的可变引用，entry也是同理
        *count+=1;  //如果想通过可变引用改变引用的值，需要使用解引用“*”，再对其进行赋值
    }
    println!("{:#?}",map);  //用debug模式输出

}
