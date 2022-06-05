use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let mut sum = 0.0;

    for num in list {
        sum += *num as f64;
    }

    sum / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut temp_list = list.clone();
    temp_list.sort();

    return if temp_list.len() % 2 == 0 {
        (temp_list[temp_list.len() / 2 + 1] + temp_list[temp_list.len() / 2]) / 2
    } else {
        temp_list[temp_list.len() / 2]
    };
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut key_value = (0, 0);

    for num in list {
        let counter = map.entry(*num).or_insert(0);
        *counter += 1;
    }

    for map_key_value in map {
        key_value = if map_key_value.1 > key_value.1 {
            map_key_value
        } else {
            key_value
        }
    }

    key_value.0
}
