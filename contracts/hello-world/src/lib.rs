#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Map};

#[contract]
pub struct AttendanceTracker;

#[contractimpl]
impl AttendanceTracker {

    // Mark attendance for a student
    pub fn mark_attendance(env: Env, student_id: Symbol) {

        let key = symbol_short!("ATT");

        let mut attendance: Map<Symbol, bool> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        // prevent duplicate attendance
        if attendance.contains_key(student_id.clone()) {
            panic!("Attendance already marked");
        }

        attendance.set(student_id, true);

        env.storage().instance().set(&key, &attendance);
    }


    // Check attendance
    pub fn check_attendance(env: Env, student_id: Symbol) -> bool {

        let key = symbol_short!("ATT");

        let attendance: Map<Symbol, bool> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        attendance.get(student_id).unwrap_or(false)
    }
}