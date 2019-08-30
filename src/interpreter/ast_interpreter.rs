use std::collections::HashMap;
use crate::parser::{ast::*, tokens::*};
use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    builtins::{api_functions::*, reserved_functions::*},
    data::Data,
    message::*,
    variable_handler::*,
};

fn priority_match<'a>(name: &str, lit: &'a Literal) -> Option<&'a Literal>{
    match name {
        "button" => {
            if let Literal::ObjectLiteral{properties} = lit {
                properties.get("accept")
            } else {
                Some(lit)
            }
        },
        _   => Some(lit)
    }
}

//TODO: add warning when comparing some objects
fn match_obj(lit1: &Literal, lit2: &Literal) -> Literal {
    match (&lit1, &lit2) {
        (Literal::FunctionLiteral{name: n1, value: v1}, Literal::FunctionLiteral{name: n2, value: v2}) => {
            println!("fl to fl");
            match (priority_match(n1, v1), priority_match(n2, v2)) {
                (Some(l1), Some(l2)) => match_obj(l1, l2),
                (_, _) => Literal::boolean(false)
            }
        },
        (Literal::FunctionLiteral{name, value}, lit) => {
            println!("fl to lit");
            match priority_match(name, value) {
                Some(l1) => match_obj(l1, lit),
                _ => Literal::boolean(false)
            }
        },
        (lit, Literal::FunctionLiteral{name, value}) => {
            println!("lit to fl");
            match priority_match(name, value) {
                Some(l1) => match_obj(l1, lit),
                _ => Literal::boolean(false)
            }
        },
        (Literal::ObjectLiteral{properties: p1}, Literal::ObjectLiteral{properties: p2}) => {
            println!("obj to obj");
            match (p1.get("object"), p2.get("object")) {
                (Some(l1), Some(l2)) => match (priority_match("object", l1), priority_match("object", l2)) {
                    (Some(l1), Some(l2)) => match_obj(l1, l2),
                    (_, _) => Literal::boolean(false)
                },
                (_, _) => Literal::boolean(false)
            }
        },
        (Literal::ObjectLiteral{properties}, lit) => {
            println!("obj to lit");
            match properties.get("object") {
                Some(l1) => match priority_match("object", l1) {
                    Some(l1) => match_obj(l1, lit),
                    _ => Literal::boolean(false)
                },
                _ => Literal::boolean(false)
            }
        },
        (lit, Literal::ObjectLiteral{properties}) => {
            println!("lit to obj");
            match properties.get("object") {
                Some(l2) => match priority_match("object", l2) {
                    Some(l2) => match_obj(l2, lit),
                    _ => Literal::boolean(false)
                },
                _ => Literal::boolean(false)
            }
        },
        (Literal::ArrayLiteral{items: i1}, Literal::ArrayLiteral{items: i2}) => Literal::boolean(i1 == i2),
        (Literal::ArrayLiteral{items}, lit) => Literal::boolean(items.contains(lit)),
        (lit, Literal::ArrayLiteral{items}) => Literal::boolean(items.contains(lit)),
        (l1, l2) => {
            println!("lit to lit");
            Literal::boolean(l1 == l2)
        }
    }
}

