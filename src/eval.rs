use crate::ast::{Node, Op};

pub fn eval(node: &Node) -> Result<i64, String> {
    match node {
        Node::Number(v) => Ok(*v),
        Node::Call(op, args) => match op {
            Op::Add => {
                let mut sum = 0i64;
                for a in args { sum += eval(a)?; }
                Ok(sum)
            }
            Op::Sub => {
                if args.is_empty() { return Err("(-) needs at least 1 argument".into()); }
                let mut it = args.iter();
                let first = eval(it.next().unwrap())?;
                it.try_fold(first, |acc, a| {
                    let v = eval(a)?;
                    Ok::<i64, String>(acc - v)
                })
            }
            Op::Mul => {
                let mut prod = 1i64;
                for a in args { prod *= eval(a)?; }
                Ok(prod)
            }
            Op::Div => {
                if args.is_empty() { return Err("(/) needs at least 1 argument".into()); }
                let mut it = args.iter();
                let first = eval(it.next().unwrap())?;
                it.try_fold(first, |acc, a| {
                    let v = eval(a)?;
                    if v == 0 { return Err("division by zero".into()); }
                    Ok(acc / v)
                })
            }
        }
    }
}
