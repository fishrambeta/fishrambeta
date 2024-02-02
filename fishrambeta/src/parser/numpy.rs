use crate::parser::IR;

impl IR {
    pub fn ir_to_numpy(mut self, implicit_multiplication: bool) -> Vec<char> {
        let name = self.name.clone();
        let mut return_data = vec![];
        match name[..] {
            ['+'] | ['-'] | ['*'] => {
                return_data.push('(');
                while self.parameters.len() > 0 {
                    return_data.append(&mut IR::ir_to_numpy(
                        self.parameters.remove(0).0,
                        implicit_multiplication,
                    ));
                    return_data.push(name[0])
                }
                return_data.pop();
                return_data.push(')');
            }
            ['^'] => {
                if self.parameters.len() != 2 {
                    panic!("Invalid power, not two parameters");
                }
                return_data.extend("np.power(".chars().collect::<Vec<char>>());
                return_data.append(&mut IR::ir_to_numpy(
                    self.parameters.remove(0).0,
                    implicit_multiplication,
                ));
                return_data.extend(",".chars().collect::<Vec<char>>());
                return_data.append(&mut IR::ir_to_numpy(
                    self.parameters.remove(0).0,
                    implicit_multiplication,
                ));
                return_data.extend(")".chars().collect::<Vec<char>>());
            }
            ['\\', 'f', 'r', 'a', 'c'] => {
                return_data.push('(');
                return_data.append(&mut IR::ir_to_numpy(
                    self.parameters.remove(0).0,
                    implicit_multiplication,
                ));
                return_data.push(')');
                return_data.push('/');
                return_data.push('(');
                return_data.append(&mut IR::ir_to_numpy(
                    self.parameters.remove(0).0,
                    implicit_multiplication,
                ));
                return_data.push(')');
            }
            ['\\', 'i', 'n', 'v'] => {
                if self.parameters.len() == 1 {
                    let mut result = vec!['-'];
                    result.push('(');
                    result.append(
                        &mut self
                            .parameters
                            .remove(0)
                            .0
                            .ir_to_numpy(implicit_multiplication),
                    );
                    result.push(')');
                    return result;
                }
                panic!();
            }
            ['\\', 's', 'q', 'r', 't'] => {
                return_data.extend("np.sqrt(".chars().collect::<Vec<char>>());
                return_data.append(&mut IR::ir_to_numpy(
                    self.parameters.remove(0).0,
                    implicit_multiplication,
                ));
                return_data.push(')');
            }
            ['\\', 's', 'i', 'n'] => {
                return_data.extend("np.sin(".chars().collect::<Vec<char>>());
                return_data.append(&mut IR::ir_to_numpy(
                    self.parameters.remove(0).0,
                    implicit_multiplication,
                ));
                return_data.push(')');
            }
            ['\\', 'c', 'o', 's'] => {
                return_data.extend("np.cos(".chars().collect::<Vec<char>>());
                return_data.append(&mut IR::ir_to_numpy(
                    self.parameters.remove(0).0,
                    implicit_multiplication,
                ));
                return_data.push(')');
            }
            ['\\', 'l', 'n'] => {
                return_data.extend("np.log(".chars().collect::<Vec<char>>());
                return_data.append(&mut IR::ir_to_numpy(
                    self.parameters.remove(0).0,
                    implicit_multiplication,
                ));
                return_data.push(')');
            }
            _ => {
                if self.parameters.len() == 0 {
                    return self.name;
                } else {
                    let mut string = self.name.into_iter().collect::<Vec<_>>();
                    for parameter in self.parameters {
                        if let Some(bracket) = parameter.1.opening_bracket() {
                            string.push(bracket)
                        };
                        string.append(&mut Self::ir_to_numpy(parameter.0, implicit_multiplication));
                        if let Some(bracket) = parameter.1.closing_bracket() {
                            string.push(bracket)
                        }
                    }
                    return string;
                }
            }
        }
        return return_data;
    }
}
