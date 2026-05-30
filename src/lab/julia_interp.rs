use std::collections::HashMap;

use lab_types::types::{VerifyArg, VerifyCase, VerifyExpectation, VerifyResult};

use crate::lab::crypto;

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    Unit,
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Str(s) => !s.is_empty(),
            Value::Array(items) => !items.is_empty(),
            Value::Tuple(items) => !items.is_empty(),
            Value::Unit => false,
        }
    }

    fn display(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::Array(items) => {
                let inner: Vec<String> = items.iter().map(Value::display).collect();
                format!("[{}]", inner.join(", "))
            }
            Value::Tuple(items) => {
                let inner: Vec<String> = items.iter().map(Value::display).collect();
                format!("({})", inner.join(", "))
            }
            Value::Unit => "nothing".to_string(),
        }
    }

    fn as_int(&self) -> Result<i64, String> {
        match self {
            Value::Int(n) => Ok(*n),
            Value::Unit => Err("Uninitialized element: expected integer, got nothing.".to_string()),
            other => Err(format!("Expected integer, got {}.", other.display())),
        }
    }

    fn default_of_same_type(&self) -> Value {
        match self {
            Value::Int(_) => Value::Int(0),
            Value::Bool(_) => Value::Bool(false),
            Value::Str(_) => Value::Str(String::new()),
            Value::Array(_) => Value::Array(Vec::new()),
            Value::Tuple(_) => Value::Tuple(Vec::new()),
            Value::Unit => Value::Unit,
        }
    }

    fn infer_array_padding(items: &[Value], assigned: &Value) -> Value {
        let typed: Vec<&Value> = items
            .iter()
            .filter(|value| !matches!(value, Value::Unit))
            .collect();

        if !typed.is_empty() {
            let first = typed[0];
            if typed
                .iter()
                .all(|value| std::mem::discriminant(*value) == std::mem::discriminant(first))
            {
                return first.default_of_same_type();
            }
            return Value::Unit;
        }

        assigned.default_of_same_type()
    }

    fn uninitialized_binary_error(op: &str) -> String {
        format!("Uninitialized element: cannot apply `{op}` to nothing.")
    }
}

#[derive(Clone)]
struct FunctionDef {
    name: String,
    params: Vec<String>,
    body: Vec<Stmt>,
}

