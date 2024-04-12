use crate::parser::{BracketType, IR};

impl IR {
    pub fn latex_to_ir(
        mut latex: Vec<char>,
        implicit_multiplication: bool,
        first_pass: bool,
    ) -> Result<Self, ParseError> {
        if first_pass{
            latex = Self::make_minus_signs_positive_minus_pairs(latex);
        }
        latex = Self::make_implicit_multiplications_explicit(latex, implicit_multiplication);
        let top_level_operators = Self::get_top_level_operators_in_latex(&latex);
        if top_level_operators.any(){
            let mut highest_priority_operator_positions = top_level_operators.get_highest_priority_top_level_operators();
            highest_priority_operator_positions.reverse();
            let mut parts = vec!();
            let mut operator = '?';
            for position in highest_priority_operator_positions.into_iter() {
                let (lhs, rhs) = latex.split_at(position);
                let (lhs, mut rhs) = (lhs.to_vec(), rhs.to_vec());
                operator = rhs.remove(0);
                latex = lhs;
                parts.push(rhs);
            }
            parts.push(latex);
            parts.reverse();
            let ir_parts = parts.into_iter().map(|part| (Self::latex_to_ir(part, implicit_multiplication, false).unwrap(), BracketType::None)).collect::<Vec<_>>();
            return Ok(Self{
                name: vec!(operator),
                parameters: ir_parts,
            })
        }
        else {
            let mut surrounding_brackets = BracketType::None;
            if BracketType::is_opening_bracket(latex[0]) && BracketType::is_closing_bracket(latex[latex.len() - 1]){
                surrounding_brackets = BracketType::get_opening_bracket_type(latex.remove(0));
                latex.remove(latex.len() - 1);
                return Ok(Self{
                    name: vec!(),
                    parameters: vec!(((Self::latex_to_ir(latex, implicit_multiplication, false)).unwrap(), surrounding_brackets))
                });
            }
            if latex[0] == '\\'{
                latex.remove(0);
                let mut command = vec!('\\');
                while latex.len() > 0 && latex[0].is_alphabetic(){
                    command.push(latex.remove(0))
                }
                let mut parameter_count = Self::get_parameter_count(&command[1..]);
                let mut parameters = vec!();
                for _ in 0..parameter_count{
                    if !BracketType::is_opening_bracket(latex[0]){
                        panic!("Command with invalid parameters, {}", command.iter().collect::<String>());
                    }
                    let surrounding_brackets = BracketType::get_opening_bracket_type(latex.remove(0));
                    let mut depth = 1;
                    let mut parameter = vec!();
                    while depth > 0{
                        let next = latex.remove(0);
                        if BracketType::is_opening_bracket(next){depth+=1}
                        else if BracketType::is_closing_bracket(next){depth-=1}
                        if depth != 0{
                            parameter.push(next);
                        }
                    }
                    parameters.push((parameter, surrounding_brackets));
                }
                return Ok(Self{
                    name: command,
                    parameters: parameters.into_iter().map(|par| ((Self::latex_to_ir(par.0, implicit_multiplication, false).unwrap(), par.1))).collect::<Vec<_>>()
                })
            }
            else if latex.iter().all(|char| char.is_numeric()){
                return Ok(Self{
                    name: latex,
                    parameters: vec!()
                });
            }
            else if latex.iter().all(|char| char.is_alphabetic()){
                return Ok(Self{
                    name: latex,
                    parameters: vec!()
                })
            }
            todo!();
        }
    }
    pub fn ir_to_latex(mut self, implicit_multiplication: bool) -> Vec<char> {
        match self.name[..]{
            ['='] | ['+'] | ['*'] | ['^']=> {
                let mut latex = vec!();
                let last = self.parameters.remove(self.parameters.len()-1);
                for param in self.parameters{
                    let (par, bracket) = param;
                    if let Some(opening_bracket) = bracket.opening_bracket(){
                        latex.push(opening_bracket)
                    }
                    latex.append(&mut Self::ir_to_latex(par, implicit_multiplication));
                    if let Some(closing_bracket) = bracket.closing_bracket(){
                        latex.push(closing_bracket)
                    }
                    latex.append(&mut self.name.clone());
                }
                if let Some(opening_bracket) = last.1.opening_bracket(){
                    latex.push(opening_bracket)
                }
                latex.append(&mut Self::ir_to_latex(last.0, implicit_multiplication));
                if let Some(closing_bracket) = last.1.closing_bracket(){
                    latex.push(closing_bracket)
                }
                return latex;
            }
            ['\\', 'f', 'r', 'a', 'c'] | ['\\', 'v', 'e', 'c']=> {
                let mut latex = self.name.to_vec();
                for param in self.parameters{

                    if let Some(opening_bracket) = param.1.opening_bracket(){
                        latex.push(opening_bracket)
                    }
                    latex.append(&mut Self::ir_to_latex(param.0, implicit_multiplication));
                    if let Some(closing_bracket) = param.1.closing_bracket(){
                        latex.push(closing_bracket)
                    }
                }
                return latex;
            }
            _=>{
                if self.name.len() == 0{
                    let mut latex = vec!();
                    let mut param = self.parameters.remove(0);
                    if let Some(opening_bracket) = param.1.opening_bracket(){
                        latex.push(opening_bracket)
                    }
                    latex.append(&mut Self::ir_to_latex(param.0, implicit_multiplication));
                    if let Some(closing_bracket) = param.1.closing_bracket(){
                        latex.push(closing_bracket)
                    }
                    return latex
                }
                else if self.name.iter().all(|char| char.is_alphabetic() || char.is_numeric()) && self.parameters.len() == 0{
                    return self.name;
                }
                todo!();
            }
        }
    }
    fn calculate_depth_difference(latex: &Vec<char>) -> isize {
        let mut depth = 0;
        for char in latex.iter() {
            if BracketType::is_opening_bracket(*char) {
                depth += 1;
            } else if BracketType::is_closing_bracket(*char) {
                depth -= 1;
            }
        }
        return depth;
    }
    fn get_top_level_operators_in_latex(latex: &Vec<char>) -> TopLevelOperators {
        let mut depth = 0;
        let mut operators = TopLevelOperators {
            powers: vec![],
            multiplications: vec![],
            additions: vec![],
            equals: vec![],
        };
        for (i, char) in latex.iter().enumerate() {
            if BracketType::is_opening_bracket(*char) {
                depth -= 1;
            } else if BracketType::is_closing_bracket(*char) {
                depth += 1;
            } else if depth == 0 {
                if *char == '=' {
                    operators.equals.push(i)
                } else if *char == '*' {
                    operators.multiplications.push(i)
                } else if *char == '+' && i != 0 && !BracketType::is_opening_bracket(latex[i - 1])
                {
                    //If the thing before the operator is not something that can be added to or subtracted from, this is not an operator
                    operators.additions.push(i);
                } else if *char == '^' && Self::check_if_caret_is_power(latex, i) {
                    //This can also be used for superscript or integral upper bounds
                    operators.powers.push(i);
                }
            }
        }
        return operators;
    }
    //Checks whether the ^ char is a power or just superscript
    fn check_if_caret_is_power(latex: &Vec<char>, pos: usize) -> bool {
        let mut parameter_count = 0;
        let mut command = vec![];
        let mut depth = if BracketType::is_closing_bracket(latex[pos - 1]) {
            1
        } else {
            0
        };
        for i in (0..(pos - depth)).rev() {
            if depth == 0 && !BracketType::is_closing_bracket(latex[i]) {
                if i == 0 && latex[i] != '\\' {
                    command = vec![];
                    break;
                }
                if !latex[i].is_alphabetic() {
                    if latex[i] != '\\' {
                        command = vec![];
                    }
                    break;
                } else if latex[i].is_alphabetic() {
                    command.push(latex[i])
                }
            } else if depth == 0 && BracketType::is_opening_bracket(latex[i + 1]) {
                parameter_count += 1;
            }
            if BracketType::is_closing_bracket(latex[i]) {
                depth += 1;
            } else if BracketType::is_opening_bracket(latex[i]) {
                depth -= 1;
            }
        }
        return if command.len() == 0 {
            true
        } else {
            command.reverse();
            Self::get_parameter_count(&command) == parameter_count
        };
    }
    ///Add * where implicit multiplations are present, to make parsing easier
    fn make_implicit_multiplications_explicit(
        mut latex: Vec<char>,
        implicit_multiplication: bool,
    ) -> Vec<char> {
        if implicit_multiplication {
            //Add multiplication signs where two letters are next to eachother, but don't do it in commands
            let mut new_latex: Vec<char> = vec![];
            let mut multiplication_insertion_suspended = false;
            for char in latex {
                if char == '\\' {
                    if let Some(prev) = new_latex.last()
                        && !multiplication_insertion_suspended {
                        if *prev != '*' { new_latex.push('*') }
                    }
                    multiplication_insertion_suspended = true
                } else if multiplication_insertion_suspended && !char.is_alphabetic() {
                    multiplication_insertion_suspended = false
                } else if let Some(prev) = new_latex.last()
                    && !multiplication_insertion_suspended
                {
                    if prev.is_alphabetic() && char.is_alphabetic() {
                        new_latex.push('*');
                    } else if (prev.is_alphabetic() && char.is_numeric())
                        || prev.is_numeric() && char.is_alphabetic()
                    {
                        new_latex.push('*')
                    }
                }
                new_latex.push(char);
            }
            latex = new_latex;
        }
        //Add multiplications between closing and opening brackets
        let mut new_latex = vec![];
        let mut building_command = false;
        let mut command = vec![];
        let mut depth = 0;
        let mut parameter_count = 0;
        for char in latex {
            //Some commands have multiple params
            if BracketType::is_opening_bracket(char) {
                depth += 1;
            } else if BracketType::is_closing_bracket(char) {
                depth -= 1;
            }
            if depth == 0 {
                if char == '\\' {
                    building_command = true;
                } else if building_command && !char.is_alphabetic() {
                    building_command = false;
                    parameter_count = Self::get_parameter_count(&command);
                } else if building_command {
                    command.push(char);
                } else {
                    command = vec![];
                }
            }
            if depth <= 1
                && BracketType::is_opening_bracket(char)
                && let Some(&prev) = new_latex.last()
            {
                if BracketType::is_closing_bracket(prev) {
                    if parameter_count <= 1 {
                        new_latex.push('*')
                    } else {
                        parameter_count -= 1
                    }
                }
            }
            new_latex.push(char);
        }
        return new_latex;
    }
    ///Replace all minus signs with +- to make parsing easier
    fn make_minus_signs_positive_minus_pairs(mut latex: Vec<char>) -> Vec<char>{
        let mut new_latex = vec!(latex.remove(0));
        for char in latex.into_iter(){
            if char == '-' && new_latex[new_latex.len() - 1] != '+'{
                new_latex.push('+');
            }
            new_latex.push(char)
        }
        return new_latex;
    }
    fn get_parameter_count(command: &[char]) -> u32 {
        let command = command.iter().collect::<String>();
        return match command.as_str() {
            "tan" | "cos" | "sin" => 0,
            "vec" => 1,
            "frac" => 2,
            _ => {
                todo!("{} has no specified parameter count", command)
            }
        };
    }
}
#[derive(Clone)]
struct TopLevelOperators {
    powers: Vec<usize>,
    multiplications: Vec<usize>,
    additions: Vec<usize>,
    equals: Vec<usize>,
}

impl TopLevelOperators {
    pub fn any(&self) -> bool {
        return self.powers.len() > 0
            || self.multiplications.len() > 0
            || self.additions.len() > 0
            || self.equals.len() > 0;
    }
    pub fn get_highest_priority_top_level_operators(self) -> Vec<usize>{
        if self.equals.len() > 0{
            return self.equals;
        }
        else if self.additions.len() > 0{
            return self.additions;
        }
        else if self.multiplications.len() > 0{
            return self.multiplications;
        }
        else {
            return self.powers;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    InvalidLatex,
    InvalidExpression,
}
