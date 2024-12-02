use std::collections::HashMap;

pub fn calc_total_distance(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    let mut total_distance = 0;

    if left_list.len() != right_list.len() {
        panic!("List lengths do not match");
    }

    let mut sorted_left_list = left_list;
    let mut sorted_right_list = right_list;

    sorted_left_list.sort();
    sorted_right_list.sort();

    for i in 0..sorted_left_list.len() {
        let (left, right) = (sorted_left_list[i], sorted_right_list[i]);
        total_distance += (left - right).abs();
    }

    return total_distance;
}

pub fn similarity_score(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    let mut scores: HashMap<i32, i32> = HashMap::new();

    fn score(n: i32, scores_map: &mut HashMap<i32, i32>, r_list: Vec<i32>) -> i32 {
        if scores_map.contains_key(&n) {
            return *scores_map.get(&n).expect("Didnt find key {n}");
        }

        let score = n * r_list.iter().filter(|&&x| x == n).count() as i32;
        scores_map.insert(n, score);
        return score;
    }

    let mut similarity_score = 0;
    for num in left_list {
        similarity_score += score(num, &mut scores, right_list.clone());
    }

    return similarity_score;
}
