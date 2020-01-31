#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/nbc_bindings.rs"));

use core::mem::MaybeUninit;

pub fn init_nbc_solver() -> *mut solver {
    let s = unsafe { solver_new() };
    unsafe { solver_set_start(s) };
    s
}

pub fn run_nbc(s : *mut solver) {
    // set native callback flag
    unsafe { (*s).native_pass_flag = 1};
    
    let simplify_ret = unsafe { solver_simplify(s) };
    if simplify_ret == -1{
        println!("Trivial UNSAT");
    }
    else{
        unsafe { 
            let assumptions_begin = veci_begin(&mut (*s).assumptions);
            let assumptions_last = veci_last(&mut (*s).assumptions);
            solver_solve(s, assumptions_begin, assumptions_last);
            solver_clear_assumptions(s);
        }
    }
}

pub fn add_clause_to_nbc_solver(s : *mut solver, given : Vec<i32>) {
    unsafe{
        let mut lits : veci = MaybeUninit::uninit().assume_init();
        // init literals
        veci_new(&mut lits);
        veci_resize(&mut lits, 0);
        for i in given{
            let lit : i32;
            let var = i.abs()-1;
            if i > 0 {
                lit = var+var;
            }
            else {
                lit = (var+var)^1;
            }
            veci_push(&mut lits, lit);
        }
        let begin = veci_begin(&mut lits);
        let last = veci_last(&mut lits);
        solver_addclause(s, begin, last);
        veci_delete(&mut lits);
    }
}

pub fn set_nb_of_solutions(s: *mut solver, k: u64){
    unsafe {
         (*s).is_k_sols = 1;
        (*s).k_sols = k;
    }
}

pub fn get_solver_time_elapsed(s: *mut solver) -> f64 {
    let time : f64 = unsafe { solver_get_time_elapsed(s) };
    time
}

pub fn get_solver_stats(s: *mut solver) -> Vec<f64> {
    // we have 8 fields
    let mut v : Vec<f64> = Vec::with_capacity(8);
    unsafe{ 
        let nb = (*s).stats.tot_solutions as f64;
        let time = solver_get_time_elapsed(s);
        let conflicts = (*s).stats.conflicts as f64;
        let decisions = (*s).stats.decisions as f64;
        let propagations = (*s).stats.propagations as f64;
        let inspects =(*s).stats.inspects as f64;
        let conflict_literals = (*s).stats.tot_literals as f64;
        let last_random_seed = (*s).random_seed as f64;
        v.push(nb);
        v.push(time);
        v.push(conflicts);
        v.push(decisions);
        v.push(propagations);
        v.push(inspects);
        v.push(conflict_literals);
        v.push(last_random_seed);
    }
    v
}

pub fn add_assumptions_to_nbc_solver(s : *mut solver, assumptions : Vec<i32>){
    for i in assumptions{
        let lit : i32;
        let var = i.abs()-1;
        if i > 0 {
            lit = var+var;
        }
        else {
            lit = (var+var)^1;
        }
        unsafe{ veci_push(&mut (*s).assumptions, lit ) };
    }
}

pub fn set_nbc_solver_rnd_seed(s: *mut solver, seed: f64){
    unsafe { (*s).random_seed = seed }; 
}

pub fn get_nbc_nb_learnt_clauses(s: *mut solver) -> u64 {
    let l = unsafe{ (*s).stats.learnts };
    l
}
