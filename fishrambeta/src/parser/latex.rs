use crate::parser::{BracketType, IR};

impl IR {
    pub fn latex_to_ir(
        mut latex: Vec<char>,
        implicit_multiplication: bool,
        allow_d_in_integral: bool,
        first_pass: bool,
    ) -> Result<Self, ParseError> {
        if first_pass{
            latex = Self::fix_signs(latex);
        }
        latex = Self::convert_integral_delimiters_to_parameters(latex, allow_d_in_integral);
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
            let ir_parts = parts.into_iter().map(|part| (Self::latex_to_ir(part, implicit_multiplication, allow_d_in_integral, false).unwrap(), BracketType::None)).collect::<Vec<_>>();
            return Ok(Self{
                name: vec!(operator),
                parameters: ir_parts,
                subscript: None,
                superscript: None
            })
        }
        else {
            let mut surrounding_brackets = BracketType::None;
            if BracketType::is_opening_bracket(latex[0]) && BracketType::is_closing_bracket(latex[latex.len() - 1]){
                surrounding_brackets = BracketType::get_opening_bracket_type(latex.remove(0));
                latex.remove(latex.len() - 1);
                return Ok(Self{
                    name: vec!(),
                    parameters: vec!(((Self::latex_to_ir(latex, implicit_multiplication, allow_d_in_integral, false)).unwrap(), surrounding_brackets)),
                    superscript: None,
                    subscript: None,
                });
            }
            if latex[0] == '\\'{
                latex.remove(0);
                let mut command = vec!('\\');
                let mut depth = 0;
                let mut start_of_block = false;
                while latex.len() > 0 && ((latex[0].is_alphanumeric() || latex[0] == '^' || latex[0] == '_') || depth > 0 || start_of_block){
                    if start_of_block{
                        start_of_block = false;
                    }
                    if (latex[0] == '^' || latex[0] == '_') && BracketType::is_opening_bracket(latex[1]){
                        start_of_block = true;
                    }
                    if BracketType::is_opening_bracket(latex[0]){
                        depth += 1;
                    }
                    if BracketType::is_closing_bracket(latex[0]){
                        depth -= 1;
                    }
                    command.push(latex.remove(0))
                }
                let (command, superscript, subscript) = Self::extract_super_and_subscript(command);
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
                    parameters: parameters.into_iter().map(|par| ((Self::latex_to_ir(par.0, implicit_multiplication, allow_d_in_integral,  false).unwrap(), par.1))).collect::<Vec<_>>(),
                    superscript,
                    subscript
                })
            }
            else if latex.iter().all(|char| char.is_numeric()){
                return Ok(Self{
                    name: latex,
                    parameters: vec!(),
                    subscript: None,
                    superscript: None,
                });
            }
            else if latex.iter().all(|char| char.is_alphabetic()){
                return Ok(Self{
                    name: latex,
                    parameters: vec!(),
                    superscript: None,
                    subscript: None,
                })
            }
            else if latex[0] == '-'{
                latex.remove(0);
                //TODO, it might be possible to have a case like -{}{}, test this (that would be a bug elsewhere)
                if BracketType::is_opening_bracket(latex[0]) && BracketType::is_closing_bracket(latex[latex.len()-1]){
                    latex.remove(0);
                    latex.remove(latex.len() - 1);
                }
                return Ok(Self{
                            name: vec!('\\', 'i', 'n', 'v'),
                            parameters: vec!((Self::latex_to_ir(latex, implicit_multiplication, allow_d_in_integral, false).unwrap(), BracketType::Curly)),
                            superscript: None,
                            subscript: None,
                        })
            }
            // else if latex[0] == '-' && (latex[1..].iter().all(|char| char.is_alphabetic()) | latex[1..].iter().all(|char| char.is_numeric())){}
            // else if latex.iter().enumerate().all(|(i,char)| {char.is_alphabetic() || (i==0 && *char == '-')})
            //     || latex.iter().enumerate().all(|(i,char)| {char.is_numeric() || (i==0 && *char == '-')})
            //     || latex.iter().enumerate().all(|(i, char)| {char.is_numeric() || (i==0 && *char == '-') || (BracketType::is_opening_bracket(*char) && i == 1) ||(BracketType::is_closing_bracket(*char) && i == latex.len()-1)})
            //     || latex.iter().enumerate().all(|(i, char)| {char.is_alphabetic() || (i==0 && *char == '-') || (BracketType::is_opening_bracket(*char) && i == 1) ||(BracketType::is_closing_bracket(*char) && i == latex.len()-1)}){
            //     latex.remove(0);
            //     return Ok(Self{
            //         name: vec!('\\', 'i', 'n', 'v'),
            //         parameters: vec!((Self{
            //             name: latex,
            //             parameters: vec!(),
            //             superscript: None,
            //             subscript: None,
            //         }, BracketType::Curly)),
            //         superscript: None,
            //         subscript: None,
            //     })
            // }
            todo!();
        }
    }
    pub fn ir_to_latex(mut self, implicit_multiplication: bool) -> Vec<char> {
        match self.name[..]{
            ['='] | ['+'] | ['^']=> {
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
            ['*'] => {
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
                    if !implicit_multiplication{
                        latex.append(&mut self.name.clone());
                    }
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
            ['\\','i','n','v'] => {
                let mut latex = vec!('-');
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
                    latex.push('-');
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
                } else if self.parameters.len() > 0{
                    let mut latex = if self.name[..]!= ['\\', 'i', 'n', 't', 'i', 'n', 't'] {self.name.to_vec()}else{vec!('\\','i','n','t')};
                    if let Some((mut subscript, bracket_type)) = self.subscript{
                        latex.push('_');
                        if let Some(opening_bracket) = bracket_type.opening_bracket(){
                            latex.push(opening_bracket);
                        }
                        latex.append(&mut subscript);
                        if let Some(closing_bracket) = bracket_type.closing_bracket(){
                            latex.push(closing_bracket);
                        }
                    }
                    if let Some((mut superscript, bracket_type)) = self.superscript{
                        latex.push('^');
                        if let Some(opening_bracket) = bracket_type.opening_bracket(){
                            latex.push(opening_bracket);
                        }
                        latex.append(&mut superscript);
                        if let Some(closing_bracket) = bracket_type.closing_bracket(){
                            latex.push(closing_bracket);
                        }
                    }
                    for (i, param) in self.parameters.into_iter().enumerate(){
                        if i == 1 && self.name[..] == ['\\', 'i', 'n', 't', 'i', 'n', 't']{
                            latex.push('d');
                        }
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
                else if self.name.iter().all(|char| char.is_alphabetic() || *char == '\\' || char.is_numeric()) && self.parameters.len() == 0{
                    return self.name;
                }
                panic!("Unknown latex parsing issue");
            }
        }
    }
    ///Counts the amount of opening minus closing brackets
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
                } else if *char == '+' && i != 0 && !BracketType::is_opening_bracket(latex[i - 1]) && latex[i - 1] != '=' && latex[i-1] != '+' && latex[i-1] != '*'
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
    ///Checks whether the ^ char is a power or just superscript
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
                if !latex[i].is_alphanumeric() && latex[i] != '_' {
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
    ///Add * where implicit multiplication are present, to make parsing easier
    fn make_implicit_multiplications_explicit(mut latex: Vec<char>, implicit_multiplication: bool) -> Vec<char> {
        if implicit_multiplication {
            //Add multiplication signs where two letters are next to each-other, but don't do it in commands
            let mut new_latex: Vec<char> = vec![];
            let mut multiplication_insertion_suspended = false;
            for (i,char) in latex.into_iter().enumerate() {
                if char == '\\' {
                    if let Some(prev) = new_latex.last()
                        && !multiplication_insertion_suspended {
                        if *prev != '*' && !BracketType::is_opening_bracket(*prev) && *prev != '=' { new_latex.push('*') }
                    }
                    multiplication_insertion_suspended = true
                } else if multiplication_insertion_suspended && !char.is_alphabetic() {
                    multiplication_insertion_suspended = false
                } else if let Some(prev) = new_latex.last()
                    && !multiplication_insertion_suspended
                {
                    if prev.is_alphabetic() && (char.is_alphabetic() || BracketType::is_opening_bracket(char)){
                        new_latex.push('*');
                    } else if (prev.is_alphabetic() && char.is_numeric())
                        || prev.is_numeric() && char.is_alphabetic()
                    {
                        new_latex.push('*')
                    } else if BracketType::is_closing_bracket(*prev) && char.is_alphanumeric(){
                        new_latex.push('*')
                    }
                }
                new_latex.push(char);
            }
            latex = new_latex;
        }
        else{
            //Add multiplication signs where two letters are next to each-other, but don't do it in commands
            let mut new_latex: Vec<char> = vec![];
            let mut multiplication_insertion_suspended = false;
            for char in latex {
                if char == '\\' {
                    if let Some(prev) = new_latex.last()
                        && !multiplication_insertion_suspended {
                        if *prev != '*' && !BracketType::is_opening_bracket(*prev) && *prev != '=' { new_latex.push('*') }
                    }
                    multiplication_insertion_suspended = true
                }
                else if multiplication_insertion_suspended && !char.is_alphabetic() {
                    multiplication_insertion_suspended = false
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
                else if prev == '_' || prev == '^'{
                    parameter_count += 1;
                }
            }
            new_latex.push(char);
        }
        return new_latex;
    }
    ///Replace all minus signs with +- to make parsing easier
    ///Remove unnecessary positive signs
    fn fix_signs(mut latex: Vec<char>) -> Vec<char>{
        let mut new_latex = vec!(latex.remove(0));
        for char in latex.into_iter(){
            if char == '-' && new_latex[new_latex.len() - 1] != '+'{
                new_latex.push('+');
            }
            new_latex.push(char);
            if char == '+' && (new_latex[new_latex.len() - 2] == '+' || new_latex[new_latex.len() - 2] == '-' || new_latex[new_latex.len() - 2] == '*' || new_latex[new_latex.len() - 2] == '='){
                new_latex.remove(new_latex.len()-1);
            }
        }
        return new_latex;
    }
    ///List of all commands and their respective parameter count
    fn get_parameter_count(command: &[char]) -> u32 {
        let command = command.iter().collect::<String>();
        return match command.as_str() {
            "pi" | "alpha" | "beta" | "gamma" | "theta" => 0,
            "vec" | "tan" | "cos" | "sin" | "sqrt" | "arctan" | "arcsin" | "arccos"=> 1,
            "frac" | "intint" => 2,
            _ => {
                todo!("{} has no specified parameter count", command)
            }
        };
    }
    ///Extracts the super- and subscript from the command name
    fn extract_super_and_subscript(mut command: Vec<char>) -> (Vec<char>, Option<(Vec<char>, BracketType)>, Option<(Vec<char>, BracketType)>){
        let mut superscript = None;
        let mut subscript = None;
        let mut command_offset = 0;
        if !command.contains(&'^') && !command.contains(&'_'){
            return (command, None, None)
        }
        while command[command_offset] != '^' && command[command_offset] != '_'{
            command_offset += 1;
        }
        let (command, super_and_subscript_part) = command.split_at(command_offset);
        let (command, mut super_and_subscript_part) = (command.to_vec(), super_and_subscript_part.to_vec());
        let mut is_superscript = super_and_subscript_part.remove(0) == '^';
        let mut new_buffer = vec!(super_and_subscript_part.remove(0));
        let mut bracket_type = if BracketType::is_opening_bracket(new_buffer[0]) {BracketType::get_opening_bracket_type(new_buffer[0])}else {BracketType::None};
        if BracketType::is_opening_bracket(new_buffer[0]){
            let mut depth = 1;
            while depth != 0{
                let next = super_and_subscript_part.remove(0);
                if BracketType::is_opening_bracket(next){
                    depth += 1;
                }
                else if BracketType::is_closing_bracket(next){
                    depth -= 1;
                }
                if depth != 0 {
                    new_buffer.push(next);
                }
            }
        }
        if BracketType::is_opening_bracket(new_buffer[0]) && BracketType::is_closing_bracket(new_buffer[new_buffer.len()-1]){
            new_buffer.remove(new_buffer.len()-1);
            new_buffer.remove(0);
        }
        if is_superscript{superscript = Some((new_buffer, bracket_type))}else{subscript = Some((new_buffer, bracket_type))}
        if super_and_subscript_part.len() != 0{
            let mut is_superscript = super_and_subscript_part.remove(0) == '^';
            let mut new_buffer = vec!(super_and_subscript_part.remove(0));
            let mut bracket_type = if BracketType::is_opening_bracket(new_buffer[0]) {BracketType::get_opening_bracket_type(new_buffer[0])}else {BracketType::None};
            if BracketType::is_opening_bracket(new_buffer[0]){
                let mut depth = 1;
                while depth != 0{
                    let next = super_and_subscript_part.remove(0);
                    if BracketType::is_opening_bracket(next){
                        depth += 1;
                    }
                    else if BracketType::is_closing_bracket(next){
                        depth -= 1;
                    }
                    //if depth != 0 {
                        new_buffer.push(next);
                    //}
                }
            }
            if BracketType::is_opening_bracket(new_buffer[0]) && BracketType::is_closing_bracket(new_buffer[new_buffer.len()-1]){
                new_buffer.remove(new_buffer.len()-1);
                new_buffer.remove(0);
            }
            if is_superscript{superscript = Some((new_buffer, bracket_type))}else{subscript = Some((new_buffer, bracket_type))}
        }
        return (command, superscript, subscript)
    }
    //Gets the positions of all 'd' chars that are integral delimiters and replace them by a parameter
    fn convert_integral_delimiters_to_parameters(mut latex: Vec<char>, allow_d_in_integral: bool) -> Vec<char>{
        let mut new_latex = vec!();
        let mut integral_found = false;
        let mut is_parsing_command = false;
        let mut started_variable_parsing = false;
        let mut just_started_variable_parsing = false;
        let mut variable_of_integration = vec!();
        if allow_d_in_integral{
            let mut depth = 1;
            for (i, char) in latex.into_iter().enumerate(){
                if new_latex.len() > 3{
                    if !integral_found && new_latex[i-4] == '\\' && new_latex[i-3] == 'i' && new_latex[i-2] == 'n' && new_latex[i-1] == 't' && !char.is_alphanumeric(){
                        if !BracketType::is_opening_bracket(char){
                            panic!("If d's are allowed within integrals, the integrand has to be surrounded by brackets")
                        }
                        integral_found = true;
                        is_parsing_command = false;
                        new_latex.push(char)
                    } else if !started_variable_parsing && char == '\\'{
                        is_parsing_command = true;
                        new_latex.push(char)
                    } else if !started_variable_parsing && is_parsing_command && !char.is_alphanumeric(){
                        is_parsing_command = false;
                        new_latex.push(char)
                    } else if integral_found && !started_variable_parsing && BracketType::is_opening_bracket(char){
                        depth += 1;
                        new_latex.push(char);
                    } else if integral_found && !started_variable_parsing && BracketType::is_closing_bracket(char){
                        depth -= 1;
                        new_latex.push(char);
                    } else if integral_found && !started_variable_parsing && depth == 0{
                        if char != 'd'{
                            panic!("Integral without variable found")
                        }
                        started_variable_parsing = true;
                        just_started_variable_parsing = true;
                        is_parsing_command = false;
                    } else if started_variable_parsing{
                        if just_started_variable_parsing{
                            variable_of_integration.push(char);
                            just_started_variable_parsing = false;
                        }
                        else if depth == 0 && !is_parsing_command{
                            let already_has_brackets = BracketType::is_opening_bracket(variable_of_integration[0]);
                            if !already_has_brackets {
                                new_latex.push('{')
                            };
                            new_latex.append(&mut variable_of_integration);
                            if !already_has_brackets {
                                new_latex.push('}');
                            }
                            started_variable_parsing = false;
                            integral_found = false;
                        }
                        else {
                            variable_of_integration.push(char)
                        }
                        if char == '\\'{
                            is_parsing_command = true;
                        }else if is_parsing_command && !char.is_alphanumeric(){
                            is_parsing_command = false;
                        }
                    }
                    else{
                        new_latex.push(char)
                    }
                }
                else{
                    new_latex.push(char)
                }
            }
        }
        else{
            let mut delimiter_start = false;
            let mut delimiter_just_started = false;
            let mut depth = 0;
            for (i, char) in latex.into_iter().enumerate(){
                if new_latex.len() > 3{
                    if !integral_found && new_latex[i-4] == '\\' && new_latex[i-3] == 'i' && new_latex[i-2] == 'n' && new_latex[i-1] == 't' && !char.is_alphanumeric() && depth == 0{
                        integral_found = true;
                        is_parsing_command = false;
                        new_latex.push('i');
                        new_latex.push('n');
                        new_latex.push('t');
                        new_latex.push(char)
                    } else if !delimiter_start && char == '\\' && depth == 0{
                        is_parsing_command = true;
                        new_latex.push(char)
                    } else if !delimiter_start && is_parsing_command && !char.is_alphanumeric() && depth == 0{
                        is_parsing_command = false;
                        new_latex.push(char)
                    }
                    else if !delimiter_start && integral_found && !is_parsing_command && char == 'd' && depth == 0{
                        delimiter_start = true;
                        delimiter_just_started = true;
                        is_parsing_command = false;
                    } else if delimiter_start{
                        //Always push the first char
                        if delimiter_just_started{
                            delimiter_just_started = false;
                            variable_of_integration.push(char);
                        }
                        else{
                            if depth == 0 && !is_parsing_command{
                                delimiter_start = false;
                                let already_has_brackets = BracketType::is_opening_bracket(variable_of_integration[0]);
                                if !already_has_brackets {
                                    new_latex.push('{')
                                };
                                new_latex.append(&mut variable_of_integration);
                                if !already_has_brackets {
                                    new_latex.push('}');
                                }
                                new_latex.push(char);
                                started_variable_parsing = false;
                                integral_found = false;
                            }
                            else{ variable_of_integration.push(char)}
                        }
                    }
                    else{
                        new_latex.push(char)
                    }
                }
                else{
                    new_latex.push(char)
                }
                if BracketType::is_opening_bracket(char){
                    depth += 1;
                }
                else if BracketType::is_closing_bracket(char){
                    depth -= 1;
                }
                if char == '\\'{
                    is_parsing_command = true;
                }
                else if is_parsing_command && !char.is_alphanumeric(){
                    is_parsing_command = false;
                }
            }
        }
        if variable_of_integration.len() > 0{
            let is_already_in_brackets = BracketType::is_opening_bracket(variable_of_integration[0]);
            if !is_already_in_brackets {
                new_latex.push('{');
            }
            new_latex.append(&mut variable_of_integration);
            if !is_already_in_brackets {
                new_latex.push('}');
            }
        }
        return new_latex;
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
