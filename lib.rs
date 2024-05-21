use std::{collections::HashMap, sync::Arc};

pub mod evaluator;

/// The list of coin denominations accepted by the machine, in cents.
const DENOMINATIONS: [u32; 3] = [5, 10, 25];

/// The maximum amount of money that the machine can hold (in cents).
const MAX_CENTS: u32 = 500;

/// An action that can be taken with respect to the vending machine:
/// insert money, or dispense an item.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VendingAction {
    /// Insert the specified number of cents into the machine.
    Insert(u32),

    /// Attempt to dispense an item from the machine.
    Vend(Arc<str>),
    /// (An [`Arc<str>`] is a reference-counted pointer
    /// to a string on the heap. You can mostly treat it just like
    /// a [`String`], except that it's immutable.
    /// Call the `.clone()` method to make a copy of it.)
}

/// A finite state machine representing the vending machine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fsm {
    /// The ID of the initial state.
    /// (ID values can be whatever you want,
    /// as long as each ID uniquely identifies a state.)
    pub start: u32,

    /// A map representing a list of possible transitions:
    /// (initial state, transition symbol) → final state.
    pub transitions: HashMap<(u32, VendingAction), u32>,
    // cf. Here we don't expect you to store the list of states 
    // or transition symbols. Nor is there any final states. 
    // (every state can be final) 
}