#[derive(Clone)]
enum Stmt {
    Assign { names: Vec<String>, expr: Expr },
    IndexAssign { target: String, index: Expr, value: Expr },
    Expr(Expr),
    If {
        cond: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    While { cond: Expr, body: Vec<Stmt> },
    For { var: String, iter: Expr, body: Vec<Stmt> },
    Return(Option<Expr>),
}

#[derive(Clone)]
enum Expr {
    Int(i64),
    Bool(bool),
    Str(String),
    Var(String),
    Array(Vec<Expr>),
    Tuple(Vec<Expr>),
    Unary { op: &'static str, expr: Box<Expr> },
    Binary {
        op: &'static str,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call { name: String, args: Vec<Expr> },
    Index { target: Box<Expr>, index: Box<Expr> },
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        step: Option<Box<Expr>>,
    },
    Comprehension {
        item: Box<Expr>,
        var: String,
        iter: Box<Expr>,
        filter: Option<Box<Expr>>,
    },
}

enum AssignLhs {
    Names(Vec<String>),
    Index { target: String, index: Expr },
}

const INSTRUCTION_QUOTA_EXCEEDED: &str =
    "Instruction quota exceeded: Execution halted to prevent thread lock.";
pub const DEFAULT_INSTRUCTION_BUDGET: usize = 100_000;
const MAX_INSTRUCTION_BUDGET: usize = 1_000_000;

struct Interpreter {
    globals: HashMap<String, Value>,
    functions: HashMap<String, FunctionDef>,
    stdout: Vec<String>,
    instruction_budget: usize,
}

enum Flow {
    None,
    Return(Value),
}

pub fn execute(source: &str) -> Result<(Vec<String>, Option<String>), String> {
    execute_with_budget(source, DEFAULT_INSTRUCTION_BUDGET)
}

pub fn execute_with_budget(
    source: &str,
    instruction_budget: usize,
) -> Result<(Vec<String>, Option<String>), String> {
    let instruction_budget = instruction_budget.clamp(1, MAX_INSTRUCTION_BUDGET);
    let mut interpreter = Interpreter {
        globals: HashMap::new(),
        functions: HashMap::new(),
        stdout: Vec::new(),
        instruction_budget,
    };

    let mut parser = Parser::new(source);
    let program = parser.parse_program()?;

    let mut last_value: Option<Value> = None;
    for item in program {
        match item {
            ProgramItem::Function(def) => {
                interpreter.functions.insert(def.name.clone(), def);
            }
            ProgramItem::Statement(stmt) => {
                let flow = interpreter.run_stmt(&stmt)?;
                if let Flow::Return(value) = flow {
                    last_value = Some(value);
                    break;
                }
                if let Stmt::Expr(expr) = &stmt {
                    last_value = Some(interpreter.eval_expr(expr)?);
                }
            }
        }
    }

    let result = last_value.map(|v| v.display());
    Ok((interpreter.stdout, result))
}

pub fn grade_user_code(
    source: &str,
    cases: &[VerifyCase],
    probes: &HashMap<String, i64>,
) -> Result<Vec<VerifyResult>, String> {
    let mut interpreter = Interpreter {
        globals: HashMap::new(),
        functions: HashMap::new(),
        stdout: Vec::new(),
        instruction_budget: DEFAULT_INSTRUCTION_BUDGET,
    };

    let mut parser = Parser::new(source);
    let program = parser.parse_program()?;

    for item in program {
        if let ProgramItem::Function(def) = item {
            interpreter.functions.insert(def.name.clone(), def);
        }
    }

    for (name, value) in probes {
        interpreter
            .globals
            .insert(name.clone(), Value::Int(*value));
    }

    let mut results = Vec::with_capacity(cases.len());
    for case in cases {
        results.push(evaluate_user_case(&mut interpreter, case));
    }
    Ok(results)
}

fn evaluate_user_case(interpreter: &mut Interpreter, case: &VerifyCase) -> VerifyResult {
    let args: Vec<Expr> = match resolve_verify_args(&case.args, interpreter) {
        Ok(args) => args,
        Err(message) => {
            return VerifyResult {
                expression: case.expression.clone(),
                passed: false,
                expected: format_expectation(&case.expected),
                got: message,
            };
        }
    };

    let outcome = interpreter.eval_call(&case.function, &args);

    let (passed, got) = match (&case.expected, outcome) {
        (VerifyExpectation::Value(expected), Ok(value)) => {
            match value.as_int() {
                Ok(got) => (*expected == got, got.to_string()),
                Err(message) => (false, message),
            }
        }
        (VerifyExpectation::Value(expected), Err(_)) => (false, format!("error (expected {expected})")),
        (VerifyExpectation::Error, Ok(value)) => (false, value.display()),
        (VerifyExpectation::Error, Err(_)) => (true, "error".to_string()),
    };

    VerifyResult {
        expression: case.expression.clone(),
        passed,
        expected: format_expectation(&case.expected),
        got,
    }
}

fn resolve_verify_args(
    args: &[VerifyArg],
    interpreter: &Interpreter,
) -> Result<Vec<Expr>, String> {
    args.iter()
        .map(|arg| match arg {
            VerifyArg::Literal(value) => Ok(Expr::Int(*value)),
            VerifyArg::Probe(name) => {
                if interpreter.globals.contains_key(name) {
                    Ok(Expr::Var(name.clone()))
                } else {
                    Err(format!("Unknown probe: {name}"))
                }
            }
        })
        .collect()
}

fn format_expectation(expected: &VerifyExpectation) -> String {
    match expected {
        VerifyExpectation::Value(value) => value.to_string(),
        VerifyExpectation::Error => "error".to_string(),
    }
}

enum ProgramItem {
    Function(FunctionDef),
    Statement(Stmt),
}

impl Interpreter {
    fn consume_instruction(&mut self) -> Result<(), String> {
        if self.instruction_budget == 0 {
            return Err(INSTRUCTION_QUOTA_EXCEEDED.to_string());
        }
        self.instruction_budget -= 1;
        Ok(())
    }

    fn run_stmt(&mut self, stmt: &Stmt) -> Result<Flow, String> {
        self.consume_instruction()?;
        match stmt {
            Stmt::Assign { names, expr } => {
                let value = self.eval_expr(expr)?;
                self.assign_names(names, value)?;
                Ok(Flow::None)
            }
            Stmt::IndexAssign { target, index, value } => {
                let idx = self.eval_expr(index)?.as_int()? as usize;
                if idx == 0 {
                    return Err("Julia arrays are 1-indexed.".to_string());
                }
                let val = self.eval_expr(value)?;
                let arr = self
                    .globals
                    .get_mut(target)
                    .ok_or_else(|| format!("Undefined array: {target}"))?;
                let Value::Array(items) = arr else {
                    return Err(format!("{target} is not an array."));
                };
                if items.len() < idx {
                    let padding = Value::infer_array_padding(items, &val);
                    items.resize(idx, padding);
                }
                items[idx - 1] = val;
                Ok(Flow::None)
            }
            Stmt::Expr(expr) => {
                let _ = self.eval_expr(expr)?;
                Ok(Flow::None)
            }
            Stmt::If {
                cond,
                then_body,
                else_body,
            } => {
                if self.eval_expr(cond)?.is_truthy() {
                    self.run_block(then_body)
                } else if let Some(body) = else_body {
                    self.run_block(body)
                } else {
                    Ok(Flow::None)
                }
            }
            Stmt::While { cond, body } => {
                while self.eval_expr(cond)?.is_truthy() {
                    self.consume_instruction()?;
                    match self.run_block(body)? {
                        Flow::None => {}
                        flow => return Ok(flow),
                    }
                }
                Ok(Flow::None)
            }
            Stmt::For { var, iter, body } => {
                for item in self.eval_iterable(iter)? {
                    self.consume_instruction()?;
                    self.globals.insert(var.clone(), item);
                    match self.run_block(body)? {
                        Flow::None => {}
                        flow => return Ok(flow),
                    }
                }
                Ok(Flow::None)
            }
            Stmt::Return(expr) => {
                let value = match expr {
                    Some(expr) => self.eval_expr(expr)?,
                    None => Value::Unit,
                };
                Ok(Flow::Return(value))
            }
        }
    }

