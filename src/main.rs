use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Number(f64),
    Text(String),
    Bool(bool),
    Nothing,
    List(Vec<Value>),
    Function(Vec<String>, Vec<Statement>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Text(s) => write!(f, "{}", s),
            Value::Bool(b) => {
                if *b {
                    write!(f, "yes")
                } else {
                    write!(f, "no")
                }
            }
            Value::Nothing => write!(f, "nothing"),
            Value::List(items) => {
                let strs: Vec<String> = items.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", strs.join(" "))
            }
            Value::Function(_, _) => write!(f, "<function>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Statement {
    Print(Expr),
    Assign(String, Expr),
    If(Expr, Vec<Statement>, Vec<Statement>),
    While(Expr, Vec<Statement>),
    For(String, Expr, Vec<Statement>),
    FunctionDef(String, Vec<String>, Vec<Statement>),
    FunctionCall(String, Vec<Expr>),
    Return(Expr),
    Input(String),
}

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Number(f64),
    Text(String),
    Bool(bool),
    Variable(String),
    List(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
}

struct Interpreter {
    vars: HashMap<String, Value>,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    fn eval_expr(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            Expr::Text(s) => Value::Text(s.clone()),
            Expr::Bool(b) => Value::Bool(*b),
            Expr::Variable(name) => self.vars.get(name).cloned().unwrap_or(Value::Nothing),
            Expr::List(items) => {
                let vals: Vec<Value> = items.iter().map(|e| self.eval_expr(e)).collect();
                Value::List(vals)
            }
            Expr::Index(list_expr, index_expr) => {
                let list_val = self.eval_expr(list_expr);
                let index_val = self.eval_expr(index_expr);

                if let (Value::List(items), Value::Number(idx)) = (list_val, index_val) {
                    let i = idx as usize;
                    if i < items.len() {
                        return items[i].clone();
                    }
                }
                Value::Nothing
            }
            Expr::BinaryOp(left, op, right) => {
                let l = self.eval_expr(left);
                let r = self.eval_expr(right);
                self.apply_op(&l, op, &r)
            }
            Expr::FunctionCall(name, args) => {
                let arg_vals: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect();
                self.call_function(name, arg_vals)
            }
        }
    }

    fn apply_op(&self, left: &Value, op: &str, right: &Value) -> Value {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => match op {
                "+" => Value::Number(l + r),
                "-" => Value::Number(l - r),
                "*" => Value::Number(l * r),
                "/" => Value::Number(l / r),
                "%" => Value::Number(l % r),
                ">" => Value::Bool(l > r),
                "<" => Value::Bool(l < r),
                "==" => Value::Bool((l - r).abs() < f64::EPSILON),
                "!=" => Value::Bool((l - r).abs() >= f64::EPSILON),
                ">=" => Value::Bool(l >= r),
                _ => Value::Nothing,
            },
            (Value::Text(l), Value::Text(r)) if op == "+" => {
                Value::Text(format!("{}{}", l, r))
            }
            (Value::Text(l), Value::Text(r)) if op == "==" => {
                Value::Bool(l == r)
            }
            (Value::List(l), Value::List(r)) if op == "+" => {
                let mut combined = l.clone();
                combined.extend(r.clone());
                Value::List(combined)
            }
            (Value::Bool(l), Value::Bool(r)) if op == "==" => {
                Value::Bool(l == r)
            }
            _ => Value::Nothing,
        }
    }

    fn call_function(&self, name: &str, args: Vec<Value>) -> Value {
        // Built-in functions (operators disguised as names)
        match name {
            "#" => { // len - # looks like counting
                if let Some(Value::List(items)) = args.get(0) {
                    return Value::Number(items.len() as f64);
                }
                if let Some(Value::Text(s)) = args.get(0) {
                    return Value::Number(s.len() as f64);
                }
                return Value::Number(0.0);
            }
            "$" => { // to_string - $ looks like S
                if let Some(val) = args.get(0) {
                    return Value::Text(format!("{}", val));
                }
                return Value::Text(String::new());
            }
            "~" => { // to_number - ~ looks wavy like numbers
                if let Some(Value::Text(s)) = args.get(0) {
                    if let Ok(n) = s.parse::<f64>() {
                        return Value::Number(n);
                    }
                }
                if let Some(Value::Number(n)) = args.get(0) {
                    return Value::Number(*n);
                }
                return Value::Number(0.0);
            }
            "^" => { // push - ^ points up (add to top)
                if let (Some(Value::List(items)), Some(val)) = (args.get(0), args.get(1)) {
                    let mut new_list = items.clone();
                    new_list.push(val.clone());
                    return Value::List(new_list);
                }
                return Value::Nothing;
            }
            "v" => { // pop - v points down (remove from top)
                if let Some(Value::List(items)) = args.get(0) {
                    if !items.is_empty() {
                        let mut new_list = items.clone();
                        new_list.pop();
                        return Value::List(new_list);
                    }
                }
                return Value::Nothing;
            }
            "&" => { // join list to string - & connects things
                if let (Some(Value::List(items)), Some(Value::Text(sep))) = (args.get(0), args.get(1)) {
                    let strs: Vec<String> = items.iter().map(|v| format!("{}", v)).collect();
                    return Value::Text(strs.join(sep));
                }
                return Value::Text(String::new());
            }
            "|" => { // split string to list - | divides things
                if let (Some(Value::Text(s)), Some(Value::Text(sep))) = (args.get(0), args.get(1)) {
                    let parts: Vec<Value> = s.split(sep.as_str())
                        .map(|p| Value::Text(p.to_string()))
                        .collect();
                    return Value::List(parts);
                }
                return Value::List(vec![]);
            }
            "!" => { // not - ! means opposite
                if let Some(Value::Bool(b)) = args.get(0) {
                    return Value::Bool(!b);
                }
                return Value::Bool(false);
            }
            "%" => { // random number 0-n
                if let Some(Value::Number(max)) = args.get(0) {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
                    let random = (seed % (*max as u128)) as f64;
                    return Value::Number(random);
                }
                return Value::Number(0.0);
            }
            _ => {}
        }

        // User-defined functions
        if let Some(Value::Function(params, body)) = self.vars.get(name) {
            let mut new_interp = self.clone();
            for (i, param) in params.iter().enumerate() {
                if let Some(arg) = args.get(i) {
                    new_interp.vars.insert(param.clone(), arg.clone());
                }
            }
            for stmt in body {
                if let Statement::Return(expr) = stmt {
                    return new_interp.eval_expr(expr);
                }
                new_interp.execute(stmt);
            }
        }
        Value::Nothing
    }

    fn execute(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Print(expr) => {
                println!("{}", self.eval_expr(expr));
            }
            Statement::Assign(name, expr) => {
                let val = self.eval_expr(expr);
                self.vars.insert(name.clone(), val);
            }
            Statement::If(cond, then_block, else_block) => {
                let val = self.eval_expr(cond);
                if matches!(val, Value::Bool(true)) {
                    for s in then_block {
                        self.execute(s);
                    }
                } else {
                    for s in else_block {
                        self.execute(s);
                    }
                }
            }
            Statement::While(cond, body) => {
                while matches!(self.eval_expr(cond), Value::Bool(true)) {
                    for s in body {
                        self.execute(s);
                    }
                }
            }
            Statement::For(var, list_expr, body) => {
                let list_val = self.eval_expr(list_expr);
                if let Value::List(items) = list_val {
                    for item in items {
                        self.vars.insert(var.clone(), item);
                        for s in body {
                            self.execute(s);
                        }
                    }
                }
            }
            Statement::FunctionDef(name, params, body) => {
                self.vars.insert(
                    name.clone(),
                    Value::Function(params.clone(), body.clone()),
                );
            }
            Statement::FunctionCall(name, args) => {
                let arg_vals: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect();
                self.call_function(name, arg_vals);
            }
            Statement::Input(var_name) => {
                print!("? ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                // Try to parse as number, otherwise store as text
                if let Ok(num) = input.parse::<f64>() {
                    self.vars.insert(var_name.clone(), Value::Number(num));
                } else {
                    self.vars.insert(var_name.clone(), Value::Text(input.to_string()));
                }
            }
            Statement::Return(_) => {}
        }
    }
}

