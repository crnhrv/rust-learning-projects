use std::collections::HashMap;

fn main() {
    let vec1 = vec![1,2,3,4,5,6,7,8,9,10,11,11,11];
    let mut vec2 = vec![643,34,4,32354,566,54534,42333,21,11,1,11,11,1];
    vec2.sort();

    let vec2_med = get_median(&vec2);
    let vec1_med = get_median(&vec1);

    let vec2_mode = get_mode(&vec2);
    let vec1_mode = get_mode(&vec1);

    println!("vec1: {:?}", vec1);
    println!("vec2: {:?}", vec2);


    println!("vec1 median: {}, vec1 mode: {}, vec2 median: {}, vec2 mode: {}", vec1_med, vec1_mode, vec2_med, vec2_mode);
}

fn get_median(sorted_slice: &[u32]) -> u32 {    
    let length = sorted_slice.len();
    if length % 2 == 0 {
        return (sorted_slice[(length / 2) - 1] + sorted_slice[(length / 2)]) / 2
    }  else {        
        return sorted_slice[(length / 2)]
    }
}

fn get_mode(slice: &[u32]) -> u32 {
    let mut map = HashMap::new();

    let mut max = (0,0);
    for value in slice {
        let kv = map.entry(value).or_insert(0);
        *kv += 1;
        
        if *kv > max.0
        {
            max.1 = *kv;
            max.0 = *value;
        } 
    };
    
    return max.0;
}