//TODO: add warning when comparing some objects
fn cmp_lit(
    infix: &Infix,
    lit1: Result<SmartLiteral, ErrorInfo>,
    lit2: Result<SmartLiteral, ErrorInfo>,
) -> Result<SmartLiteral, ErrorInfo> {
    match (infix, lit1, lit2) {
        (Infix::NotEqual, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 != l2), interval: l1.interval.to_owned()}),
        (Infix::Equal, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 == l2), interval: l1.interval.to_owned()}),
        (Infix::GreaterThanEqual, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 >= l2), interval: l1.interval.to_owned()}),
        (Infix::LessThanEqual, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 <= l2), interval: l1.interval.to_owned()}),
        (Infix::GreaterThan, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 > l2), interval: l1.interval.to_owned()}),
        (Infix::LessThan, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 < l2), interval: l1.interval.to_owned()}),
        (Infix::Or, Ok(l1), Ok(..)) => Ok(SmartLiteral{literal: Literal::boolean(true), interval: l1.interval.to_owned()}),
        (Infix::Or, Ok(l1), Err(..)) => Ok(SmartLiteral{literal: Literal::boolean(true), interval: l1.interval.to_owned()}),
        (Infix::Or, Err(e), Ok(..)) => Ok(SmartLiteral{literal: Literal::boolean(true), interval: e.interval.to_owned()}),
        (Infix::And, Ok(l1), Ok(..)) => Ok(SmartLiteral{literal: Literal::boolean(true), interval: l1.interval.to_owned()}),
        (Infix::Adition, Ok(l1), Ok(l2)) => l1 + l2,
        (Infix::Substraction, Ok(l1), Ok(l2)) => l1 - l2,
        (Infix::Divide, Ok(l1), Ok(l2)) => l1 / l2,
        (Infix::Multiply, Ok(l1), Ok(l2)) => l1 * l2,
        (Infix::Match, Ok(ref l1), Ok(ref l2)) => {
            println!(" MATCH ");
            Ok(SmartLiteral{literal: match_obj(&l1.literal, &l2.literal), interval: l1.interval.to_owned()})
        },
        (_, Ok(l1), ..) => Ok(SmartLiteral{literal: Literal::boolean(false), interval: l1.interval.to_owned()}),
        (_, Err(e), ..) => Ok(SmartLiteral{literal: Literal::boolean(false), interval: e.interval.to_owned()}),
    }
}

fn check_if_ident(expr: &Expr) -> bool {
    println!("check_if_ident ",);
    match expr {
        Expr::LitExpr { .. } => true,
        Expr::IdentExpr(..) => true,
        Expr::BuilderExpr(..) => true,
        Expr::ComplexLiteral(..) => true,
        _ => false,
    }
}

pub fn evaluate_condition(
    infix: &Infix,
    expr1: &Expr,
    expr2: &Expr,
    data: &mut Data,
) -> Result<SmartLiteral, ErrorInfo> {
    match (expr1, expr2) {
        (exp1, ..) if Infix::Not == *infix && check_if_ident(exp1) => {
            match get_var_from_ident(exp1, data) {
                Ok(SmartLiteral {
                    literal: Literal::BoolLiteral{value: false, ..},
                    interval,
                }) => Ok(SmartLiteral {
                    literal: Literal::boolean(true),
                    interval,
                }),
                Ok(SmartLiteral {
                    literal: Literal::IntLiteral{value: 0, ..},
                    interval,
                }) => Ok(SmartLiteral {
                    literal: Literal::boolean(true),
                    interval,
                }),
                Ok(SmartLiteral { interval, .. }) => Ok(SmartLiteral {
                    literal: Literal::boolean(false),
                    interval,
                }),
                Err(err) => Ok(SmartLiteral {
                    literal: Literal::boolean(true),
                    interval: err.interval,
                }),
            }
        }
        (exp1, exp2) if check_if_ident(exp1) && check_if_ident(exp2) => {
            println!("Start match ");
            let lit = cmp_lit(infix, get_var_from_ident(exp1, data), get_var_from_ident(exp2, data))?;
            Ok(SmartLiteral {literal: lit.literal, interval: interval_from_expr(exp1)})
        },
        (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp1, exp2)) => cmp_lit(
            infix,
            evaluate_condition(i1, ex1, ex2, data),
            evaluate_condition(i2, exp1, exp2, data),
        ),
        (Expr::InfixExpr(i1, ex1, ex2), exp) => cmp_lit(
            infix,
            evaluate_condition(i1, ex1, ex2, data),
            gen_literal_form_exp(exp, data),
        ),
        (exp, Expr::InfixExpr(i1, ex1, ex2)) => cmp_lit(
            infix,
            gen_literal_form_exp(exp, data),
            evaluate_condition(i1, ex1, ex2, data),
        ),
        (e1, _e2) => Err(
            ErrorInfo{
                message: "error in evaluate_condition function".to_owned(),
                interval: interval_from_expr(e1)
            }
        )
    }
}

