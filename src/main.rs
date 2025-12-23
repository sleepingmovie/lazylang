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
                        Value::Text(t) => format!("\"{}\"", t),
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
    If(Expr, Vec<Statement>, Vec<(Expr, Vec<Statement>)>, Vec<Statement>),
    While(Expr, Vec<Statement>),
    For(String, Expr, Vec<Statement>),
    FunctionDef(String, Vec<String>, Vec<Statement>),
    QuickFunctionDef(String, Vec<String>, Expr),
    FunctionCall(String, Vec<Expr>, bool), // name, args, mutates
    Return(Expr),
    Input(Vec<String>, Option<String>, bool),
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
    FunctionCall(String, Vec<Expr>, bool), // name, args, mutates
    InputExpr,
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

    fn read_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn parse_input_value(&self, input: &str) -> Value {
        if let Ok(num) = input.parse::<f64>() {
            Value::Number(num)
        } else {
            Value::Text(input.to_string())
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
            Statement::If(cond, then_block, else_ifs, else_block) => {
                let c_val = self.eval_expr(cond);
                if matches!(c_val, Value::Bool(true)) {
                    return self.run_block(then_block);
                }
                for (elif_cond, elif_block) in else_ifs {
                    let elif_val = self.eval_expr(elif_cond);
                    if matches!(elif_val, Value::Bool(true)) {
                        return self.run_block(elif_block);
                    }
                }
                return self.run_block(else_block);
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
            Statement::QuickFunctionDef(name, params, expr) => {
                let body = vec![Statement::Return(expr.clone())];
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert(name.clone(), Value::Function(params.clone(), body));
                }
                None
            }
            Statement::FunctionCall(name, args, mutates) => {
                let vals: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect();
                let result = self.call_function(name, vals, *mutates);

                if *mutates {
                    if let Some(Expr::Variable(var_name)) = args.first() {
                        self.set_var(var_name, result.clone());
                    }
                }
                None
            }
            Statement::Input(vars, prompt, is_iter) => {
                if *is_iter {
                    let base_prompt = prompt.as_ref().map(|s| s.as_str()).unwrap_or("+? ");
                    for (i, var) in vars.iter().enumerate() {
                        let actual_prompt = base_prompt.replace("{?}", &(i + 1).to_string());
                        let input = self.read_input(&actual_prompt);
                        let val = self.parse_input_value(&input);
                        self.set_var(var, val);
                    }
                } else {
                    let actual_prompt = prompt.as_ref().map(|s| s.as_str()).unwrap_or("+? ");
                    for var in vars {
                        let input = self.read_input(actual_prompt);
                        let val = self.parse_input_value(&input);
                        self.set_var(var, val);
                    }
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
                    let i = idx as i64;
                    let actual_idx = if i < 0 {
                        (items.len() as i64 + i) as usize
                    } else {
                        i as usize
                    };
                    if actual_idx < items.len() { return items[actual_idx].clone(); }
                }
                Value::Nothing
            }
            Expr::BinaryOp(left, op, right) => {
                let l = self.eval_expr(left);
                let r = self.eval_expr(right);
                self.apply_op(&l, op, &r)
            }
            Expr::FunctionCall(name, args, mutates) => {
                let arg_vals: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect();
                let result = self.call_function(name, arg_vals, *mutates);

                if *mutates {
                    if let Some(Expr::Variable(var_name)) = args.first() {
                        self.set_var(var_name, result.clone());
                    }
                }
                result
            }
            Expr::InputExpr => {
                let input = self.read_input("+? ");
                self.parse_input_value(&input)
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

    fn call_function(&mut self, name: &str, args: Vec<Value>, mutates: bool) -> Value {
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
            "<>" => {
                if let Some(Value::List(items)) = args.get(0) {
                    let mut reversed = items.clone();
                    reversed.reverse();
                    return Value::List(reversed);
                }
                Value::Nothing
            },
            "++" => {
                if let Some(Value::List(items)) = args.get(0) {
                    let mut sorted = items.clone();
                    sorted.sort_by(|a, b| {
                        match (a, b) {
                            (Value::Number(x), Value::Number(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
                            (Value::Text(x), Value::Text(y)) => x.cmp(y),
                            _ => std::cmp::Ordering::Equal,
                        }
                    });
                    return Value::List(sorted);
                }
                Value::Nothing
            },
            "--" => {
                if let Some(Value::List(items)) = args.get(0) {
                    let mut sorted = items.clone();
                    sorted.sort_by(|a, b| {
                        match (a, b) {
                            (Value::Number(x), Value::Number(y)) => y.partial_cmp(x).unwrap_or(std::cmp::Ordering::Equal),
                            (Value::Text(x), Value::Text(y)) => y.cmp(x),
                            _ => std::cmp::Ordering::Equal,
                        }
                    });
                    return Value::List(sorted);
                }
                Value::Nothing
            },
            "><" => {
                if let (Some(Value::List(items)), Some(val)) = (args.get(0), args.get(1)) {
                    for item in items {
                        if item == val {
                            return Value::Bool(true);
                        }
                    }
                    return Value::Bool(false);
                }
                Value::Bool(false)
            },
            "<<" => {
                if let Some(Value::List(items)) = args.get(0) {
                    let mut unique = Vec::new();
                    for item in items {
                        if !unique.contains(item) {
                            unique.push(item.clone());
                        }
                    }
                    return Value::List(unique);
                }
                Value::Nothing
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
    let lines: Vec<&str> = code.lines()
        .map(|l| {
            if let Some(idx) = l.find("//") {
                &l[..idx]
            } else {
                l
            }
        })
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    let mut idx = 0;
    parse_lines(&lines, &mut idx)
}

fn parse_lines(lines: &[&str], current: &mut usize) -> Vec<Statement> {
    let mut statements = Vec::new();
    while *current < lines.len() {
        let line = lines[*current];

        if line == "}" {
            *current += 1;
            return statements;
        }

        if line.starts_with("??") {
            println!("Error: Unexpected '??' without matching '?' (Orphaned Else)");
            *current += 1;
            let _ = parse_lines(lines, current);
            continue;
        }

        if line.contains("~>") {
            let parts: Vec<&str> = line.split("~>").collect();
            let sig = parts[0].trim();
            if let Some(paren_idx) = sig.find('(') {
                let name = sig[..paren_idx].trim().to_string();
                let params_str = sig[paren_idx + 1..].trim_end_matches(')').trim();
                let params: Vec<String> = if params_str.is_empty() { vec![] }
                else { params_str.split_whitespace().map(|s| s.to_string()).collect() };

                let expr_str = parts[1].trim();
                let expr = parse_expr(expr_str);

                statements.push(Statement::QuickFunctionDef(name, params, expr));
                *current += 1;
                continue;
            }
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
            let cond_str = line[2..].trim().trim_end_matches('{').trim();
            let cond = parse_expr(cond_str);
            *current += 1;
            let body = parse_lines(lines, current);
            statements.push(Statement::While(cond, body));
            continue;
        }

        if line.starts_with(">> ") {
            let content = line[3..].trim();
            if let Some(first_space) = content.find(char::is_whitespace) {
                let var_name = content[..first_space].trim().to_string();
                let mut rest = content[first_space..].trim();

                if rest.starts_with("->") {
                    rest = rest[2..].trim();
                }

                let list_expr_str = rest.trim_end_matches('{').trim();
                let list_expr = parse_expr(list_expr_str);

                *current += 1;
                let body = parse_lines(lines, current);
                statements.push(Statement::For(var_name, list_expr, body));
                continue;
            }
        }

        if line.starts_with("? ") && !line.contains(':') {
            let cond_str = line[2..].trim().trim_end_matches('{').trim();
            let cond = parse_expr(cond_str);
            *current += 1;
            let then_block = parse_lines(lines, current);

            let mut else_ifs = Vec::new();
            let mut else_block = Vec::new();

            while *current < lines.len() {
                let next_line = lines[*current];
                let next_clean = next_line.trim().trim_end_matches('{').trim();

                if next_clean == "??" {
                    *current += 1;
                    else_block = parse_lines(lines, current);
                    break;
                } else if next_line.starts_with("?? ") {
                    let elif_cond_str = next_line[3..].trim().trim_end_matches('{').trim();
                    let elif_cond = parse_expr(elif_cond_str);
                    *current += 1;
                    let elif_block = parse_lines(lines, current);
                    else_ifs.push((elif_cond, elif_block));
                } else {
                    break;
                }
            }

            statements.push(Statement::If(cond, then_block, else_ifs, else_block));
            continue;
        }

        if let Some(stmt) = parse_simple_statement(line) {
            statements.push(stmt);
        }
        *current += 1;
    }
    statements
}

fn parse_simple_statement(line: &str) -> Option<Statement> {
    let line = line.trim();

    if line.starts_with("+? ") {
        let content = line[3..].trim();
        if let Some(colon_idx) = content.find(':') {
            let vars_part = content[..colon_idx].trim();
            let prompt_part = content[colon_idx + 1..].trim();
            let vars: Vec<String> = vars_part.split_whitespace().map(|s| s.to_string()).collect();
            let prompt = if prompt_part.starts_with('"') && prompt_part.ends_with('"') {
                prompt_part[1..prompt_part.len()-1].to_string()
            } else {
                prompt_part.to_string()
            };
            let is_iter = prompt.contains("{?}");
            return Some(Statement::Input(vars, Some(prompt), is_iter));
        } else {
            let vars: Vec<String> = content.split_whitespace().map(|s| s.to_string()).collect();
            return Some(Statement::Input(vars, None, false));
        }
    }

    if line.starts_with("->") {
        let content = if line.starts_with("-> ") { line[3..].trim() } else { line[2..].trim() };
        return Some(Statement::Return(parse_expr(content)));
    }

    if line.ends_with("++") && !line.contains('(') {
        return Some(Statement::IncDec(line[..line.len()-2].trim().to_string(), "++".to_string()));
    }
    if line.ends_with("--") && !line.contains('(') {
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

    if !line.starts_with("=>") && !line.starts_with("}") {
        let expr = parse_expr(line);
        match expr {
            Expr::FunctionCall(name, args, true) => return Some(Statement::FunctionCall(name, args, true)),
            _ => return Some(Statement::Print(expr)),
        }
    }

    None
}

// --- HELPER FUNCTIONS ---

fn find_assign_op(s: &str) -> Option<usize> {
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    let mut i = 0;
    let mut in_quotes = false;

    while i < chars.len() {
        let (byte_idx, c) = chars[i];
        if c == '"' { in_quotes = !in_quotes; }

        if !in_quotes && c == '=' {
            let prev = if i > 0 { chars[i-1].1 } else { ' ' };
            let next = if i+1 < chars.len() { chars[i+1].1 } else { ' ' };
            if prev != '>' && prev != '<' && prev != '!' && prev != '='
                && prev != '+' && prev != '-' && prev != '*' && prev != '/' && prev != '~'
                && next != '=' {
                return Some(byte_idx);
            }
        }
        i += 1;
    }
    None
}

fn parse_expr(s: &str) -> Expr {
    let s = s.trim();

    if s == "+??" { return Expr::InputExpr; }

    if s.starts_with('!') && !s.starts_with("!=") {
        let operand = parse_expr(&s[1..]);
        return Expr::FunctionCall("!".to_string(), vec![operand], false);
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

    if s.ends_with(")*") {
        if let Some(idx) = find_matching_open(&s[..s.len()-1], '(', ')') {
            let name = s[..idx].trim().to_string();
            let args_str = s[idx+1..s.len()-2].trim();
            let args = parse_function_args(args_str);
            return Expr::FunctionCall(name, args, true);
        }
    } else if s.ends_with(')') {
        if let Some(idx) = find_matching_open(s, '(', ')') {
            let name = s[..idx].trim().to_string();

            if name.is_empty() {
                return parse_expr(&s[idx+1..s.len()-1]);
            }

            let args_str = s[idx+1..s.len()-1].trim();
            let args = parse_function_args(args_str);
            return Expr::FunctionCall(name, args, false);
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

fn parse_function_args(args_str: &str) -> Vec<Expr> {
    if args_str.is_empty() { return vec![]; }
    let arrow_parts = split_by_arrow(args_str);
    if arrow_parts.len() > 1 {
        return arrow_parts.iter().map(|p| parse_expr(p.trim())).collect();
    }
    let space_parts = split_args_outside_parens(arrow_parts[0]);
    space_parts.iter().map(|p| parse_expr(p.trim())).collect()
}

fn split_by_arrow(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    let mut current_byte_start = 0;
    let mut i = 0;
    let mut in_quotes = false;
    let mut paren_depth = 0;

    while i < chars.len() {
        let (byte_idx, c) = chars[i];
        if c == '"' { in_quotes = !in_quotes; }

        if !in_quotes {
            if c == '(' { paren_depth += 1; }
            else if c == ')' { paren_depth -= 1; }

            if paren_depth == 0 && i + 1 < chars.len() && c == '-' && chars[i+1].1 == '>' {
                result.push(&s[current_byte_start..byte_idx]);
                i += 2;
                if i < chars.len() { current_byte_start = chars[i].0; }
                else { current_byte_start = s.len(); }
                continue;
            }
        }
        i += 1;
    }

    if current_byte_start < s.len() { result.push(&s[current_byte_start..]); }
    else if result.is_empty() { result.push(s); }
    result
}

fn find_op_outside_parens(s: &str, op: &str, reverse: bool) -> Option<usize> {
    let s_chars: Vec<(usize, char)> = s.char_indices().collect();
    let op_chars: Vec<char> = op.chars().collect();
    let len = s_chars.len();
    let op_len = op_chars.len();

    if len < op_len { return None; }

    let check_at = |i: usize| -> bool {
        if i + op_len > len { return false; }

        if i == 0 || i + op_len == len { return false; }

        if op == "<" {
            if i + 1 < len && s_chars[i+1].1 == '>' { return false; }
            if i > 0 && s_chars[i-1].1 == '>' { return false; }
            if i + 1 < len && s_chars[i+1].1 == '<' { return false; }
            if i > 0 && s_chars[i-1].1 == '<' { return false; }
        }
        if op == ">" {
            if i > 0 && s_chars[i-1].1 == '<' { return false; }
            if i + 1 < len && s_chars[i+1].1 == '<' { return false; }
            if i + 1 < len && s_chars[i+1].1 == '>' { return false; }
            if i > 0 && s_chars[i-1].1 == '>' { return false; }
        }
        if op == "+" {
            if i + 1 < len && s_chars[i+1].1 == '+' { return false; }
            if i > 0 && s_chars[i-1].1 == '+' { return false; }
        }
        if op == "-" {
            if i + 1 < len && s_chars[i+1].1 == '-' { return false; }
            if i > 0 && s_chars[i-1].1 == '-' { return false; }
            if i + 1 < len && s_chars[i+1].1 == '>' { return false; }
        }

        for k in 0..op_len {
            if s_chars[i+k].1 != op_chars[k] { return false; }
        }
        return true;
    };

    let mut balance = 0;
    let mut in_quotes = false;

    if reverse {
        let mut i = len;
        while i > 0 {
            i -= 1;
            let (_, c) = s_chars[i];
            if c == '"' { in_quotes = !in_quotes; }
            if !in_quotes {
                if c == ')' || c == ']' { balance += 1; }
                else if c == '(' || c == '[' { balance -= 1; }
                if balance == 0 && check_at(i) { return Some(s_chars[i].0); }
            }
        }
    } else {
        let mut i = 0;
        while i < len {
            let (_, c) = s_chars[i];
            if c == '"' { in_quotes = !in_quotes; }
            if !in_quotes {
                if c == '(' || c == '[' { balance += 1; }
                else if c == ')' || c == ']' { balance -= 1; }
                if balance == 0 && check_at(i) { return Some(s_chars[i].0); }
            }
            i += 1;
        }
    }
    None
}

fn find_matching_open(s: &str, open: char, close: char) -> Option<usize> {
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    if chars.len() < 2 { return None; }
    let mut balance = 1;
    let mut i = chars.len() - 2;

    loop {
        let (byte_idx, c) = chars[i];
        if c == close { balance += 1; }
        if c == open {
            balance -= 1;
            if balance == 0 { return Some(byte_idx); }
        }
        if i == 0 { break; }
        i -= 1;
    }
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
        println!("Lazy Lang REPL - Type 'exit' to quit, 'run' to execute buffer");
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