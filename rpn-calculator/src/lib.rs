#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Add => {
                if stack.len() < 2 {
                    return None;
                }

                let v2 = stack.pop();
                let v1 = stack.pop();
                let value = v1.unwrap() + v2.unwrap();
                stack.push(value);
            }
            CalculatorInput::Subtract => {
                if stack.len() < 2 {
                    return None;
                }

                let v2 = stack.pop();
                let v1 = stack.pop();
                let value = v1.unwrap() - v2.unwrap();
                stack.push(value);
            }
            CalculatorInput::Multiply => {
                if stack.len() < 2 {
                    return None;
                }

                let v2 = stack.pop();
                let v1 = stack.pop();
                let value = v1.unwrap() * v2.unwrap();
                stack.push(value);
            }
            CalculatorInput::Divide => {
                if stack.len() < 2 {
                    return None;
                }

                let v2 = stack.pop();
                let v1 = stack.pop();
                let value = v1.unwrap() / v2.unwrap();
                stack.push(value);
            }
            CalculatorInput::Value(v) => stack.push(*v),
        }
    }

    if stack.len() != 1 {
        return None;
    }

    Some(stack[0])
}
