#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::<i32>::new();
    for elem in inputs {
        match elem {
            CalculatorInput::Value(x) => {
                stack.push(*x);
            }
            _ => {
                if stack.len() < 2 {
                    return None;
                }

                let second = stack.pop().unwrap();
                let first = stack.pop().unwrap();

                let res = match elem {
                    CalculatorInput::Add => first + second,
                    CalculatorInput::Subtract => first - second,
                    CalculatorInput::Multiply => first * second,
                    CalculatorInput::Divide => first / second,
                    _ => panic!("Type Error")
                };
                stack.push(res);
            }
        }
    }

    if stack.len() > 1 || stack.len() == 0{
        return None;
    }

    return Some(stack[0]);
}
