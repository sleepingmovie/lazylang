use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

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
                let strs: Vec<String> = items.iter().map(|v| {
                    match v {
                        Value::Text(t) => format!("\"{}\"", t), // Quote strings in lists
                        _ => format!("{}", v)
                    }
                }).collect();
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
struct Interpreter {
    scopes: Vec<HashMap<String, Value>>,
    rng_state: u64,
}

impl Interpreter {
    fn new() -> Self {
        let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        let hasher = RandomState::new().build_hasher();
        seed ^= hasher.finish();
        if seed == 0 { seed = 123456789; }

        Self {
            scopes: vec![HashMap::new()],
            rng_state: seed,
        }
    }

    fn get_var(&self, name: &str) -> Value {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return val.clone();
            }
        }
        Value::Nothing
    }

    fn set_var(&mut self, name: &str, val: Value) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), val);
                return;
            }
        }
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), val);
        }
    }

    fn execute(&mut self, stmt: &Statement) -> Option<Value> {
        match stmt {
            Statement::Print(expr) => {
                let val = self.eval_expr(expr);
                if val != Value::Nothing {
                    println!("{}", val);
                    io::stdout().flush().unwrap();
                }
                None
            }
            Statement::Assign(name, expr) => {
                let val = self.eval_expr(expr);
                self.set_var(name, val);
                None
            }
            Statement::AugAssign(name, op, expr) => {
                let current_val = self.get_var(name);
                if current_val == Value::Nothing { return None; }
                let operand = self.eval_expr(expr);
                let new_val = self.apply_op(&current_val, op, &operand);
                self.set_var(name, new_val);
                None
            }
            Statement::IncDec(name, op) => {
                let current_val = self.get_var(name);
                let one = Value::Number(1.0);
                let new_val = match op.as_str() {
                    "++" => self.apply_op(&current_val, "+", &one),
                    "--" => self.apply_op(&current_val, "-", &one),
                    _ => current_val
                };
                self.set_var(name, new_val);
                None
            }
            Statement::If(cond, then_block, else_block) => {
                let c_val = self.eval_expr(cond);
                if matches!(c_val, Value::Bool(true)) {
                    return self.run_block(then_block);
                } else {
                    return self.run_block(else_block);
                }
            }
            Statement::While(cond, body) => {
                while matches!(self.eval_expr(cond), Value::Bool(true)) {
                    if let Some(v) = self.run_block(body) {
                        return Some(v);
                    }
                }
                None
            }
            Statement::For(var, list_expr, body) => {
                if let Value::List(items) = self.eval_expr(list_expr) {
                    for item in items {
                        self.set_var(var, item);
                        if let Some(v) = self.run_block(body) {
                            return Some(v);
                        }
                    }
                }
                None
            }
            Statement::FunctionDef(name, params, body) => {
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert(name.clone(), Value::Function(params.clone(), body.clone()));
                }
                None
            }
            Statement::FunctionCall(name, args) => {
                let vals: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect();
                self.call_function(name, vals);
                None
            }
            Statement::Input(var_name) => {
                print!("? ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                if let Ok(num) = input.parse::<f64>() {
                    self.set_var(var_name, Value::Number(num));
                } else {
                    self.set_var(var_name, Value::Text(input.to_string()));
                }
                None
            }
            Statement::Return(expr) => {
                Some(self.eval_expr(expr))
            }
        }
    }

    fn run_block(&mut self, body: &Vec<Statement>) -> Option<Value> {
        for stmt in body {
            if let Some(val) = self.execute(stmt) {
                return Some(val);
            }
        }
        None
    }

    fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            Expr::Text(s) => Value::Text(s.clone()),
            Expr::Bool(b) => Value::Bool(*b),
            Expr::Variable(name) => self.get_var(name),
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

    fn next_random(&mut self, max: u64) -> f64 {
        if max == 0 { return 0.0; }
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng_state = x;
        (x % max) as f64
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
            (Value::Text(l), Value::Text(r)) if op == "!=" => Value::Bool(l != r),
            (Value::Text(l), Value::Number(r)) if op == "+" => Value::Text(format!("{}{}", l, r)),
            (Value::Number(l), Value::Text(r)) if op == "+" => Value::Text(format!("{}{}", l, r)),
            (Value::Bool(l), Value::Bool(r)) if op == "==" => Value::Bool(l == r),
            (Value::Bool(l), Value::Bool(r)) if op == "!=" => Value::Bool(l != r),
            (Value::List(l), Value::List(r)) if op == "+" => {
                let mut new_list = l.clone();
                new_list.extend(r.clone());
                Value::List(new_list)
            },
            _ => Value::Nothing,
        }
    }

    fn call_function(&mut self, name: &str, args: Vec<Value>) -> Value {
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
                    let strs: Vec<String> = items.iter().map(|v| match v {
                        Value::Text(t) => t.clone(),
                        _ => format!("{}", v)
                    }).collect();
                    return Value::Text(strs.join(sep));
                }
                Value::Text(String::new())
            },
            "|" => {
                if let (Some(Value::Text(s)), Some(Value::Text(sep))) = (args.get(0), args.get(1)) {
                    if sep.is_empty() { return Value::List(vec![]); }
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
                let fn_val = self.get_var(name);
                if let Value::Function(params, body) = fn_val {
                    let mut local_scope = HashMap::new();
                    for (i, param) in params.iter().enumerate() {
                        if let Some(arg) = args.get(i) {
                            local_scope.insert(param.clone(), arg.clone());
                        }
                    }

                    self.scopes.push(local_scope);
                    let result = self.run_block(&body);
                    self.scopes.pop();

                    return result.unwrap_or(Value::Nothing);
                }
                Value::Nothing
            }
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

        if line.contains("=>") && !line.starts_with('"') {
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

        if line.starts_with(">> ") {
            let content = line[2..].trim();
            if let Some(space_idx) = content.find(char::is_whitespace) {
                let var = content[..space_idx].trim().to_string();
                let list_expr = parse_expr(content[space_idx..].trim());
                *current += 1;
                let body = parse_lines(lines, current);
                statements.push(Statement::For(var, list_expr, body));
                continue;
            }
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

    if line.starts_with("?") {
        let content = line[1..].trim();
        if !is_condition(content) {
            return Some(Statement::Input(content.to_string()));
        }
    }

    if line.starts_with("->") {
        let content = if line.starts_with("-> ") {
            line[3..].trim()
        } else {
            line[2..].trim()
        };
        return Some(Statement::Return(parse_expr(content)));
    }

    if line.ends_with("++") {
        return Some(Statement::IncDec(line[..line.len()-2].trim().to_string(), "++".to_string()));
    }
    if line.ends_with("--") {
        return Some(Statement::IncDec(line[..line.len()-2].trim().to_string(), "--".to_string()));
    }

    for op in &["+=", "-=", "*=", "/="] {
        if let Some(idx) = line.find(op) {
            let var = line[..idx].trim().to_string();
            let expr = parse_expr(line[idx+2..].trim());
            return Some(Statement::AugAssign(var, op[..1].to_string(), expr));
        }
    }

    if let Some(eq_idx) = find_assign_op(line) {
        let var = line[..eq_idx].trim().to_string();
        let expr = parse_expr(line[eq_idx+1..].trim());
        return Some(Statement::Assign(var, expr));
    }

    if !line.starts_with("=>") && !line.starts_with("<=") && !line.starts_with("<-") {
        return Some(Statement::Print(parse_expr(line)));
    }

    None
}

fn find_assign_op(s: &str) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut in_quotes = false;

    while i < chars.len() {
        if chars[i] == '"' {
            in_quotes = !in_quotes;
        }

        if !in_quotes && chars[i] == '=' {
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

    let logical_ops = ["==", "!=", ">=", "<=", ">", "<"];
    for op in &logical_ops {
        if let Some(idx) = find_op_outside_parens(s, op, false) {
            let left = parse_expr(&s[..idx]);
            let right = parse_expr(&s[idx + op.len()..]);
            return Expr::BinaryOp(Box::new(left), op.to_string(), Box::new(right));
        }
    }

    let math_ops = ["+", "-", "*", "/", "%"];
    for op in &math_ops {
        if let Some(idx) = find_op_outside_parens(s, op, true) {
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

fn find_op_outside_parens(s: &str, op: &str, reverse: bool) -> Option<usize> {
    let s_chars: Vec<char> = s.chars().collect();
    let op_chars: Vec<char> = op.chars().collect();
    let len = s_chars.len();
    let op_len = op_chars.len();

    if len < op_len { return None; }

    let check_at = |i: usize| -> bool {
        if i + op_len > len { return false; }
        if &s_chars[i..i+op_len] == &op_chars[..] {
            if op == "-" && i == 0 { return false; }
            return true;
        }
        false
    };

    let mut balance = 0;
    let mut in_quotes = false;

    if reverse {
        let mut i = len;
        while i > 0 {
            i -= 1;
            let c = s_chars[i];
            if c == '"' { in_quotes = !in_quotes; }
            if !in_quotes {
                if c == ')' || c == ']' { balance += 1; }
                else if c == '(' || c == '[' { balance -= 1; }

                if balance == 0 && check_at(i) { return Some(i); }
            }
        }
    } else {
        let mut i = 0;
        while i < len {
            let c = s_chars[i];
            if c == '"' { in_quotes = !in_quotes; }
            if !in_quotes {
                if c == '(' || c == '[' { balance += 1; }
                else if c == ')' || c == ']' { balance -= 1; }

                if balance == 0 && check_at(i) { return Some(i); }
            }
            i += 1;
        }
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
    let mut in_quotes = false;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '"' {
            in_quotes = !in_quotes;
            cur.push(c);
        } else if in_quotes {
            cur.push(c);
        } else {
            if c == '(' || c == '[' { balance += 1; }
            else if c == ')' || c == ']' { balance -= 1; }

            if balance == 0 && c.is_whitespace() {
                if !cur.is_empty() { args.push(cur.clone()); cur.clear(); }
            } else {
                cur.push(c);
            }
        }
    }
    if !cur.is_empty() { args.push(cur); }

    let mut merged = Vec::new();
    let mut buffer = String::new();

    for part in args {
        let is_math_op = ["+", "-", "*", "/", "%", "==", "!=", ">", "<", ">=", "<=", "!"].contains(&part.as_str());

        let prev_ends_op = buffer.ends_with(|c: char| "+-*/%=!><".contains(c));

        if is_math_op || prev_ends_op {
            buffer.push_str(&part);
        } else {
            if !buffer.is_empty() {
                merged.push(buffer.clone());
            }
            buffer = part;
        }
    }
    if !buffer.is_empty() { merged.push(buffer); }

    merged
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match fs::read_to_string(&args[1]) {
            Ok(code) => {
                let stmts = parse(&code);
                let mut interp = Interpreter::new();
                interp.run_block(&stmts);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        println!("Lazy Lang REPL");
        let mut interp = Interpreter::new();
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
                interp.run_block(&stmts);
                buf.clear();
            } else {
                buf.push_str(input);
                buf.push('\n');
            }
        }
    }
}