    fn run_block(&mut self, stmts: &[Stmt]) -> Result<Flow, String> {
        for stmt in stmts {
            match self.run_stmt(stmt)? {
                Flow::None => {}
                flow => return Ok(flow),
            }
        }
        Ok(Flow::None)
    }

    fn assign_names(&mut self, names: &[String], value: Value) -> Result<(), String> {
        if names.len() == 1 {
            if names[0] != "_" {
                self.globals.insert(names[0].clone(), value);
            }
            return Ok(());
        }

        let Value::Tuple(items) = value else {
            return Err("Expected multiple return values.".to_string());
        };
        if items.len() != names.len() {
            return Err("Mismatch between returned values and assignment targets.".to_string());
        }
        for (name, item) in names.iter().zip(items.into_iter()) {
            if name != "_" {
                self.globals.insert(name.clone(), item);
            }
        }
        Ok(())
    }

    fn eval_iterable(&mut self, expr: &Expr) -> Result<Vec<Value>, String> {
        match self.eval_expr(expr)? {
            Value::Array(items) => Ok(items),
            other => Err(format!("Expected iterable, got {}.", other.display())),
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        self.consume_instruction()?;
        match expr {
            Expr::Int(value) => Ok(Value::Int(*value)),
            Expr::Bool(value) => Ok(Value::Bool(*value)),
            Expr::Str(value) => Ok(Value::Str(self.format_string(value)?)),
            Expr::Var(name) => self
                .globals
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {name}")),
            Expr::Array(items) => {
                let values = items
                    .iter()
                    .map(|item| self.eval_expr(item))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Value::Array(values))
            }
            Expr::Tuple(items) => {
                let values = items
                    .iter()
                    .map(|item| self.eval_expr(item))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Value::Tuple(values))
            }
            Expr::Unary { op, expr } => match (*op, self.eval_expr(expr)?) {
                ("-", Value::Int(n)) => Ok(Value::Int(-n)),
                ("-", Value::Unit) => Err(Value::uninitialized_binary_error("-")),
                ("!", value) => Ok(Value::Bool(!value.is_truthy())),
                (_, other) => Err(format!("Unsupported unary operation on {}.", other.display())),
            },
            Expr::Binary { op, left, right } => self.eval_binary(op, left, right),
            Expr::Call { name, args } => self.eval_call(name, args),
            Expr::Index { target, index } => {
                let target = self.eval_expr(target)?;
                let index = self.eval_expr(index)?.as_int()? as usize;
                if index == 0 {
                    return Err("Julia arrays are 1-indexed.".to_string());
                }
                match target {
                    Value::Array(items) => items
                        .get(index - 1)
                        .cloned()
                        .ok_or_else(|| format!("Index {index} out of bounds.")),
                    other => Err(format!("Cannot index {}.", other.display())),
                }
            }
            Expr::Range { start, end, step } => {
                let start = self.eval_expr(start)?.as_int()?;
                let end = self.eval_expr(end)?.as_int()?;
                let step = match step {
                    Some(step) => self.eval_expr(step)?.as_int()?,
                    None => 1,
                };
                if step == 0 {
                    return Err("Range step cannot be zero.".to_string());
                }
                let mut values = Vec::new();
                let mut current = start;
                loop {
                    self.consume_instruction()?;
                    if step > 0 {
                        if current > end {
                            break;
                        }
                    } else if current < end {
                        break;
                    }
                    values.push(Value::Int(current));
                    current += step;
                }
                Ok(Value::Array(values))
            }
            Expr::Comprehension {
                item,
                var,
                iter,
                filter,
            } => {
                let mut results = Vec::new();
                for candidate in self.eval_iterable(iter)? {
                    self.consume_instruction()?;
                    self.globals.insert(var.clone(), candidate);
                    if let Some(filter) = filter {
                        if !self.eval_expr(filter)?.is_truthy() {
                            continue;
                        }
                    }
                    results.push(self.eval_expr(item)?);
                }
                Ok(Value::Array(results))
            }
        }
    }

