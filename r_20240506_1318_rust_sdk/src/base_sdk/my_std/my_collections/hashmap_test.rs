#[cfg(test)]
mod hashmap_test_1 {
    use std::collections::HashMap;

    #[test]
    fn stdin_read_line_1() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);

        println!("You score: {score} {team_name}");

        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // 由于已经 insert到了map里面，所有权进行了改变
        // println!("You score: {field_name}");
        // println!("You score: {field_value}");

        // map 字段的更新
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        println!("{:?}", scores);

    }
}