fn valid_condition(expr: &Expr, data: &mut Data) -> bool {
    
    dbg!(expr);
    match expr {
        Expr::InfixExpr(inf, exp1, exp2) => {
            println!("valid_condition");        
            match evaluate_condition(inf, exp1, exp2, data) {
            
                Ok(SmartLiteral{literal: Literal::BoolLiteral{value: false, ..}, ..}) => false,
                Ok(_) => true,
                Err(_e) => false,
            }
        },
        Expr::LitExpr( SmartLiteral{literal: Literal::BoolLiteral{value}, ..}) => *value,
        Expr::LitExpr( SmartLiteral{literal: Literal::Null{..}, ..}) => false,
        Expr::LitExpr( .. ) => true,
        Expr::BuilderExpr(..) => get_var_from_ident(expr, data).is_ok(), // error
        Expr::IdentExpr(ident, ..) => get_var(ident.to_owned(), data).is_ok(),      // error
        _ => false, // return error
    }
}

fn add_to_message(root: MessageData, action: MessageType) -> MessageData {
    match action {
        MessageType::Msg(msg) => root.add_message(msg),
        MessageType::Empty => root,
    }
}

fn match_builtin(name: &str, args: HashMap<String, Literal>, span: Interval, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match name {
        TYPING => Ok(typing(args, name.to_owned(), span)?),
        WAIT => Ok(wait(args, name.to_owned(), span)?),
        URL => Ok(url(args, name.to_owned(), span)?),
        IMAGE => Ok(img(args, name.to_owned(), span)?),
        ONE_OF => Ok(one_of(args, span)?),
        SHUFFLE => Ok(shuffle(args, span)?),
        QUESTION => Ok(question(args, name.to_owned(), span)?),
        BUTTON => Ok(button(args, name.to_owned(), &span)?),
        FN => Ok(api(args, span, data)?),
        OBJECT => Ok(object(args)?),
        _ => Ok(text(args, name.to_owned(), span)?)
    }
}

fn format_object_attributes(expr: &Expr, data: &mut Data) -> Result<HashMap<String, Literal>, ErrorInfo> {
    let mut obj: HashMap<String, Literal> = HashMap::new();
    let vec = match expr {
        Expr::VecExpr(vec, ..) => vec,
        _e                     => return Err(
            ErrorInfo{
                message: format!("ERROR: Object attributes {:?} bad format", expr),
                interval: interval_from_expr(expr)
            }
        )
    };

    for elem in vec.iter() {
        match elem {
            Expr::ObjectExpr(ObjectType::Assign(var_name, var)) => {
                let value = expr_to_literal(var, data)?.literal;
                obj.insert(var_name.ident.to_owned(), value);
            }
            Expr::ObjectExpr(ObjectType::Normal(name, value)) => {
                let interval = interval_from_expr(elem);                
                let (name, literal) = normal_object_to_literal(&name.ident, value, interval, data)?;

                obj.insert(name, literal);
            }
            _ => {
                let value = expr_to_literal(elem, data)?.literal;
                obj.insert("default".to_owned(), value);
            }
        }
    }

    Ok(obj)
}

fn normal_object_to_literal(name: &str, value: &Expr, interval: Interval , data: &mut Data) -> Result<(String, Literal), ErrorInfo> {
    let obj = format_object_attributes(value, data)?;

    if BUILT_IN.contains(&name) {
        Ok(
            (name.to_owned(), match_builtin(&name, obj, interval.to_owned(), data)?)
        )
    } 
    else {
        Err( 
            ErrorInfo{
                message: format!("ERROR: unknown function {}", name),
                interval: interval.to_owned()
            }
        )
    }
}

