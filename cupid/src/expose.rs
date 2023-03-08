use crate::{value::Value, vm::Vm};

pub fn cupid_clock(vm: &Vm, _args: &[Value]) -> Value {
    let time = vm.start_time.elapsed().unwrap().as_secs_f64();
    Value::Float(time)
}

pub fn cupid_panic(_vm: &Vm, args: &[Value]) -> Value {
    let mut terms: Vec<String> = vec![];

    for &arg in args.iter().filter(|a| **a != Value::Nil) {
        let term = format!("{}", arg);
        terms.push(term);
    }

    panic!("panic: {}", terms.join(", "))
}

pub fn cupid_push(_vm: &Vm, args: &[Value]) -> Value {
    match args[0] {
        Value::Array(mut array) => {
            array.items.push(args[1]);
            Value::Nil
        }
        _ => panic!("expected array"),
    }
}

pub fn cupid_pop(_vm: &Vm, args: &[Value]) -> Value {
    match args[0] {
        Value::Array(mut array) => array.items.pop().unwrap_or(Value::Nil),
        _ => panic!("expected array"),
    }
}

pub fn cupid_len(_vm: &Vm, args: &[Value]) -> Value {
    match args[0] {
        Value::Array(array) => Value::Int(array.items.len() as i32),
        _ => panic!("expected array"),
    }
}

pub fn cupid_get(_vm: &Vm, args: &[Value]) -> Value {
    match (args[0], args[1]) {
        (Value::Array(array), Value::Int(i)) => array.items[i as usize],
        _ => panic!("expected array"),
    }
}