    fn eval_binary(&mut self, op: &str, left: &Expr, right: &Expr) -> Result<Value, String> {
        if op == "&&" {
            let left_val = self.eval_expr(left)?;
            if !left_val.is_truthy() {
                return Ok(Value::Bool(false));
            }
            return Ok(Value::Bool(self.eval_expr(right)?.is_truthy()));
        }
        if op == "in" {
            let needle = self.eval_expr(left)?;
            let haystack = self.eval_expr(right)?;
            return Ok(Value::Bool(match haystack {
                Value::Array(items) => items.iter().any(|item| item == &needle),
                other => {
                    return Err(format!(
                        "Right-hand side of `in` must be an array, got {}.",
                        other.display()
                    ));
                }
            }));
        }

        let left = self.eval_expr(left)?;
        let right = self.eval_expr(right)?;
        if matches!(left, Value::Unit) || matches!(right, Value::Unit) {
            return match op {
                "==" | "!=" => Ok(Value::Bool(match op {
                    "==" => left == right,
                    _ => left != right,
                })),
                _ => Err(Value::uninitialized_binary_error(op)),
            };
        }
        match (op, left, right) {
            ("+", Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            ("+", Value::Str(a), Value::Str(b)) => Ok(Value::Str(format!("{a}{b}"))),
            ("+", Value::Str(a), Value::Int(b)) => Ok(Value::Str(format!("{a}{b}"))),
            ("+", Value::Int(a), Value::Str(b)) => Ok(Value::Str(format!("{a}{b}"))),
            ("-", Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            ("*", Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            ("/" | "÷", Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err("Division by zero.".to_string())
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            ("%", Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err("Modulo by zero.".to_string())
                } else {
                    Ok(Value::Int(a % b))
                }
            }
            ("==", left, right) => Ok(Value::Bool(left == right)),
            ("!=", left, right) => Ok(Value::Bool(left != right)),
            (">=", Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (">", Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            ("<=", Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            ("<", Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (op, left, right) => Err(format!(
                "Unsupported operation {op} between {} and {}.",
                left.display(),
                right.display()
            )),
        }
    }

    fn eval_call(&mut self, name: &str, args: &[Expr]) -> Result<Value, String> {
        if name == "println" {
            let values: Result<Vec<Value>, String> =
                args.iter().map(|arg| self.eval_expr(arg)).collect();
            let line = values?
                .iter()
                .map(Value::display)
                .collect::<Vec<_>>()
                .join("");
            self.stdout.push(line);
            return Ok(Value::Unit);
        }

        if name == "error" {
            let message = if args.is_empty() {
                "error".to_string()
            } else {
                self.eval_expr(&args[0])?.display()
            };
            return Err(message);
        }

        if let Some(value) = self.eval_builtin(name, args)? {
            return Ok(value);
        }

        let function = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined function: {name}"))?;
        if function.params.len() != args.len() {
            return Err(format!(
                "Function {name} expects {} arguments, got {}.",
                function.params.len(),
                args.len()
            ));
        }

        let mut locals = self.globals.clone();
        for (param, arg) in function.params.iter().zip(args.iter()) {
            locals.insert(param.clone(), self.eval_expr(arg)?);
        }

        let mut frame = Interpreter {
            globals: locals,
            functions: self.functions.clone(),
            stdout: Vec::new(),
            instruction_budget: self.instruction_budget,
        };

        let flow = frame.run_block(&function.body)?;
        self.instruction_budget = frame.instruction_budget;
        self.stdout.extend(frame.stdout);
        match flow {
            Flow::Return(value) => Ok(value),
            Flow::None => Ok(Value::Unit),
        }
    }

    fn eval_builtin(&mut self, name: &str, args: &[Expr]) -> Result<Option<Value>, String> {
        let mut eval = |count: usize| -> Result<Vec<i64>, String> {
            if args.len() != count {
                return Err(format!("{name} expects {count} arguments."));
            }
            args.iter()
                .map(|arg| self.eval_expr(arg).and_then(|value| value.as_int()))
                .collect()
        };

        let value = match name {
            "gcd" => {
                let vals = eval(2)?;
                Value::Int(crypto::gcd(vals[0], vals[1]))
            }
            "invmod" => {
                let vals = eval(2)?;
                Value::Int(crypto::mod_inverse(vals[0], vals[1])?)
            }
            "powermod" => {
                let vals = eval(3)?;
                Value::Int(crypto::powermod(vals[0], vals[1], vals[2])?)
            }
            "isodd" => Value::Bool(eval(1)?[0].rem_euclid(2) != 0),
            "div" => {
                let vals = eval(2)?;
                if vals[1] == 0 {
                    return Err("Division by zero.".to_string());
                }
                Value::Int(vals[0] / vals[1])
            }
            "fill" => {
                if args.len() != 2 {
                    return Err("fill expects 2 arguments.".to_string());
                }
                let value = self.eval_expr(&args[0])?;
                let len = self.eval_expr(&args[1])?.as_int()?;
                if len < 0 {
                    return Err("fill length must be non-negative.".to_string());
                }
                Value::Array(vec![value; len as usize])
            }
            "isprime" => Value::Bool(crypto::is_prime(eval(1)?[0])? == 1),
            "primes" => Value::Array(
                crypto::sieve_primes(eval(1)?[0])?
                    .into_iter()
                    .map(Value::Int)
                    .collect(),
            ),
            "prime" => Value::Int(crypto::nth_prime(eval(1)?[0])?),
            "factor" => Value::Str(crypto::factor_display(eval(1)?[0])?),
            _ => return Ok(None),
        };

        Ok(Some(value))
    }

    fn format_string(&mut self, template: &str) -> Result<String, String> {
        let mut output = String::new();
        let mut chars = template.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '$' {
                let mut ident = String::new();
                while chars
                    .peek()
                    .map(|next| next.is_ascii_alphanumeric() || *next == '_')
                    .unwrap_or(false)
                {
                    ident.push(chars.next().unwrap());
                }
                if ident.is_empty() {
                    output.push('$');
                } else {
                    let value = self
                        .globals
                        .get(&ident)
                        .cloned()
                        .ok_or_else(|| format!("Undefined variable: {ident}"))?;
                    output.push_str(&value.display());
                }
            } else {
                output.push(ch);
            }
        }
        Ok(output)
    }
}

struct Parser {
    source: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
        }
    }

    fn parse_program(&mut self) -> Result<Vec<ProgramItem>, String> {
        let mut items = Vec::new();
        while !self.is_eof() {
            self.skip_trivia();
            if self.is_eof() {
                break;
            }
            if self.starts_with_keyword("function") {
                items.push(ProgramItem::Function(self.parse_function()?));
            } else {
                items.push(ProgramItem::Statement(self.parse_statement()?));
            }
        }
        Ok(items)
    }

    fn parse_function(&mut self) -> Result<FunctionDef, String> {
        self.expect_keyword("function")?;
        let name = self.parse_identifier()?;
        self.expect_char('(')?;
        let params = self.parse_param_list()?;
        self.expect_char(')')?;
        self.skip_trivia();
        let body = self.parse_block_until(&["end"])?;
        self.expect_keyword("end")?;
        Ok(FunctionDef { name, params, body })
    }

    fn parse_block_until(&mut self, terminators: &[&str]) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        loop {
            self.skip_trivia();
            if self.is_eof() {
                return Err(format!(
                    "Unclosed block, expected one of: {}",
                    terminators.join(", ")
                ));
            }
            if terminators
                .iter()
                .any(|term| self.starts_with_keyword(term))
            {
                break;
            }
            if self.starts_with_keyword("elseif") || self.starts_with_keyword("else") {
                break;
            }
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        self.skip_trivia();
        if self.starts_with_keyword("if") {
            return self.parse_if();
        }
        if self.starts_with_keyword("while") {
            return self.parse_while();
        }
        if self.starts_with_keyword("for") {
            return self.parse_for();
        }
        if self.starts_with_keyword("return") {
            self.advance_keyword("return");
            self.skip_trivia();
            if self.starts_with_keyword("end")
                || self.starts_with_keyword("else")
                || self.starts_with_keyword("elseif")
                || self.is_statement_boundary()
            {
                return Ok(Stmt::Return(None));
            }
            return Ok(Stmt::Return(Some(self.parse_expression()?)));
        }

        let checkpoint = self.pos;
        if let Some(lhs) = self.try_parse_assign_lhs()? {
            self.skip_trivia();
            self.expect_char('=')?;
            match lhs {
                AssignLhs::Names(names) => {
                    let first = self.parse_expression()?;
                    let expr = if names.len() > 1 {
                        self.skip_trivia();
                        if self.peek_char() == Some(',') {
                            let mut items = vec![first];
                            while self.peek_char() == Some(',') {
                                self.advance();
                                self.skip_trivia();
                                items.push(self.parse_expression()?);
                                self.skip_trivia();
                            }
                            Expr::Tuple(items)
                        } else {
                            first
                        }
                    } else {
                        first
                    };
                    return Ok(Stmt::Assign { names, expr });
                }
                AssignLhs::Index { target, index } => {
                    let value = self.parse_expression()?;
                    return Ok(Stmt::IndexAssign { target, index, value });
                }
            }
        }
        self.pos = checkpoint;
        Ok(Stmt::Expr(self.parse_expression()?))
    }

    fn try_parse_assign_lhs(&mut self) -> Result<Option<AssignLhs>, String> {
        self.skip_trivia();
        let checkpoint = self.pos;

        if self.peek_char() == Some('(') {
            self.advance();
            let mut names = Vec::new();
            loop {
                self.skip_trivia();
                names.push(self.parse_identifier()?);
                self.skip_trivia();
                if self.peek_char() == Some(',') {
                    self.advance();
                    continue;
                }
                break;
            }
            self.expect_char(')')?;
            self.skip_trivia();
            if self.peek_char() != Some('=') {
                self.pos = checkpoint;
                return Ok(None);
            }
            return Ok(Some(AssignLhs::Names(names)));
        }

        let first = self.parse_identifier()?;
        self.skip_trivia();

        if self.peek_char() == Some('[') {
            self.advance();
            let index = self.parse_expression()?;
            self.expect_char(']')?;
            self.skip_trivia();
            if self.peek_char() != Some('=') {
                self.pos = checkpoint;
                return Ok(None);
            }
            return Ok(Some(AssignLhs::Index {
                target: first,
                index,
            }));
        }

        let mut names = vec![first];
        while self.peek_char() == Some(',') {
            self.advance();
            self.skip_trivia();
            names.push(self.parse_identifier()?);
            self.skip_trivia();
        }

        if self.peek_char() != Some('=') {
            self.pos = checkpoint;
            return Ok(None);
        }
        Ok(Some(AssignLhs::Names(names)))
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("if")?;
        let cond = self.parse_expression()?;
        self.skip_trivia();
        let then_body = self.parse_block_until(&["elseif", "else", "end"])?;
        let else_body = if self.starts_with_keyword("elseif") {
            self.advance_keyword("elseif");
            Some(vec![self.parse_if()?])
        } else if self.starts_with_keyword("else") {
            self.advance_keyword("else");
            Some(self.parse_block_until(&["end"])?)
        } else {
            None
        };
        self.expect_keyword("end")?;
        Ok(Stmt::If {
            cond,
            then_body,
            else_body,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("while")?;
        let cond = self.parse_expression()?;
        self.skip_trivia();
        let body = self.parse_block_until(&["end"])?;
        self.expect_keyword("end")?;
        Ok(Stmt::While { cond, body })
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.expect_keyword("for")?;
        let var = self.parse_identifier()?;
        self.skip_trivia();
        self.expect_keyword("in")?;
        let iter = self.parse_expression()?;
        self.skip_trivia();
        let body = self.parse_block_until(&["end"])?;
        self.expect_keyword("end")?;
        Ok(Stmt::For { var, iter, body })
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_range()
    }

    fn parse_range(&mut self) -> Result<Expr, String> {
        let start = self.parse_in()?;
        self.skip_trivia();
        if self.peek_char() == Some(':') {
            self.advance();
            self.skip_trivia();
            if self.peek_char() == Some(':') {
                self.advance();
                self.skip_trivia();
                let step = self.parse_in()?;
                self.skip_trivia();
                self.expect_char(':')?;
                self.skip_trivia();
                let end = self.parse_in()?;
                return Ok(Expr::Range {
                    start: Box::new(start),
                    step: Some(Box::new(step)),
                    end: Box::new(end),
                });
            }
            let end = self.parse_in()?;
            return Ok(Expr::Range {
                start: Box::new(start),
                end: Box::new(end),
                step: None,
            });
        }
        Ok(start)
    }

    fn parse_in(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_logic_or()?;
        self.skip_trivia();
        while self.starts_with_keyword("in") {
            self.advance_keyword("in");
            let right = self.parse_logic_or()?;
            expr = Expr::Binary {
                op: "in",
                left: Box::new(expr),
                right: Box::new(right),
            };
            self.skip_trivia();
        }
        Ok(expr)
    }

    fn parse_logic_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_logic_and()?;
        self.skip_trivia();
        while self.match_str("||") {
            let right = self.parse_logic_and()?;
            expr = Expr::Binary {
                op: "||",
                left: Box::new(expr),
                right: Box::new(right),
            };
            self.skip_trivia();
        }
        Ok(expr)
    }

    fn parse_logic_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison()?;
        self.skip_trivia();
        while self.match_str("&&") {
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                op: "&&",
                left: Box::new(expr),
                right: Box::new(right),
            };
            self.skip_trivia();
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_additive()?;
        self.skip_trivia();
        loop {
            let Some(op) = self.match_one(&["==", "!=", ">=", "<=", ">", "<"]) else {
                break;
            };
            let right = self.parse_additive()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
            self.skip_trivia();
        }
        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplicative()?;
        self.skip_trivia();
        loop {
            let Some(op) = self.match_one(&["+", "-"]) else {
                break;
            };
            let right = self.parse_multiplicative()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
            self.skip_trivia();
        }
        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;
        self.skip_trivia();
        loop {
            let op = if self.match_str("*") {
                "*"
            } else if self.match_str("%") {
                "%"
            } else if self.match_one(&["/", "÷"]).is_some() {
                "/"
            } else {
                break;
            };
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
            self.skip_trivia();
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        self.skip_trivia();
        if let Some(op) = self.match_one(&["-", "!"]) {
            return Ok(Expr::Unary {
                op,
                expr: Box::new(self.parse_unary()?),
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            self.skip_trivia();
            if self.peek_char() == Some('[') {
                self.advance();
                let index = self.parse_expression()?;
                self.expect_char(']')?;
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                };
                continue;
            }
            if self.peek_char() == Some('(') {
                let Expr::Var(name) = expr else {
                    return Err("Only functions can be called.".to_string());
                };
                self.advance();
                let args = self.parse_call_args()?;
                expr = Expr::Call { name, args };
                continue;
            }
            break;
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        self.skip_trivia();
        if self.peek_char() == Some('[') {
            return self.parse_array_or_comprehension();
        }
        if self.peek_char() == Some('"') {
            return Ok(Expr::Str(self.parse_string()?));
        }
        if self.peek_char() == Some('(') {
            self.advance();
            self.skip_trivia();
            if self.peek_char() == Some(')') {
                self.advance();
                return Ok(Expr::Tuple(Vec::new()));
            }
            let first = self.parse_expression()?;
            self.skip_trivia();
            if self.peek_char() == Some(',') {
                let mut items = vec![first];
                while self.peek_char() == Some(',') {
                    self.advance();
                    self.skip_trivia();
                    items.push(self.parse_expression()?);
                    self.skip_trivia();
                }
                self.expect_char(')')?;
                return Ok(Expr::Tuple(items));
            }
            self.expect_char(')')?;
            return Ok(first);
        }
        if self
            .peek_char()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            return Ok(Expr::Int(self.parse_number()?));
        }
        if self.starts_with_keyword("true") {
            self.advance_keyword("true");
            return Ok(Expr::Bool(true));
        }
        if self.starts_with_keyword("false") {
            self.advance_keyword("false");
            return Ok(Expr::Bool(false));
        }
        Ok(Expr::Var(self.parse_identifier()?))
    }

    fn parse_array_or_comprehension(&mut self) -> Result<Expr, String> {
        self.expect_char('[')?;
        self.skip_trivia();
        if self.starts_with_keyword("for") {
            return Err("Array comprehension requires an element expression.".to_string());
        }
        let first = self.parse_expression()?;
        self.skip_trivia();
        if self.starts_with_keyword("for") {
            self.expect_keyword("for")?;
            let var = self.parse_identifier()?;
            self.skip_trivia();
            self.expect_keyword("in")?;
            let iter = self.parse_expression()?;
            self.skip_trivia();
            let filter = if self.starts_with_keyword("if") {
                self.advance_keyword("if");
                Some(self.parse_expression()?)
            } else {
                None
            };
            self.skip_trivia();
            self.expect_char(']')?;
            return Ok(Expr::Comprehension {
                item: Box::new(first),
                var,
                iter: Box::new(iter),
                filter: filter.map(Box::new),
            });
        }
        let mut items = vec![first];
        while self.peek_char() == Some(',') {
            self.advance();
            self.skip_trivia();
            items.push(self.parse_expression()?);
            self.skip_trivia();
        }
        self.expect_char(']')?;
        Ok(Expr::Array(items))
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expr>, String> {
        self.skip_trivia();
        if self.peek_char() == Some(')') {
            self.advance();
            return Ok(Vec::new());
        }
        let mut args = vec![self.parse_expression()?];
        self.skip_trivia();
        while self.peek_char() == Some(',') {
            self.advance();
            self.skip_trivia();
            args.push(self.parse_expression()?);
            self.skip_trivia();
        }
        self.expect_char(')')?;
        Ok(args)
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.expect_char('"')?;
        let mut value = String::new();
        while !self.is_eof() {
            let ch = self
                .advance()
                .ok_or_else(|| "Unterminated string.".to_string())?;
            if ch == '"' {
                break;
            }
            value.push(ch);
        }
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<i64, String> {
        let start = self.pos;
        while self
            .peek_char()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            self.advance();
        }
        self.source[start..self.pos]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .map_err(|_| "Invalid integer literal.".to_string())
    }

    fn parse_param_list(&mut self) -> Result<Vec<String>, String> {
        self.skip_trivia();
        if self.peek_char() == Some(')') {
            return Ok(Vec::new());
        }
        let mut params = vec![self.parse_identifier()?];
        self.skip_trivia();
        while self.peek_char() == Some(',') {
            self.advance();
            self.skip_trivia();
            params.push(self.parse_identifier()?);
            self.skip_trivia();
        }
        Ok(params)
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        self.skip_trivia();
        let start = self.pos;
        let first = self
            .peek_char()
            .ok_or_else(|| "Expected identifier.".to_string())?;
        if !first.is_ascii_alphabetic() && first != '_' {
            return Err(format!("Expected identifier, found '{first}'."));
        }
        self.advance();
        while self
            .peek_char()
            .map(|c| c.is_ascii_alphanumeric() || c == '_')
            .unwrap_or(false)
        {
            self.advance();
        }
        Ok(self.source[start..self.pos].iter().collect())
    }

    fn skip_trivia(&mut self) {
        loop {
            while self
                .peek_char()
                .map(|c| c.is_whitespace())
                .unwrap_or(false)
            {
                self.advance();
            }
            if self.match_str("#") {
                while self.peek_char().is_some_and(|c| c != '\n') {
                    self.advance();
                }
                continue;
            }
            if self.starts_with_keyword("using") {
                while self.peek_char().is_some_and(|c| c != '\n') {
                    self.advance();
                }
                continue;
            }
            break;
        }
    }

    fn is_statement_boundary(&self) -> bool {
        matches!(self.peek_char(), Some('\n') | None)
    }

    fn starts_with_keyword(&self, keyword: &str) -> bool {
        if !self.source[self.pos..]
            .iter()
            .collect::<String>()
            .starts_with(keyword)
        {
            return false;
        }
        let next = self.source.get(self.pos + keyword.len());
        !next
            .map(|c| c.is_ascii_alphanumeric() || *c == '_')
            .unwrap_or(false)
    }

    fn expect_keyword(&mut self, keyword: &str) -> Result<(), String> {
        if self.starts_with_keyword(keyword) {
            self.advance_keyword(keyword);
            Ok(())
        } else {
            Err(format!("Expected keyword `{keyword}`."))
        }
    }

    fn advance_keyword(&mut self, keyword: &str) {
        self.pos += keyword.len();
    }

    fn expect_char(&mut self, ch: char) -> Result<(), String> {
        if self.peek_char() == Some(ch) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected '{ch}'."))
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += 1;
        Some(ch)
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn match_str(&mut self, target: &str) -> bool {
        if self.source[self.pos..]
            .iter()
            .collect::<String>()
            .starts_with(target)
        {
            self.pos += target.len();
            true
        } else {
            false
        }
    }

    fn match_one<'a>(&mut self, options: &'a [&str]) -> Option<&'a str> {
        for option in options {
            if self.match_str(option) {
                return Some(option);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_blueprint_julia(path: &str) -> String {
        let content = std::fs::read_to_string(path).expect("read lab md");
        let marker = "```julia";
        let start = content
            .find(marker)
            .unwrap_or_else(|| panic!("no julia block in {path}"));
        let after = &content[start + marker.len()..];
        let end = after.find("```").expect("unclosed julia fence");
        after[..end].trim().to_string()
    }

    #[test]
    fn mod_01_blueprint_prints_inverse_values() {
        let code = extract_blueprint_julia("research-docs/julia-crypto/mod_01_lab.md");
        let (stdout, _) = execute(&code).expect("mod_01 blueprint should run");
        assert!(stdout.iter().any(|line| line == "4"), "stdout: {stdout:?}");
        assert!(stdout.iter().any(|line| line == "3"), "stdout: {stdout:?}");
    }

    #[test]
    fn mod_02_blueprint_prints_powermod_result() {
        let code = extract_blueprint_julia("research-docs/julia-crypto/mod_02_lab.md");
        let (stdout, _) = execute(&code).expect("mod_02 blueprint should run");
        assert!(
            stdout.iter().any(|line| line == "31"),
            "stdout: {stdout:?}"
        );
    }

    #[test]
    fn mod_inverse_error_when_not_invertible() {
        let code = r#"
function extended_gcd(a, b)
    if b == 0
        return (a, 1, 0)
    end
    g, x1, y1 = extended_gcd(b, a % b)
    return (g, y1, x1 - (a ÷ b) * y1)
end

function modInverse(a, n)
    g, x, _ = extended_gcd(a, n)
    if g != 1
        error("Modular inverse does not exist")
    else
        return (x % n + n) % n
    end
end

println(modInverse(2, 4))
"#;
        let err = execute(code).expect_err("should error");
        assert!(
            err.contains("Modular inverse does not exist"),
            "got: {err}"
        );
    }

    #[test]
    fn index_assign_pads_int_arrays_with_zero() {
        let code = r#"
function padded_sum()
    a = [1, 2]
    a[5] = 9
    return a[3] + a[4]
end
padded_sum()
"#;
        let (_, result) = execute(code).expect("int array padding should run");
        assert_eq!(result.as_deref(), Some("0"));
    }

    #[test]
    fn index_assign_pads_string_arrays_with_empty_string() {
        let code = r#"
function padded_slot()
    s = ["hello"]
    s[3] = "!"
    return s[2] == ""
end
padded_slot()
"#;
        let (_, result) = execute(code).expect("string array padding should run");
        assert_eq!(result.as_deref(), Some("true"));
    }

    #[test]
    fn index_assign_mixed_array_pads_with_unit_and_reports_uninitialized_math() {
        let code = r#"
a = [1, "x"]
a[4] = 7
println(a[2] + a[3])
"#;
        let err = execute(code).expect_err("mixed array padding should use nothing");
        assert!(
            err.contains("Uninitialized element"),
            "got: {err}"
        );
    }

    #[test]
    fn infinite_while_loop_hits_instruction_quota() {
        let code = r#"
while true
    println("loop")
end
"#;
        let err = execute_with_budget(code, 100).expect_err("should halt infinite loop");
        assert!(
            err.contains("Instruction quota exceeded"),
            "got: {err}"
        );
    }

    #[test]
    fn large_for_loop_hits_instruction_quota() {
        let code = r#"
total = 0
for i in 1:1000000
    total = total + i
end
total
"#;
        let err = execute_with_budget(code, 500).expect_err("should halt large for loop");
        assert!(
            err.contains("Instruction quota exceeded"),
            "got: {err}"
        );
    }

    #[test]
    fn blueprint_still_runs_within_default_budget() {
        let code = extract_blueprint_julia("research-docs/julia-crypto/mod_01_lab.md");
        execute(&code).expect("mod_01 blueprint should stay within default instruction budget");
    }
}