impl Clone for Interpreter {
    fn clone(&self) -> Self {
        Self {
            vars: self.vars.clone(),
        }
    }
}

fn parse(code: &str) -> Vec<Statement> {
    let mut statements = Vec::new();
    let lines: Vec<&str> = code.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        // Function definition: name(param1 param2) =>
        if line.contains("=>") {
            let parts: Vec<&str> = line.split("=>").collect();
            let sig = parts[0].trim();

            if let Some(paren_idx) = sig.find('(') {
                let name = sig[..paren_idx].trim().to_string();
                let params_str = sig[paren_idx + 1..].trim_end_matches(')').trim();
                let params: Vec<String> = if params_str.is_empty() {
                    vec![]
                } else {
                    params_str.split_whitespace().map(|s| s.to_string()).collect()
                };

                let mut body = Vec::new();
                i += 1;

                while i < lines.len() && lines[i].trim() != "<=" {
                    if let Some(stmt) = parse_statement(lines[i]) {
                        body.push(stmt);
                    }
                    i += 1;
                }

                statements.push(Statement::FunctionDef(name, params, body));
            }
        } else if let Some(stmt) = parse_statement(line) {
            statements.push(stmt);
        }

        i += 1;
    }

    statements
}

fn parse_statement(line: &str) -> Option<Statement> {
    let line = line.trim();

    // Skip end markers
    if line == "<=" {
        return None;
    }

    // Input: ? varname
    if line.starts_with("? ") && !line.contains('>') && !line.contains('<') && !line.contains("==") {
        let var_name = line[2..].trim().to_string();
        return Some(Statement::Input(var_name));
    }

    // For loop: >> item items
    if line.starts_with(">> ") {
        let parts: Vec<&str> = line[3..].trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let var = parts[0].to_string();
            let list_name = parts[1].to_string();
            return Some(Statement::For(var, Expr::Variable(list_name), vec![]));
        }
    }

    // Print (just text or variable name)
    if !line.contains('=') && !line.contains('(') && !line.contains('>') && !line.contains('[') && !line.starts_with('?') && !line.starts_with('@') {
        if line.starts_with('"') && line.ends_with('"') {
            return Some(Statement::Print(Expr::Text(line[1..line.len()-1].to_string())));
        } else if let Ok(num) = line.parse::<f64>() {
            return Some(Statement::Print(Expr::Number(num)));
        } else {
            return Some(Statement::Print(Expr::Variable(line.to_string())));
        }
    }

    // Assignment: x = value
    if line.contains('=') && !line.contains("==") && !line.contains("=>") && !line.contains("<=") && !line.contains(">=") {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            let var = parts[0].trim().to_string();
            let expr = parse_expr(parts[1].trim());
            return Some(Statement::Assign(var, expr));
        }
    }

    // Conditional: ? x > 5
    if line.starts_with("? ") && (line.contains('>') || line.contains('<') || line.contains("==") || line.contains("!=")) {
        let cond_str = line[2..].trim();
        let cond = parse_expr(cond_str);
        return Some(Statement::If(cond, vec![], vec![]));
    }

    // Loop: @ x < 10
    if line.starts_with("@ ") {
        let cond_str = line[2..].trim();
        let cond = parse_expr(cond_str);
        return Some(Statement::While(cond, vec![]));
    }

    // Return: -> value
    if line.starts_with("-> ") {
        let expr_str = line[3..].trim();
        let expr = parse_expr(expr_str);
        return Some(Statement::Return(expr));
    }

    // Function call: func(arg1 arg2)
    if line.contains('(') && line.contains(')') {
        let paren_idx = line.find('(')?;
        let name = line[..paren_idx].trim().to_string();
        let args_str = line[paren_idx + 1..].trim_end_matches(')').trim();
        let args: Vec<Expr> = if args_str.is_empty() {
            vec![]
        } else {
            args_str.split_whitespace().map(|s| parse_expr(s)).collect()
        };
        return Some(Statement::FunctionCall(name, args));
    }

    None
}

