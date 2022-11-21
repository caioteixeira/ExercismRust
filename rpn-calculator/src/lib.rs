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
            CalculatorInput::Value(v) => stack.push(*v),
            _ if stack.len() < 2 => return None,
            _ => process_operation(input, &mut stack),
        }
    }

    if stack.len() != 1 {
        return None;
    }

    Some(stack[0])
}

fn process_operation(operation: &CalculatorInput, stack: &mut Vec<i32>) {
    let v2 = stack.pop().unwrap();
    let v1 = stack.pop().unwrap();
    let value = match operation {
        CalculatorInput::Add => v1 + v2,
        CalculatorInput::Subtract => v1 - v2,
        CalculatorInput::Multiply => v1 * v2,
        CalculatorInput::Divide => v1 / v2,
        _ => unreachable!(),
    };

    stack.push(value);
}
