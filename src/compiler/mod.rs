mod c;
mod global;
mod layout;

use crate::ast::{Expression, LiteralExpression, Statement};

use self::c::UniqueNameGenerator;

#[derive(Debug)]
pub struct Compiler {
    codes: String,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            codes: String::new(),
        }
    }

    pub fn compile_top(&mut self, top_stmt: &Statement) {
        let mut unit_compiler = UnitCompiler::new("top_level".to_string(), &vec![]);
        unit_compiler.compile_fundef("top_level", &vec![], top_stmt);
        self.codes = unit_compiler.collect_codes();
    }

    pub fn get_output(&self) -> String {
        self.codes.clone()
    }
}

#[derive(Debug)]
pub struct UnitCompiler {
    current_function: String,
    params: Vec<String>,

    code: Vec<String>,
    gen: UniqueNameGenerator,
}

type LocalVarName = String;

impl UnitCompiler {
    pub fn new(function_name: String, params: &[String]) -> UnitCompiler {
        Self {
            current_function: function_name,
            params: params.to_vec(),
            code: Vec::new(),
            gen: UniqueNameGenerator::new(),
        }
    }

    fn add_line(&mut self, line: impl Into<String>) {
        self.code.push(line.into());
    }

    pub fn compile_expr(&mut self, expr: &Expression) -> LocalVarName {
        match expr {
            Expression::Binary(bin) => {
                let op = bin.op();
                let left = bin.left();
                let right = bin.right();
                // generate operands
                let lhs = self.compile_expr(left);
                let rhs = self.compile_expr(right);

                // generate operator
                use crate::ast::BinaryOperator::*;
                let (ty, op, convfun) = match op {
                    Plus => ("int32_t", "+", "factory_alloc_integer"),
                    Minus => ("int32_t", "-", "factory_alloc_integer"),
                    Times => ("int32_t", "*", "factory_alloc_integer"),
                    Divide => ("int32_t", "/", "factory_alloc_integer"),
                    Modulo => ("int32_t", "%", "factory_alloc_integer"),
                    Eq => ("bool", "==", "factory_alloc_boolean"),
                    Neq => ("bool", "!=", "factory_alloc_boolean"),
                    Lt => ("bool", "<", "factory_alloc_boolean"),
                    Le => ("bool", "<=", "factory_alloc_boolean"),
                    Gt => ("bool", ">", "factory_alloc_boolean"),
                    Ge => ("bool", ">=", "factory_alloc_boolean"),
                };
                let result_tmp_var = self.gen.next_var_name();
                self.add_line(format!(
                    "{} {} = {} {} {};",
                    ty, result_tmp_var, lhs, op, rhs
                ));
                let result_var = self.gen.next_var_name();
                self.add_line(format!(
                    "struct FactoryObject* {} = {}({});",
                    result_var, convfun, result_tmp_var
                ));
                result_var
            }
            Expression::Literal(lit) => match lit {
                LiteralExpression::Integer(i) => {
                    let result_var = self.gen.next_var_name();
                    self.add_line(format!("int32_t {} = {};", result_var, i));
                    result_var
                }
                LiteralExpression::String(s) => {
                    todo!()
                }
                _ => unimplemented!(),
            },
            Expression::Name(name) => {
                let var_name = name.get_name();
                var_name.to_string()
                // TODO: fallback to dictionary search when var_name is not found in local scope
            }
            Expression::FunCall(func) => {
                let callee = func.callee();
                let args = func.args();

                let callee_name = match callee {
                    Expression::Name(name) => name.get_name(),
                    _ => todo!("closure is not supported yet"),
                };

                let arg_vars: Vec<String> = args.iter().map(|a| self.compile_expr(a)).collect();

                // generate call
                let result_var = self.gen.next_var_name();
                self.add_line(format!(
                    "struct FactoryObject* {} = {}({});",
                    result_var,
                    callee_name,
                    arg_vars.join(", ")
                ));
                result_var
            }
            _ => unimplemented!(),
        }
    }

    pub fn compile_stmt(&mut self, stmt: &Statement) {
        println!("compiling stmt: {:?}", stmt);
        match stmt {
            Statement::Expression(expr) => {
                let var = self.compile_expr(expr);
                self.add_line(format!("{};", var))
            }
            Statement::Assignment(assign) => {
                let name = assign.name();

                let value = self.compile_expr(assign.expression());
                self.add_line(format!("{} = {};", name, value));
            }
            Statement::Block(blk) => {
                self.add_line("{".to_string());
                for stmt in blk.iter() {
                    self.compile_stmt(stmt);
                }
                self.add_line("}".to_string());
            }
            Statement::Conditional(cond) => {
                // evaluate condition
                let cond_expr = cond.cond();
                let cond_value = self.compile_expr(cond_expr);

                self.add_line(format!("if ({})", cond_value));
                // generate body
                let body = cond.then();
                self.compile_stmt(body);

                // generate else
                if let Some(else_body) = cond.otherwise() {
                    self.add_line("else".to_string());
                    self.compile_stmt(else_body);
                }
            }
            Statement::While(wh) => {
                let cond = wh.cond();
                let body = wh.body();

                let cond_label = self.gen.next_label_name();
                let body_end_label = self.gen.next_label_name();

                // evaluate condition
                self.add_line(format!("{}:", cond_label));
                let cond_value = self.compile_expr(cond);
                self.add_line(format!("if (!{})", cond_value));
                self.add_line(format!("goto {};", body_end_label));
                self.compile_stmt(body);
                self.add_line(format!("goto {};", cond_label));
                self.add_line(format!("{}: ;", body_end_label));
            }
            Statement::FuncDef(def) => {
                let func_name = def.name();
                let func_params = def.params();
                let func_body = def.body();

                self.compile_fundef(func_name, func_params, func_body);
            }
            Statement::Return(ret) => match ret.expression() {
                None => {
                    self.add_line("return factory_alloc_null();".to_string());
                }
                Some(e) => {
                    let val = self.compile_expr(e);
                    self.add_line(format!("return {};", val));
                }
            },
            _ => unimplemented!(),
        }
    }

    fn gen_header(&mut self) {
        self.add_line(format!("struct FactoryObject* {}(", self.current_function));
        self.params.clone().iter().for_each(|p| {
            self.add_line(format!("struct FactoryObject* {},", p));
        });
        self.add_line(")".to_string());
    }

    fn compile_fundef(&mut self, name: &str, params: &[String], body: &Statement) {
        let mut unit = UnitCompiler::new(name.to_string(), params);

        unit.gen_header();
        unit.compile_stmt(body);

        self.add_line(unit.collect_codes());
    }

    // first code will always be a main code
    fn collect_codes(&self) -> String {
        self.code.join("\n")
    }
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub jmp_to_label: Option<String>,
    pub this_label: Vec<String>,
}