fn expr_to_literal(expr: &Expr, data: &mut Data) -> Result<SmartLiteral, ErrorInfo> {
    match expr {
        Expr::ObjectExpr(ObjectType::As(name, var)) => {
            let value = expr_to_literal(var, data)?;
            data.step_vars.insert(name.ident.to_owned(), value.literal.clone());
            Ok(value)
        }
        Expr::ObjectExpr(ObjectType::Normal(name, value)) => {
            let interval = interval_from_expr(expr);
            let (_name, literal) = normal_object_to_literal(&name.ident, value, interval.to_owned(), data)?;
            
            Ok( SmartLiteral{ literal , interval} )
        }
        Expr::ObjectExpr(ObjectType::Assign(var_name, var)) => {
            let value = expr_to_literal(var, data)?.literal;
            Ok(SmartLiteral{
                    literal: Literal::name_object(var_name.ident.to_owned(), &value),
                    interval: interval_from_expr(expr)
                }
            )
        }
        Expr::BuilderExpr(..) => get_var_from_ident(expr, data),
        Expr::ComplexLiteral(vec, ..) => Ok(get_string_from_complexstring(vec, data)),
        Expr::VecExpr(vec, range) => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(expr_to_literal(value, data)?.literal)
            }

            Ok(SmartLiteral{
                    literal: Literal::array(array),
                    interval: range.start.to_owned()
                }
            )
        }
        Expr::IdentExpr(var, ..) => Ok(get_var(var.to_owned(), data)?),
        Expr::LitExpr(literal) => Ok(literal.clone()),
        e => Err(
            ErrorInfo{
                message: format!("ERROR: Expr {:?} can't be converted to Literal", expr),
                interval: interval_from_expr(e)
            }
        )
    }
}

fn match_functions(action: &Expr, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match action {
        Expr::ObjectExpr(ObjectType::As(name, expr)) => {
            let lit = match_functions(expr, data)?;

            data.step_vars.insert(name.ident.to_owned(), lit.clone());
            Ok(lit)
        }
        Expr::ObjectExpr(ObjectType::Normal(..)) => Ok(expr_to_literal(action, data)?.literal),
        Expr::BuilderExpr(..) => Ok(expr_to_literal(action, data)?.literal),
        Expr::ComplexLiteral(vec, ..) => Ok(get_string_from_complexstring(vec, data).literal),
        Expr::InfixExpr(infix, exp1, exp2) => Ok(evaluate_condition(infix, exp1, exp2, data)?.literal),
        Expr::IdentExpr(ident, ..) => match get_var(ident.to_owned(), data) {
            Ok(val) => Ok(val.literal),
            Err(_e) => Ok(Literal::null())
        },
        Expr::LitExpr { .. } => {
            Ok(expr_to_literal(action, data)?.literal)
        },
        Expr::VecExpr(..) => {
            Ok(expr_to_literal(action, data)?.literal)
        },
        e => Err(
            ErrorInfo{
                message: format!("Error must be a valid function {:?}", e),
                interval: interval_from_expr(e)
            }
        )
    }
}

fn match_actions(
    function: &ObjectType,
    mut root: MessageData,
    data: &mut Data,
) -> Result<MessageData, ErrorInfo> {
    match function {
        ObjectType::Say(arg) => {
            Ok(add_to_message(
                root, 
                MessageType::Msg(
                    Message::new(match_functions(arg, data)?)
                )
            ))
        },
        ObjectType::Use(arg) => {
            match_functions(arg, data)?;
            Ok(root)
        }
        ObjectType::Goto(GotoType::Step, step_name) => Ok(root.add_next_step(&step_name.ident)),
        ObjectType::Goto(GotoType::Flow, flow_name) => Ok(root.add_next_flow(&flow_name.ident)),
        ObjectType::Remember(name, variable) => {
            root = root.add_to_memory(name.ident.to_owned(), match_functions(variable, data)?);
            Ok(root)
        }
        ObjectType::Import {
            step_name: name, ..
        } => {
            if let Some(Expr::Block { arg: actions, .. }) = data
                .ast
                .flow_instructions
                .get(&InstructionType::NormalStep(name.ident.to_owned()))
            {
                match interpret_block(&actions, data) {
                    Ok(root2) => Ok(root + root2),
                    Err(err) => Err(
                        ErrorInfo{
                            message: format!("Error in import function {:?}", err),
                            interval: interval_from_reserved_fn(function)
                        }
                    )
                }
            } else {
                Err(
                    ErrorInfo{
                        message: format!("Error step {} not found in flow", name.ident),
                        interval: interval_from_reserved_fn(function)
                    }
                )
            }
        }
        reserved => Err(
            ErrorInfo{
                message: "Error must be a valid action".to_owned(),
                interval: interval_from_reserved_fn(reserved)
            }
        )
    }
}

