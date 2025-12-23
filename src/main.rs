use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::cell::RefCell;

// --- DATA TYPES ---
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
                if n.fract() == 0.0 { write!(f, "{}", *n as i64) } else { write!(f, "{}", n) }
            }
            Value::Text(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", if *b { "yes" } else { "no" }),
            Value::Nothing => write!(f, ""),
            Value::List(items) => {
                let strs: Vec<String> = items.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", strs.join(" "))
            }
            Value::Function(_, _) => write!(f, "<function>"),
        }
    }
}

// --- STATEMENTS ---
#[derive(Debug, Clone, PartialEq)]
enum Statement {
    Print(Expr),
    Assign(String, Expr),
    AugAssign(String, String, Expr),
    IncDec(String, String),
    If(Expr, Vec<Statement>, Vec<Statement>),
    While(Expr, Vec<Statement>),
    For(String, Expr, Vec<Statement>),
    FunctionDef(String, Vec<String>, Vec<Statement>),
    FunctionCall(String, Vec<Expr>),
    Return(Expr),
    Input(String),
}

// --- EXPRESSIONS ---
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

// --- INTERPRETER ---
struct InterpreterRef {
    vars: HashMap<String, Value>,
    rng_state: RefCell<u64>,
}

impl InterpreterRef {
    fn new() -> Self {
        let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        let hasher = RandomState::new().build_hasher();
        seed ^= hasher.finish();
        if seed == 0 { seed = 123456789; }

        Self {
            vars: HashMap::new(),
            rng_state: RefCell::new(seed),
        }
    }

