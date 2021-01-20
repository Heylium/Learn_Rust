use std::collections::HashMap;

pub fn main() {
    let field_name=String::from("Favorite color");
    let field_value=String::from("Blue");
    let mut map=HashMap::new();
    map.insert(&field_name,&field_value);//hashmap对有copy trait类型的数据为值传递，非copy trait类型为引用传递，hashmap会获得数据所有权，所以最好还是用引用&传入。

    println!("{}: {}",field_name,field_value);

}