fn parse_expr(s: &str) -> Expr {
    let s = s.trim();

    // List literal: [1 2 3]
    if s.starts_with('[') && s.ends_with(']') {
        let content = s[1..s.len()-1].trim();
        if content.is_empty() {
            return Expr::List(vec![]);
        }
        let items: Vec<Expr> = content.split_whitespace().map(|item| parse_expr(item)).collect();
        return Expr::List(items);
    }

    // List indexing: mylist[0]
    if s.contains('[') && s.contains(']') && !s.starts_with('[') {
        let bracket_idx = s.find('[').unwrap();
        let list_expr = parse_expr(&s[..bracket_idx]);
        let index_str = &s[bracket_idx + 1..s.len()-1];
        let index_expr = parse_expr(index_str);
        return Expr::Index(Box::new(list_expr), Box::new(index_expr));
    }

    // String literal
    if s.starts_with('"') && s.ends_with('"') {
        return Expr::Text(s[1..s.len()-1].to_string());
    }

    // Number
    if let Ok(num) = s.parse::<f64>() {
        return Expr::Number(num);
    }

    // Boolean
    if s == "yes" || s == "true" {
        return Expr::Bool(true);
    }
    if s == "no" || s == "false" {
        return Expr::Bool(false);
    }

    // Binary operations
    for op in &["==", "!=", ">=", "<=", ">", "<", "+", "-", "*", "/", "%"] {
        if let Some(idx) = s.find(op) {
            let left = parse_expr(&s[..idx]);
            let right = parse_expr(&s[idx + op.len()..]);
            return Expr::BinaryOp(Box::new(left), op.to_string(), Box::new(right));
        }
    }

    // Function call in expression
    if s.contains('(') && s.contains(')') {
        let paren_idx = s.find('(').unwrap();
        let name = s[..paren_idx].trim().to_string();
        let args_str = s[paren_idx + 1..].trim_end_matches(')').trim();
        let args: Vec<Expr> = if args_str.is_empty() {
            vec![]
        } else {
            args_str.split_whitespace().map(|s| parse_expr(s)).collect()
        };
        return Expr::FunctionCall(name, args);
    }

    // Variable
    Expr::Variable(s.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // File mode
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(code) => {
                let statements = parse(&code);
                let mut interpreter = Interpreter::new();
                for stmt in statements {
                    interpreter.execute(&stmt);
                }
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", filename, e);
                std::process::exit(1);
            }
        }
    } else {
        // REPL mode
        println!("Lazy Programming Language");
        println!("Type 'exit' to quit, 'run' to execute code\n");

        let mut code_buffer = String::new();
        let mut interpreter = Interpreter::new();

        loop {
            print!("lazy> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "exit" {
                break;
            }

            if input == "run" {
                let statements = parse(&code_buffer);
                for stmt in statements {
                    interpreter.execute(&stmt);
                }
                code_buffer.clear();
            } else {
                code_buffer.push_str(input);
                code_buffer.push('\n');
            }
        }
    }
}