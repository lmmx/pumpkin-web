use pumpkin_solver::results::SatisfactionResult;
use pumpkin_solver::results::ProblemSolution;
use pumpkin_solver::termination::Indefinite;
use pumpkin_solver::{constraints, Solver};
use std::ffi::{c_int, CStr, CString};
use std::os::raw::{c_char};

// Empty main function
fn main() {
    println!("Pumpkin solver WASM module initialized");
}

// Exported functions
#[no_mangle]
pub extern "C" fn solve_constraint(min_x: c_int, max_x: c_int, min_y: c_int, max_y: c_int) -> *mut c_char {
    let result = solve_constraint_internal(min_x, max_x, min_y, max_y);
    
    let json_result = match result {
        Ok((x, y)) => format!("{{\"success\": true, \"x\": {}, \"y\": {}}}", x, y),
        Err(msg) => format!("{{\"success\": false, \"message\": \"{}\"}}", msg),
    };
    
    // Convert the result to a C string
    let c_str = CString::new(json_result).unwrap();
    // We must return a pointer that will be freed by JavaScript
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    
    // Convert the pointer back to a CString and drop it
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn debug_message(message: *const c_char) {
    let c_str = unsafe {
        if message.is_null() {
            return;
        }
        CStr::from_ptr(message)
    };
    
    if let Ok(msg_str) = c_str.to_str() {
        println!("{}", msg_str);
    }
}

fn solve_constraint_internal(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Result<(i32, i32), String> {
    // Create a solver with default settings
    let mut solver = Solver::default();

    // Create two integer variables x and y with the specified domains
    let x = solver.new_bounded_integer(min_x, max_x);
    let y = solver.new_bounded_integer(min_y, max_y);

    // Enforce the constraint: x + y = 12
    let constraint_result = solver
        .add_constraint(constraints::equals(vec![x, y], 12))
        .post();
    
    // Check if the constraint was successfully posted
    if constraint_result.is_err() {
        return Err("Failed to post constraint to solver".to_string());
    }

    // Configure an indefinite termination condition and a default branching strategy
    let mut termination = Indefinite;
    let mut brancher = solver.default_brancher();

    // Attempt to find a satisfiable solution
    match solver.satisfy(&mut brancher, &mut termination) {
        SatisfactionResult::Satisfiable(solution) => {
            let x_val = solution.get_integer_value(x);
            let y_val = solution.get_integer_value(y);
            
            Ok((x_val, y_val))
        }
        SatisfactionResult::Unsatisfiable => {
            Err("No solution exists for the given constraints".to_string())
        }
        SatisfactionResult::Unknown => {
            Err("The solver could not determine satisfiability within the termination condition".to_string())
        }
    }
}
