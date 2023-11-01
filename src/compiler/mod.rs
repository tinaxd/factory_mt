mod c;
mod global;
mod layout;
mod llvm;

use std::collections::HashMap;

use inkwell::{
    builder::{Builder, BuilderError},
    context::Context,
    values::{BasicValueEnum, PointerValue},
    IntPredicate,
};

use crate::ast::{Expression, LiteralExpression, Statement};

use self::c::UniqueNameGenerator;

#[derive(Debug)]
pub struct Compiler {
    ctx: Context,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ctx: Context::create(),
        }
    }

    pub fn compile_top(&mut self, top_stmt: &Statement) {
        let module = self.ctx.create_module("main");

        let tag_type = self.ctx.i8_type();
        let int_type = self.ctx.i32_type();
        let factory_value = self
            .ctx
            .struct_type(&[tag_type.into(), int_type.into()], false);

        module.add_global(factory_value, None, "factory_value");

        let mut unit_compiler =
            UnitCompiler::new("top_level".to_string(), &vec![], &self.ctx, module);
        unit_compiler.compile_fundef("top_level", &vec![], top_stmt);
    }
}

#[derive(Debug)]
pub struct UnitCompiler<'a> {
    current_function: String,
    params: Vec<String>,

    ctx: &'a Context,
    module: inkwell::module::Module<'a>,

    allocas: HashMap<String, PointerValue<'a>>,

    rand: UniqueNameGenerator,
}

type LocalVarName = String;

type BuilderResult<T> = Result<T, BuilderError>;

impl<'a> UnitCompiler<'a> {
    pub fn new(
        function_name: String,
        params: &[String],
        ctx: &'a Context,
        module: inkwell::module::Module<'a>,
    ) -> Self {
        Self {
            current_function: function_name,
            params: params.to_vec(),
            ctx,
            module,
            allocas: HashMap::new(),
            rand: UniqueNameGenerator::new(),
        }
    }

    pub fn compile_expr(
        &mut self,
        expr: &Expression,
        builder: Builder<'a>,
    ) -> BuilderResult<BasicValueEnum<'a>> {
        match expr {
            Expression::Binary(bin) => {
                let op = bin.op();
                let left = bin.left();
                let right = bin.right();
                // generate operands
                let lhs = self.compile_expr(left, builder)?;
                let rhs = self.compile_expr(right, builder)?;

                // limit to IntValue
                let lhs_int = match lhs {
                    BasicValueEnum::IntValue(i) => i,
                    _ => todo!("not supported yet"),
                };
                let rhs_int = match rhs {
                    BasicValueEnum::IntValue(i) => i,
                    _ => todo!("not supported yet"),
                };

                let new_name = self.rand.next_var_name();

                // generate operator
                use crate::ast::BinaryOperator::*;
                let new_value: BasicValueEnum<'a> = match op {
                    Plus => builder.build_int_add(lhs_int, rhs_int, &new_name)?.into(),
                    Minus => builder.build_int_sub(lhs_int, rhs_int, &new_name)?.into(),
                    Times => builder.build_int_mul(lhs_int, rhs_int, &new_name)?.into(),
                    Divide => todo!("not supported yet"),
                    Modulo => todo!("not supported yet"),
                    Eq => builder
                        .build_int_compare(IntPredicate::EQ, lhs_int, rhs_int, &new_name)?
                        .into(),
                    Neq => builder
                        .build_int_compare(IntPredicate::NE, lhs_int, rhs_int, &new_name)?
                        .into(),
                    Lt => builder
                        .build_int_compare(IntPredicate::SLT, lhs_int, rhs_int, &new_name)?
                        .into(),
                    Le => builder
                        .build_int_compare(IntPredicate::SLE, lhs_int, rhs_int, &new_name)?
                        .into(),
                    Gt => builder
                        .build_int_compare(IntPredicate::SGT, lhs_int, rhs_int, &new_name)?
                        .into(),
                    Ge => builder
                        .build_int_compare(IntPredicate::SGE, lhs_int, rhs_int, &new_name)?
                        .into(),
                };
                Ok(new_value)
            }
            Expression::Literal(lit) => match lit {
                LiteralExpression::Integer(i) => {
                    let i = *i;
                    let new_value = self.ctx.i32_type().const_int(i as u64, true);
                    Ok(new_value.into())
                }
                LiteralExpression::String(s) => {
                    todo!()
                }
                _ => unimplemented!(),
            },
            Expression::Name(name) => {
                let var_name = name.get_name();
                let fv = self.module.get_global("factory_value").unwrap();
                let ptr = self.allocas.get(var_name).expect("variable not found");
                let new_name = self.rand.next_var_name();
                let gep = builder.build_gep(pointee_ty, ptr, ordered_indexes, name)
                let new_value = builder.build_load(fv, *ptr, &new_name)?;
                Ok(new_value.into())
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
