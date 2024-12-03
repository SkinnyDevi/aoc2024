use regex::Regex;

fn mul_str_to_pair(mul_str: &String) -> (i32, i32) {
    let params_str: Vec<&str> = (mul_str).split(",").collect();

    let mut params: Vec<i32> = Vec::with_capacity(2);
    for pstr in params_str {
        match pstr.parse::<i32>() {
            Ok(parsed) => params.push(parsed),
            Err(e) => println!("Could not parse param ({pstr}): {e}"),
        }
    }

    return (params[0], params[1]);
}

fn get_valid_mul_instances(memory: &String) -> Vec<(i32, i32)> {
    let pattern = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let mut found_muls: Vec<(i32, i32)> = Vec::new();
    for mul_match in pattern.find_iter(&memory) {
        let mut mul = String::from(mul_match.as_str());
        mul = mul.replace("mul(", "").replace(")", "");
        found_muls.push(mul_str_to_pair(&mul));
    }

    return found_muls;
}

fn get_valid_condtional_mul_instances(memory: &String) -> Vec<(i32, i32)> {
    let pattern = Regex::new(r"(do\(\)|don't\(\))|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let mut found_muls: Vec<(i32, i32)> = Vec::new();
    let mut can_parse_mul = true;
    for mul_match in pattern.find_iter(&memory) {
        let mut mul = String::from(mul_match.as_str());
        if mul == "do()" || mul == "don't()" {
            can_parse_mul = mul == "do()";
            continue;
        }

        if !can_parse_mul {
            continue;
        }

        mul = mul.replace("mul(", "").replace(")", "");
        found_muls.push(mul_str_to_pair(&mul));
    }

    return found_muls;
}

pub fn multiply_memory(memory: &String, use_conditional_muls: Option<bool>) -> i32 {
    let use_mul_conditions = use_conditional_muls.unwrap_or(false);
    let found_muls: Vec<(i32, i32)> = if !use_mul_conditions {
        get_valid_mul_instances(memory)
    } else {
        get_valid_condtional_mul_instances(memory)
    };

    let mut result = 0;
    for mul_param in found_muls {
        result += mul_param.0 * mul_param.1;
    }

    return result;
}
