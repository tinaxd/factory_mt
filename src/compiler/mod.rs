mod c;
mod global;
mod layout;
mod llvm;

use std::collections::HashMap;

use inkwell::{
    basic_block::BasicBlock,
    builder::{Builder, BuilderError},
    context::Context,
    module::Linkage,
    values::{BasicValueEnum, PointerValue},
    IntPredicate,
};

use crate::ast::{
    Expression, FuncDefStatement, LiteralExpression, PrimitiveType, ReturnStatement, Statement,
    TypeExpression,
};

use self::c::UniqueNameGenerator;

#[derive(Debug)]
pub struct Compiler {
    ctx: Context,
}

impl Compiler {
    pub fn new() -> Self {
        let ctx = Context::create();
        Self { ctx }
    }

    pub fn compile_top(&mut self, top_stmt: &Statement) -> inkwell::module::Module {
        let module = self.ctx.create_module("main");

        // let tag_type = self.ctx.i8_type();
        // let int_type = self.ctx.i32_type();
        // let factory_value = self
        //     .ctx
        //     .struct_type(&[tag_type.into(), int_type.into()], false);

        // module.add_global(factory_value, None, "factory_value");

        let builder = self.ctx.create_builder();

        let mut unit_compiler = UnitCompiler::new("top_level".to_string(), &self.ctx, &module);
        unit_compiler
            .compile_stmt(
                &Statement::FuncDef(FuncDefStatement::new(
                    "top_level".to_string(),
                    vec![],
                    top_stmt.clone(),
                    TypeExpression::Primitive(PrimitiveType::Integer),
                )),
                &builder,
            )
            .unwrap();

        module
    }

    pub fn dump_module(module: &inkwell::module::Module) {
        module.print_to_stderr();
    }
}

type AllocaMap<'a> = HashMap<String, PointerValue<'a>>;

#[derive(Debug)]
pub struct UnitCompiler<'a, 'b> {
    current_function: String,

    ctx: &'a Context,
    module: &'b inkwell::module::Module<'a>,

    allocas: AllocaMap<'a>,

    rand: UniqueNameGenerator,
}

type LocalVarName = String;

type BuilderResult<T> = Result<T, BuilderError>;

