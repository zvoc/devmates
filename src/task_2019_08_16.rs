// 16/08/2019
//
// Add the mathematical operators + or - before any of the digits in the
// decimal numeric string 123456789 such that the resulting mathematical
// expression adds up to 100. Return all possible solutions.
//
// Example:
// 1+2+3-4+5+6+78+9
// 123+45-67+8-9
// -1+2-3+4+5+6+78+9

use std::time::Instant;

const MAX_DEPTH: u32 = 10;
const TARGET_NUM: i32 = 100;

trait Operator {
    fn forward(state: &State, depth: u32,
               depth_str: &String, expression: &mut String) -> State;
    fn backward(depth_str: &String, expression: &mut String);
}

struct Plus {}
impl Operator for Plus {
    fn forward(state: &State, depth: u32,
               depth_str: &String, expression: &mut String) -> State {
        expression.push('+');
        expression.push_str(depth_str.as_str());

        return State {
            sum: state.sum + depth as i32,
            curr_num: depth as i32,
        };
    }

    fn backward(depth_str: &String, expression: &mut String) {
        expression.truncate(expression.len() - depth_str.len() - 1);
    }
}

struct Minus {}
impl Operator for Minus {
    fn forward(state: &State, depth: u32,
               depth_str: &String, expression: &mut String) -> State {
        expression.push('-');
        expression.push_str(depth_str.as_str());

        return State {
            sum: state.sum - depth as i32,
            curr_num: -(depth as i32),
        };
    }

    fn backward(depth_str: &String, expression: &mut String) {
        expression.truncate(expression.len() - depth_str.len() - 1);
    }
}

struct Concat {}
impl Operator for Concat {
    fn forward(state: &State, depth: u32,
               depth_str: &String, expression: &mut String) -> State {
        expression.push_str(depth_str.as_str());
        let c: i32 = if state.curr_num < 0 { -1 } else { 1 };
        let new_curr_num = state.curr_num * 10 + c * depth as i32;
        let new_sum = state.sum - state.curr_num + new_curr_num;
        return State {
            sum: new_sum,
            curr_num: new_curr_num,
        };
    }

    fn backward(depth_str: &String, expression: &mut String) {
        expression.truncate(expression.len() - depth_str.len());
    }
}

struct State {
    sum: i32,
    curr_num: i32,
}

fn search_states<T: Operator>(
    depth: u32,
    state: &State,
    expression: &mut String,
    expressions: &mut Vec<String>,
) {
    let curr_depth = depth + 1;

    if curr_depth >= MAX_DEPTH {
        return;
    }

    let curr_depth_str = curr_depth.to_string();
    let curr_state = T::forward(state, curr_depth, &curr_depth_str, expression);

    if curr_depth + 1 == MAX_DEPTH {
        if curr_state.sum == TARGET_NUM {
            expressions.push(expression.clone());
        }
    } else {
        search_states::<Plus>(curr_depth, &curr_state,
                              expression, expressions);
        search_states::<Concat>(curr_depth, &curr_state,
                                expression, expressions);
        search_states::<Minus>(curr_depth, &curr_state,
                               expression, expressions);
    }

    T::backward(&curr_depth_str, expression);
}

fn main() {
    let now = Instant::now();

    let mut expressions: Vec<String> = Vec::new();
    let mut curr_expression = String::new();
    let state = State {
        sum: 0,
        curr_num: 0,
    };

    search_states::<Minus>(0, &state, &mut curr_expression, &mut expressions);
    search_states::<Concat>(0, &state, &mut curr_expression, &mut expressions);

    println!("Finished in: {}", now.elapsed().as_millis());

    println!("Expression that are equal to {}:", TARGET_NUM);
    for expression in expressions {
        println!("{}", expression);
    }
}