    fn next_random(&self, max: u64) -> f64 {
        if max == 0 { return 0.0; }
        let mut x = *self.rng_state.borrow();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *self.rng_state.borrow_mut() = x;
        (x % max) as f64
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
                    if (idx as usize) < items.len() { return items[idx as usize].clone(); }
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
                "<=" => Value::Bool(l <= r),
                _ => Value::Nothing,
            },
            (Value::Text(l), Value::Text(r)) if op == "+" => Value::Text(format!("{}{}", l, r)),
            (Value::Text(l), Value::Text(r)) if op == "==" => Value::Bool(l == r),
            (Value::Text(l), Value::Number(r)) if op == "+" => Value::Text(format!("{}{}", l, r)),
            (Value::Number(l), Value::Text(r)) if op == "+" => Value::Text(format!("{}{}", l, r)),
            (Value::Bool(l), Value::Bool(r)) if op == "==" => Value::Bool(l == r),
            _ => Value::Nothing,
        }
    }

    fn call_function(&self, name: &str, args: Vec<Value>) -> Value {
        match name {
            "?=" => {
                if let Some(Value::Number(max_float)) = args.get(0) {
                    return Value::Number(self.next_random(*max_float as u64));
                }
                Value::Number(0.0)
            },
            "#" => {
                if let Some(Value::List(i)) = args.get(0) { return Value::Number(i.len() as f64); }
                if let Some(Value::Text(s)) = args.get(0) { return Value::Number(s.len() as f64); }
                Value::Number(0.0)
            }
            "$" => {
                if let Some(v) = args.get(0) { return Value::Text(format!("{}", v)); }
                Value::Text(String::new())
            }
            "~" => {
                if let Some(Value::Text(s)) = args.get(0) {
                    return s.parse::<f64>().map(Value::Number).unwrap_or(Value::Number(0.0));
                }
                if let Some(Value::Number(n)) = args.get(0) { return Value::Number(*n); }
                Value::Number(0.0)
            }
            "^" => {
                if let (Some(Value::List(items)), Some(val)) = (args.get(0), args.get(1)) {
                    let mut new_list = items.clone();
                    new_list.push(val.clone());
                    return Value::List(new_list);
                }
                Value::Nothing
            },
            "v" => {
                if let Some(Value::List(items)) = args.get(0) {
                    if !items.is_empty() {
                        let mut new_list = items.clone();
                        new_list.pop();
                        return Value::List(new_list);
                    }
                }
                Value::Nothing
            },
            "&" => {
                if let (Some(Value::List(items)), Some(Value::Text(sep))) = (args.get(0), args.get(1)) {
                    let strs: Vec<String> = items.iter().map(|v| format!("{}", v)).collect();
                    return Value::Text(strs.join(sep));
                }
                Value::Text(String::new())
            },
            "|" => {
                if let (Some(Value::Text(s)), Some(Value::Text(sep))) = (args.get(0), args.get(1)) {
                    let parts: Vec<Value> = s.split(sep.as_str()).map(|p| Value::Text(p.to_string())).collect();
                    return Value::List(parts);
                }
                Value::List(vec![])
            },
            "!" => {
                if let Some(Value::Bool(b)) = args.get(0) { return Value::Bool(!b); }
                Value::Bool(false)
            },
            _ => {
                if let Some(Value::Function(params, body)) = self.vars.get(name) {
                    let mut new_interp = InterpreterRef::new();
                    new_interp.vars = self.vars.clone();
                    for (i, param) in params.iter().enumerate() {
                        if let Some(arg) = args.get(i) {
                            new_interp.vars.insert(param.clone(), arg.clone());
                        }
                    }
                    return new_interp.run_function(body);
                }
                Value::Nothing
            }
        }
    }

    fn run_function(&mut self, body: &Vec<Statement>) -> Value {
        for stmt in body {
            if let Statement::Return(expr) = stmt {
                return self.eval_expr(expr);
            }
            self.execute(stmt);
        }
        Value::Nothing
    }

    fn execute(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Print(expr) => {
                let val = self.eval_expr(expr);
                if val != Value::Nothing {
                    println!("{}", val);
                    io::stdout().flush().unwrap();
                }
            }
            Statement::Assign(name, expr) => {
                let val = self.eval_expr(expr);
                self.vars.insert(name.clone(), val);
            }
            Statement::AugAssign(name, op, expr) => {
                let current_val = self.vars.get(name).cloned().unwrap_or(Value::Number(0.0));
                let operand = self.eval_expr(expr);
                let new_val = self.apply_op(&current_val, op, &operand);
                self.vars.insert(name.clone(), new_val);
            }
            Statement::IncDec(name, op) => {
                let current_val = self.vars.get(name).cloned().unwrap_or(Value::Number(0.0));
                let one = Value::Number(1.0);
                let new_val = match op.as_str() {
                    "++" => self.apply_op(&current_val, "+", &one),
                    "--" => self.apply_op(&current_val, "-", &one),
                    _ => current_val
                };
                self.vars.insert(name.clone(), new_val);
            }
            Statement::If(cond, then_block, else_block) => {
                if matches!(self.eval_expr(cond), Value::Bool(true)) {
                    for s in then_block { self.execute(s); }
                } else {
                    for s in else_block { self.execute(s); }
                }
            }
            Statement::While(cond, body) => {
                while matches!(self.eval_expr(cond), Value::Bool(true)) {
                    for s in body { self.execute(s); }
                }
            }
            Statement::For(var, list_expr, body) => {
                if let Value::List(items) = self.eval_expr(list_expr) {
                    for item in items {
                        self.vars.insert(var.clone(), item);
                        for s in body { self.execute(s); }
                    }
                }
            }
            Statement::FunctionDef(name, params, body) => {
                self.vars.insert(name.clone(), Value::Function(params.clone(), body.clone()));
            }
            Statement::FunctionCall(name, args) => {
                let vals: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect();
                self.call_function(name, vals);
            }
            Statement::Input(var_name) => {
                print!("? ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
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

// --- PARSER ---

fn parse(code: &str) -> Vec<Statement> {
    let lines: Vec<&str> = code.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    let mut idx = 0;
    parse_lines(&lines, &mut idx)
}

fn parse_lines(lines: &[&str], current: &mut usize) -> Vec<Statement> {
    let mut statements = Vec::new();
    while *current < lines.len() {
        let line = lines[*current];

        if line == "<-" || line == "<=" {
            *current += 1;
            return statements;
        }

        if line.starts_with('"') {
            if let Some(stmt) = parse_simple_statement(line) {
                statements.push(stmt);
            }
            *current += 1;
            continue;
        }

        if line.contains("=>") {
            let parts: Vec<&str> = line.split("=>").collect();
            let sig = parts[0].trim();
            if let Some(paren_idx) = sig.find('(') {
                let name = sig[..paren_idx].trim().to_string();
                let params_str = sig[paren_idx + 1..].trim_end_matches(')').trim();
                let params: Vec<String> = if params_str.is_empty() { vec![] }
                else { params_str.split_whitespace().map(|s| s.to_string()).collect() };

                *current += 1;
                let body = parse_lines(lines, current);
                statements.push(Statement::FunctionDef(name, params, body));
                continue;
            }
        }

        if line.starts_with("@ ") {
            let cond = parse_expr(line[2..].trim());
            *current += 1;
            let body = parse_lines(lines, current);
            statements.push(Statement::While(cond, body));
            continue;
        }

        if line.starts_with("?") {
            let content = line[1..].trim();
            if is_condition(content) {
                let cond = parse_expr(content);
                *current += 1;
                let body = parse_lines(lines, current);
                statements.push(Statement::If(cond, body, vec![]));
                continue;
            }
        }

        if let Some(stmt) = parse_simple_statement(line) {
            statements.push(stmt);
        }
        *current += 1;
    }
    statements
}

fn is_condition(s: &str) -> bool {
    let ops = ["==", "!=", ">", "<", "+", "-", "*", "/", "(", ")", "yes", "no", "true", "false", "!"];
    for op in &ops { if s.contains(op) { return true; } }
    if s.parse::<f64>().is_ok() { return true; }
    false
}

fn parse_simple_statement(line: &str) -> Option<Statement> {
    let line = line.trim();

    if line.starts_with('"') && line.ends_with('"') && line.len() >= 2 {
        return Some(Statement::Print(Expr::Text(line[1..line.len()-1].to_string())));
    }

    if line.starts_with("?") {
        let content = line[1..].trim();
        if !is_condition(content) {
            return Some(Statement::Input(content.to_string()));
        }
    }

    if line.starts_with("-> ") {
        return Some(Statement::Return(parse_expr(line[3..].trim())));
    }

    if line.ends_with("++") {
        let var = line[..line.len()-2].trim().to_string();
        return Some(Statement::IncDec(var, "++".to_string()));
    }
    if line.ends_with("--") {
        let var = line[..line.len()-2].trim().to_string();
        return Some(Statement::IncDec(var, "--".to_string()));
    }

    for op in &["+=", "-=", "*=", "/="] {
        if let Some(idx) = line.find(op) {
            let var = line[..idx].trim().to_string();
            let expr = parse_expr(line[idx+2..].trim());
            let math_op = op[..1].to_string();
            return Some(Statement::AugAssign(var, math_op, expr));
        }
    }

    if let Some(eq_idx) = find_assign_op(line) {
        let var = line[..eq_idx].trim().to_string();
        let expr = parse_expr(line[eq_idx+1..].trim());
        return Some(Statement::Assign(var, expr));
    }

    if line.contains('(') && line.ends_with(')') {
        if let Expr::FunctionCall(name, args) = parse_expr(line) {
            return Some(Statement::FunctionCall(name, args));
        }
    }

    if !line.starts_with("=>") && !line.starts_with("<=") && !line.starts_with("<-") {
        if let Ok(num) = line.parse::<f64>() {
            return Some(Statement::Print(Expr::Number(num)));
        } else {
            return Some(Statement::Print(parse_expr(line)));
        }
    }

    None
}

fn find_assign_op(s: &str) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '=' {
            let prev = if i > 0 { chars[i-1] } else { ' ' };
            let next = if i+1 < chars.len() { chars[i+1] } else { ' ' };
            if prev != '>' && prev != '<' && prev != '!' && prev != '='
                && prev != '+' && prev != '-' && prev != '*' && prev != '/'
                && next != '=' {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn parse_expr(s: &str) -> Expr {
    let s = s.trim();

    if s.starts_with('!') && !s.starts_with("!=") {
        let operand = parse_expr(&s[1..]);
        return Expr::FunctionCall("!".to_string(), vec![operand]);
    }

    let ops = ["==", "!=", ">=", "<=", ">", "<", "+", "-", "*", "/", "%"];
    for op in &ops {
        if let Some(idx) = find_op_outside_parens(s, op) {
            let left = parse_expr(&s[..idx]);
            let right = parse_expr(&s[idx + op.len()..]);
            return Expr::BinaryOp(Box::new(left), op.to_string(), Box::new(right));
        }
    }

    if s.starts_with('[') && s.ends_with(']') {
        let content = s[1..s.len()-1].trim();
        if content.is_empty() { return Expr::List(vec![]); }
        let items: Vec<Expr> = split_args_outside_parens(content).into_iter().map(|i| parse_expr(&i)).collect();
        return Expr::List(items);
    }

    if s.ends_with(']') {
        if let Some(idx) = find_matching_open(s, '[', ']') {
            let list = parse_expr(&s[..idx]);
            let index = parse_expr(&s[idx+1..s.len()-1]);
            return Expr::Index(Box::new(list), Box::new(index));
        }
    }

    if s.ends_with(')') {
        if let Some(idx) = find_matching_open(s, '(', ')') {
            let name = s[..idx].trim().to_string();
            let args_str = s[idx+1..s.len()-1].trim();
            let args: Vec<Expr> = if args_str.is_empty() { vec![] }
            else { split_args_outside_parens(args_str).into_iter().map(|i| parse_expr(&i)).collect() };
            return Expr::FunctionCall(name, args);
        }
    }

    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        return Expr::Text(s[1..s.len()-1].to_string());
    }
    if let Ok(n) = s.parse::<f64>() { return Expr::Number(n); }
    if s == "yes" || s == "true" { return Expr::Bool(true); }
    if s == "no" || s == "false" { return Expr::Bool(false); }

    Expr::Variable(s.to_string())
}

fn find_op_outside_parens(s: &str, op: &str) -> Option<usize> {
    let s_chars: Vec<char> = s.chars().collect();
    let op_chars: Vec<char> = op.chars().collect();
    let mut balance = 0;
    let mut i = 0;
    while i < s_chars.len() {
        if s_chars[i] == '(' || s_chars[i] == '[' { balance += 1; }
        else if s_chars[i] == ')' || s_chars[i] == ']' { balance -= 1; }
        if balance == 0 && s_chars[i..].starts_with(&op_chars) {
            if op == "-" && i == 0 { }
            else { return Some(i); }
        }
        i += 1;
    }
    None
}

fn find_matching_open(s: &str, open: char, close: char) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    let mut balance = 1;
    let mut i = chars.len() - 2;
    while i > 0 {
        if chars[i] == close { balance += 1; }
        if chars[i] == open {
            balance -= 1;
            if balance == 0 { return Some(i); }
        }
        i -= 1;
    }
    if i == 0 && chars[0] == open && balance == 0 { return Some(0); }
    None
}

fn split_args_outside_parens(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut cur = String::new();
    let mut balance = 0;
    for c in s.chars() {
        if c == '(' || c == '[' { balance += 1; }
        else if c == ')' || c == ']' { balance -= 1; }
        if balance == 0 && c.is_whitespace() {
            if !cur.is_empty() { args.push(cur.clone()); cur.clear(); }
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty() { args.push(cur); }
    args
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match fs::read_to_string(&args[1]) {
            Ok(code) => {
                let stmts = parse(&code);
                let mut interp = InterpreterRef::new();
                for s in stmts { interp.execute(&s); }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        println!("Lazy Lang REPL");
        let mut interp = InterpreterRef::new();
        let mut buf = String::new();
        loop {
            print!("lazy> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "exit" { break; }
            if input == "run" {
                let stmts = parse(&buf);
                for s in stmts { interp.execute(&s); }
                buf.clear();
            } else {
                buf.push_str(input);
                buf.push('\n');
            }
        }
    }
}