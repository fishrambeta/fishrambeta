use crate::math::Equation;

pub struct IR{
    name: Vec<char>,
    parameters: Vec<IR>,
    surrounding_brackets: BracketType,
}
impl IR{
    pub fn latex_to_equation(latex: Vec<char>, implicit_multiplication: bool) -> Equation{
        return Self::latex_to_ir(latex, implicit_multiplication, BracketType::None).ir_to_equation();
    }
    pub fn equation_to_latex(equation: Equation, implicit_multiplication: bool) -> String{
        return Self::equation_to_ir(equation).ir_to_latex(implicit_multiplication).into_iter().collect::<String>();
    }
    pub fn latex_to_ir(latex: Vec<char>, implicit_multiplication: bool, surrounding_brackets: BracketType) -> Self{
        let mut latex = latex;
        let top_level_operators = Self::get_operators_in_top_level_from_latex(&latex, implicit_multiplication);
        if top_level_operators.any() {
            return if top_level_operators.powers.len() > 0 {
                let mut parts = vec!();
                for power in top_level_operators.powers {
                    let (lhs, rhs) = latex.split_at(power);
                    let (lhs, mut rhs) = (lhs.to_vec(), rhs.to_vec());
                    rhs.remove(0);
                    latex = rhs;
                    parts.push(lhs);
                }
                parts.push(latex);
                Self {
                    name: vec!['*'],
                    parameters: parts.into_iter().map(|parts| Self::latex_to_ir(parts, implicit_multiplication, BracketType::None)).collect::<Vec<_>>(),
                    surrounding_brackets,
                }
            } else if top_level_operators.multiplications_and_divisions.len() > 0 {
                let (lhs, rhs) = latex.split_at(top_level_operators.multiplications_and_divisions[0]);
                let (lhs, mut rhs) = (lhs.to_vec(), rhs.to_vec());
                let operator = rhs.remove(0);
                IR {
                    name: vec![operator],
                    parameters: vec![Self::latex_to_ir(lhs, implicit_multiplication, BracketType::None), Self::latex_to_ir(rhs, implicit_multiplication, BracketType::None)],
                    surrounding_brackets,
                }
            } else {
                let (lhs, rhs) = latex.split_at(top_level_operators.additions_and_subtractions[0]);
                let (lhs, mut rhs) = (lhs.to_vec(), rhs.to_vec());
                let operator = rhs.remove(0);
                IR {
                    name: vec![operator],
                    parameters: vec![Self::latex_to_ir(lhs, implicit_multiplication, BracketType::None), Self::latex_to_ir(rhs, implicit_multiplication, BracketType::None)],
                    surrounding_brackets,
                }
            }
        }
        else {
            if latex.starts_with(&['\\']){
                todo!();
            }
            else if latex.contains(&'\\'){
                todo!()
            }
            else if latex.contains(&'{') || latex.contains(&'(') || latex.contains(&'[') || latex.contains(&'⟨'){
                todo!()
            }
            else if  latex.iter().any(|char| char.is_numeric()){
                todo!()
            }
            else if implicit_multiplication{
                todo!()
            }
            else {
                return IR{
                    name: latex,
                    parameters: vec!(),
                    surrounding_brackets,
                };
            }
        }
    }
    pub fn ir_to_latex(mut self, implicit_multiplication: bool) -> Vec<char>{
        let name = self.name.clone();
        let mut return_data = vec!();
        match name[..]{
            ['+'] | ['-'] | ['*'] | ['/'] => {
                return_data.push(self.parameters[0].surrounding_brackets.opening_bracket());
                let closing_bracket = self.parameters[0].surrounding_brackets.closing_bracket();
                return_data.append(&mut Self::ir_to_latex(self.parameters.remove(0), implicit_multiplication));
                return_data.push(closing_bracket);
                while self.parameters.len() > 0{
                    return_data.push(self.name[0]); // The operator
                    return_data.push(self.parameters[0].surrounding_brackets.opening_bracket());
                    let closing_bracket = self.parameters[0].surrounding_brackets.closing_bracket();
                    return_data.append(&mut Self::ir_to_latex(self.parameters.remove(0), implicit_multiplication));
                    return_data.push(closing_bracket);
                }
            }
            _ => {
                todo!()
            }
        }
        return return_data;
    }
    pub fn ir_to_equation(self) -> Equation{
        todo!()
    }
    pub fn equation_to_ir(equation: Equation) -> Self{
        todo!()
    }
    ///Checks for the operators within the latex with the highest priority in the top level
    fn get_operators_in_top_level_from_latex(latex: &Vec<char>, implicit_multiplication: bool) -> TopLevelOperators{
        let mut depth = 0;
        let mut powers = vec!();
        let mut multiplications_and_divisions = vec!();
        let mut additions_and_subtractions = vec!();
        for (i,char) in latex.iter().enumerate(){
            if char == &'{' || char == &'(' || char == &'['{
                depth += 1;
            } else if char == &'}' || char == &')' || char == &']'{
                depth -= 1;
            }
            else if depth == 0{
                match char{
                    '+' | '-' => {
                        additions_and_subtractions.push(i);
                    }
                    '*' | '/' => {
                        multiplications_and_divisions.push(i);
                    }
                    '^' => {
                       if Self::check_if_caret_is_power(latex, i) && Self::check_if_power_is_top_level(latex, i, implicit_multiplication){
                            powers.push(i);
                       }
                    }
                    _ => {}
                }
            }
        }
        return TopLevelOperators{
            powers,multiplications_and_divisions,additions_and_subtractions
        }
    }
    ///Because the ^ character is ambiguous in latex between powers and superscript, this has to be checked
    fn check_if_caret_is_power(latex: &Vec<char>, caret: usize) -> bool{
        let mut chars_until_command_start = vec!();
        for i in (0..caret).rev(){
            if latex[i] != '\\'{
                chars_until_command_start.push(latex[i]);
            }
            else{
                break
            }
        }
        chars_until_command_start.reverse();
        if chars_until_command_start.contains(&'{'){return true};
        let command = chars_until_command_start.into_iter().collect::<String>();
        println!("{}",command);
        if &*command == "int"{return false}
        return true;
    }
    //A power in a power is not a top level operator, this function checks whether that is the case
    fn check_if_power_is_top_level(latex: &Vec<char>, caret: usize, implicit_multiplication: bool) -> bool{
        let mut i = caret - 1;
        while i > 0{
            if latex[i] == '^'{
                let part_inbetween = latex[i..caret].to_vec();
                todo!()
            }
            i -= 1;
        }
        return true;
    }
}
pub enum BracketType{
    None,
    Curly,
    Square,
    Round,
    Angle
}
impl BracketType{
    pub fn opening_bracket(&self) -> char{
        return match self{
            Self::None => ' ',
            Self::Angle => '⟨',
            Self::Curly => '{',
            Self::Square => '[',
            Self::Round => '('
        }
    }
    pub fn closing_bracket(&self) -> char{
        return match self{
            BracketType::None => ' ',
            BracketType::Curly => '}',
            BracketType::Square => ']',
            BracketType::Round => ')',
            BracketType::Angle => '⟩',
        }
    }
}
struct TopLevelOperators{
    powers: Vec<usize>,
    multiplications_and_divisions: Vec<usize>,
    additions_and_subtractions: Vec<usize>,
}
impl TopLevelOperators{
    pub fn get_highest_priority_operators(self) -> Vec<usize>{
        return if self.powers.len() > 0 { self.powers } else if self.multiplications_and_divisions.len() > 0 { self.multiplications_and_divisions } else { self.additions_and_subtractions }
    }
    pub fn any(&self) -> bool{
        return self.powers.len() > 0 || self.multiplications_and_divisions.len() > 0 || self.additions_and_subtractions.len() > 0;
    }
}
#[cfg(test)]
mod test{
    #[test]
    fn check_full_circle(){
        todo!()
    }
    #[test]
    fn test_check_if_caret_is_power(){
        assert_eq!(super::IR::check_if_caret_is_power(&"\\int^10{a}{b}".chars().collect::<Vec<char>>(), 4), false);
        assert_eq!(super::IR::check_if_caret_is_power(&"\\frac{a}{b}^10".chars().collect::<Vec<char>>(), 11), true);
    }
}