impl<'a: 'b, 'b> UnitCompiler<'a, 'b> {
    pub fn new(
        function_name: String,
        ctx: &'a Context,
        module: &'b inkwell::module::Module<'a>,
    ) -> Self {
        Self {
            current_function: function_name,
            ctx,
            module,
            allocas: HashMap::new(),
            rand: UniqueNameGenerator::new(),
        }
    }

    fn child_compiler(&self, function_name: String, params: AllocaMap<'a>) -> Self {
        Self {
            current_function: function_name,
            ctx: self.ctx,
            module: self.module,
            allocas: params,
            rand: UniqueNameGenerator::new(),
        }
    }

    pub fn map_to_llvm_type(&self, ty: &TypeExpression) -> inkwell::types::AnyTypeEnum<'a> {
        match ty {
            TypeExpression::Primitive(prim) => match prim {
                PrimitiveType::Integer => inkwell::types::AnyTypeEnum::IntType(self.ctx.i32_type()),
            },
        }
    }

    pub fn compile_expr(
        &mut self,
        expr: &Expression,
        builder: &Builder<'a>,
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

                let new_name = "bin_op_result";

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
                let var_ty = self.ctx.i32_type();
                let ptr = self.allocas.get(var_name).expect("variable not found");
                let new_name = "load_result";
                let new_value = builder.build_load(var_ty, *ptr, new_name)?;
                Ok(new_value.into())
                // TODO: fallback to dictionary search when var_name is not found in local scope
            }
            Expression::FunCall(func) => {
                let fun = func.callee();
                let args = func.args();

                let fun_name = match fun {
                    Expression::Name(n) => n.get_name(),
                    _ => todo!("not supported yet"),
                };

                let exprs: Vec<_> = args
                    .iter()
                    .map(|e| self.compile_expr(e, builder))
                    .map(|e| e.unwrap().into())
                    .collect();

                let func = self
                    .module
                    .get_function(fun_name)
                    .expect("function not found");

                let val = builder.build_call(func, &exprs, "call_result")?;
                Ok(val.try_as_basic_value().left().unwrap())
            }
            _ => unimplemented!(),
        }
    }

    fn alloca_at_start(
        &mut self,
        name: &str,
        current_func: BasicBlock,
    ) -> BuilderResult<PointerValue<'a>> {
        let builder = self.ctx.create_builder();
        builder.position_at_end(current_func);
        if let Some(first_inst) = builder.get_insert_block().unwrap().get_first_instruction() {
            builder.position_before(&first_inst);
        }
        let var_ty = self.ctx.i32_type();
        let ptr = builder.build_alloca(var_ty, name)?;
        self.allocas.insert(name.to_string(), ptr);
        Ok(ptr)
    }

    pub fn compile_stmt(&mut self, stmt: &Statement, builder: &Builder<'a>) -> BuilderResult<()> {
        println!("compiling stmt: {:?}", stmt);
        match stmt {
            Statement::Expression(expr) => {
                let var = self.compile_expr(expr, builder)?;
                // discard it
                Ok(())
            }
            Statement::Assignment(assign) => {
                let name = assign.name();

                let alloca = match self.allocas.get(name) {
                    None => {
                        let new_alloca = self.alloca_at_start(
                            name,
                            builder
                                .get_insert_block()
                                .unwrap()
                                .get_parent()
                                .unwrap()
                                .get_first_basic_block()
                                .unwrap(),
                        )?;
                        new_alloca
                    }
                    Some(alloca) => *alloca,
                };

                let value = self.compile_expr(assign.expression(), builder)?;
                builder.build_store(alloca, value.into_int_value());
                Ok(())
            }
            Statement::Block(blk) => {
                for stmt in blk.iter() {
                    self.compile_stmt(stmt, builder)?;
                }
                Ok(())
            }
            Statement::Conditional(cond) => {
                // basic blocks
                let current_func = builder.get_insert_block().unwrap().get_parent().unwrap();
                let mut then_bb = self.ctx.append_basic_block(current_func, "then");
                let mut else_bb = self.ctx.append_basic_block(current_func, "else");
                let merge_bb = self.ctx.append_basic_block(current_func, "merge");

                // evaluate condition
                let cond_expr = cond.cond();
                let cond_value = self.compile_expr(cond_expr, builder)?;

                builder.build_conditional_branch(cond_value.into_int_value(), then_bb, else_bb);

                // generate true
                builder.position_at_end(then_bb);
                self.compile_stmt(cond.then(), builder)?;
                builder.build_unconditional_branch(merge_bb);
                then_bb = builder.get_insert_block().unwrap();

                // generate else
                builder.position_at_end(else_bb);
                if let Some(else_stmt) = cond.otherwise() {
                    self.compile_stmt(else_stmt, builder)?;
                }
                builder.build_unconditional_branch(merge_bb);
                else_bb = builder.get_insert_block().unwrap();

                // generate merge
                builder.position_at_end(merge_bb);
                Ok(())
            }
            Statement::While(wh) => {
                let cond = wh.cond();
                let body = wh.body();

                let func = builder.get_insert_block().unwrap().get_parent().unwrap();
                let cond_bb = self.ctx.append_basic_block(func, "cond");
                let body_bb = self.ctx.append_basic_block(func, "body");
                let merge_bb = self.ctx.append_basic_block(func, "merge");

                builder.build_unconditional_branch(cond_bb);

                // build cond
                builder.position_at_end(cond_bb);
                let cond_value = self.compile_expr(cond, builder)?;
                builder.build_conditional_branch(cond_value.into_int_value(), body_bb, merge_bb);

                // build body
                builder.position_at_end(body_bb);
                self.compile_stmt(body, builder)?;
                builder.build_unconditional_branch(cond_bb);

                // build merge
                builder.position_at_end(merge_bb);
                Ok(())
            }
            Statement::FuncDef(def) => {
                let func_name = def.name();
                let func_params = def.params();
                let func_body = def.body();
                let func_return = def.return_type();

                let return_type = self.map_to_llvm_type(func_return);
                let fn_type = return_type.into_int_type().fn_type(
                    &func_params
                        .iter()
                        .map(|p| self.map_to_llvm_type(p.ty().unwrap()).try_into().unwrap())
                        .collect::<Vec<_>>()
                        .as_ref(),
                    false,
                );

                let func = self
                    .module
                    .add_function(func_name, fn_type, Some(Linkage::External));

                let entry_bb = self.ctx.append_basic_block(func, "entry");
                let body_bb = self.ctx.append_basic_block(func, "body");

                builder.position_at_end(entry_bb);
                builder.build_unconditional_branch(body_bb).unwrap();

                builder.position_at_end(body_bb);

                let allocas: AllocaMap = func_params
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let param = func.get_nth_param(i as u32).unwrap();
                        param.set_name(p.name());
                        let alloca = self.alloca_at_start(p.name(), body_bb).unwrap();
                        builder.build_store(alloca, param);
                        (p.name().to_string(), alloca)
                    })
                    .collect();

                let mut unit = self.child_compiler(func_name.to_string(), allocas);

                unit.compile_stmt(func_body, builder)?;
                // add return in case it's not there
                // it should be optimized away if it's already there
                unit.compile_stmt(&Statement::Return(ReturnStatement::new_null()), builder)?;

                Ok(())
            }
            Statement::Return(ret) => match ret.expression() {
                None => {
                    builder.build_return(None);
                    Ok(())
                }
                Some(e) => {
                    let val = self.compile_expr(e, builder)?;
                    builder.build_return(Some(&val));
                    Ok(())
                }
            },
            _ => unimplemented!(),
        }
    }
}
