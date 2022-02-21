use crate::{ast::Node, error::Error, parser};
use inkwell::{
    self, builder::Builder, context::Context, execution_engine::JitFunction, OptimizationLevel,
};

type SumFunc = unsafe extern "C" fn() -> i32;

pub fn compile_string(source: &str) -> Result<i32, Error> {
    let ast = parser::parse(source)?;
    jit_compile(ast)
}

pub fn jit_compile(ast: Node) -> Result<i32, Error> {
    let context = Context::create();
    let module = context.create_module("calculator");
    let builder = context.create_builder();
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::ExecutionEngine {
            message: e.to_string(),
        })?;
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let calc = module.add_function("calc", fn_type, None);
    let basic_block = context.append_basic_block(calc, "entry");
    builder.position_at_end(basic_block);

    let return_val = compile_ast(ast, &context, &builder);
    builder.build_return(Some(&return_val));
    let calc_fn: JitFunction<SumFunc> = unsafe { execution_engine.get_function("calc") }?;

    Ok(unsafe { calc_fn.call() })
}

pub fn compile_ast<'c, 'b>(
    ast: Node,
    context: &'c Context,
    builder: &'b Builder<'c>,
) -> inkwell::values::IntValue<'c> {
    match ast {
        Node::Number(n) => {
            let i32_type = context.i32_type();
            i32_type.const_int(n as u64, false)
        }
        Node::Add(x, y) => {
            let i32_type_x = compile_ast(*x, context, builder);
            let i32_type_y = compile_ast(*y, context, builder);

            builder.build_int_add(i32_type_x, i32_type_y, "sum")
        }
        Node::Sub(x, y) => {
            let i32_type_x = compile_ast(*x, context, builder);
            let i32_type_y = compile_ast(*y, context, builder);

            builder.build_int_sub(i32_type_x, i32_type_y, "sub")
        }
        Node::Mul(x, y) => {
            let i32_type_x = compile_ast(*x, context, builder);
            let i32_type_y = compile_ast(*y, context, builder);

            builder.build_int_mul(i32_type_x, i32_type_y, "mul")
        }
        Node::Div(x, y) => {
            let i32_type_x = compile_ast(*x, context, builder);
            let i32_type_y = compile_ast(*y, context, builder);

            builder.build_int_unsigned_div(i32_type_x, i32_type_y, "div")
        }
    }
}
