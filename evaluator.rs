//! This file is where you will implement your code.

use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
};

use crate::{Fsm, VendingAction, DENOMINATIONS, MAX_CENTS};
//use VendingAction::*; // need this to use insert and vend

impl Fsm {
  // Take in a HashMap that contains the price for each item in the vending 
  // machine. Create and return a finite state machine that represents all 
  // possible ways a customer could interact with the vending machine.
  pub fn create(price_map: &HashMap<Arc<str>, u32>) -> Fsm {
      let mut transitions = HashMap::new();

      // Add transitions for inserting coins
      for &denomination in &DENOMINATIONS {
        for current_state in 0..=MAX_CENTS {
          let next_state = current_state + denomination;
          if next_state <= MAX_CENTS {
              transitions.insert((current_state, VendingAction::Insert(denomination)), next_state);
          }
        }
      }

      // Add transitions for vending items
      for (item, price) in price_map {
        for current_state in *price..=MAX_CENTS {
            let next_state = current_state - price;
            transitions.insert((current_state, VendingAction::Vend(item.clone())), next_state);
        }
      }

      let mut fsm = Fsm {
          start: 0,
          transitions,
      };
      fsm
  }
   
  // Validate the given list of actions on the FSM. A customer can only
  // insert one coin (nickel/dime/quarter) or vend one item at a time.
  // Note: the fsm passed in may not be one you have created above.
  pub fn eval(&self, actions: &[VendingAction]) -> bool {
      let mut current_state = self.start;

      for action in actions {
          if !self.is_valid_transition(current_state, action) {
              return false;
          }
          current_state = *self.transitions.get(&(current_state, action.clone())).unwrap();
      }

      current_state >= 0 && current_state <= 500
  }

  fn is_valid_transition(&self, current_state: u32, action: &VendingAction) -> bool {
      self.transitions.contains_key(&(current_state, action.clone()))
  }
  // end of impl Fsm {}
}


pub fn evaluate(fsm: &Fsm, actions: &[VendingAction]) -> Option<u32> {
  let mut current_state = fsm.start;

  for action in actions {
      if !fsm.is_valid_transition(current_state, action) {
          return None;
      }
      current_state = *fsm.transitions.get(&(current_state, action.clone())).unwrap();
  }

  Some(current_state)
}

pub fn make_change(amount: u32) -> Vec<u32> {
  let mut change = Vec::new();
  let mut remaining = amount;

  for &denomination in DENOMINATIONS.iter().rev() {
      let count = remaining / denomination;
      change.extend(std::iter::repeat(denomination).take(count as usize));
      remaining -= count * denomination;
  }

  change
}

pub fn total_value(coins: &[u32]) -> u32 {
  coins.iter().sum()
}

pub fn run_actions(fsm: &Fsm, start_state: u32, actions: &[VendingAction]) -> Option<u32> {
  let mut current_state = start_state;

  for action in actions {
      if !fsm.is_valid_transition(current_state, action) {
          return None;
      }
      current_state = *fsm.transitions.get(&(current_state, action.clone())).unwrap();
  }

  Some(current_state)
}