fn match_ask_response(
    vec: &[Expr],
    mut root: MessageData,
    data: &mut Data,
    opt: &Option<SmartIdent>,
    range: RangeInterval,
) -> Result<MessageData, ErrorInfo> {
    for block in vec.iter() {
        match (block, data.event, data.memory.is_initial_step) {
            (
                Expr::Block {
                    block_type: BlockType::Response,
                    arg: args,
                    ..
                },
                Some(..),
                false
            ) => {
                if let Some(SmartIdent{ident, interval, index}) = opt {
                    if let Some(..) = index {
                        return Err(
                            ErrorInfo{
                                message: "Error: Ask/Response default value is not an Array".to_owned(),
                                interval: range.start
                            }
                        )
                    };
                    root = root.add_to_memory(
                        ident.to_owned(),
                        gen_literal_form_event(data.event, interval.to_owned())?.literal,
                    );
                }
                return Ok(root + interpret_block(args, data)?);
            },
            (
                Expr::Block {
                    block_type: BlockType::Ask,
                    arg: args,
                    ..
                },
                None,
                false
            ) => return Ok(root + interpret_block(args, data)?),
            (
                Expr::Block {
                    block_type: BlockType::Ask,
                    arg: args,
                    ..
                },
                Some(..),
                true
            ) => return Ok(root + interpret_block(args, data)?),
            (..) => continue,
        }
    }
    Err(
        ErrorInfo{
            message: "Error fail to find the correct action block bettween Ask/Response".to_owned(),
            interval: range.start
        }
    )
}

pub fn solve_if_statments(
    statment: &IfStatement,
    mut root: MessageData,
    data: &mut Data,
) -> Result<MessageData, ErrorInfo> {
    match statment {
        IfStatement::IfStmt {
            cond,
            consequence,
            then_branch,
        } => {
            if valid_condition(cond, data) {
                root = root + interpret_block(consequence, data)?;
                return Ok(root);
            }
            if let Some(then) = then_branch {
                return solve_if_statments(then, root, data);
            }
            Ok(root)
        }
        IfStatement::ElseStmt(consequence, ..) => {
            root = root + interpret_block(consequence, data)?;
            Ok(root)
        }
    }
}

pub fn for_loop(
    ident: &SmartIdent, 
    i: &Option<SmartIdent>,
    expr: &Expr,
    block: &[Expr],
    range: &RangeInterval,
    mut root: MessageData,
    data: &mut Data
) -> Result<MessageData, ErrorInfo>  {
    let s_lit = expr_to_literal(expr, data)?;
    let vec = match s_lit.literal {
        Literal::ArrayLiteral{items} => items,
        _ => return Err(
            ErrorInfo{
                message: "Error in for loop, element is not itrerable".to_owned(),
                interval: range.start.to_owned()
            }
        )
    };

    for (value, elem) in vec.iter().enumerate() {
        data.step_vars.insert(ident.ident.to_owned(), elem.clone());
        if let Some(index) = i {
            data.step_vars.insert(index.ident.to_owned(), Literal::int(value as i64));
        };
        root = root + interpret_block(block, data)?;
    }
    data.step_vars.remove(&ident.ident);
    if let Some(index) = i {
        data.step_vars.remove(&index.ident);
    };
    Ok(root)
}

pub fn interpret_block(actions: &[Expr], data: &mut Data) -> Result<MessageData, ErrorInfo> {
    let mut root = MessageData {
        memories: None,
        messages: vec![],
        next_flow: None,
        next_step: None,
    };

    for action in actions {
        if root.next_step.is_some() || root.next_flow.is_some(){
            return Ok(root);
        }

        match action {
            Expr::ObjectExpr(fun) => root = match_actions(fun, root, data)?,
            Expr::IfExpr(ref ifstatement) => root = solve_if_statments(ifstatement, root, data)?,
            Expr::ForExpr(ident, i, expr, block, range) => root = for_loop(ident, i, expr, block, range, root, data)?,
            Expr::Block {
                block_type: BlockType::AskResponse(opt),
                arg: vec,
                range
            } => {
                root = match_ask_response(vec, root, data, opt, range.clone())?;
            }
            e => return Err(
                ErrorInfo{
                    message: "Block must start with a reserved keyword".to_owned(),
                    interval: interval_from_expr(e)
                }
            )
        };
    }
    Ok(root)
}
