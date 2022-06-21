/*
 *   Copyright (c) 2022 Nazmul Idris
 *   All rights reserved.

 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at

 *   http://www.apache.org/licenses/LICENSE-2.0

 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
*/

use async_trait::async_trait;
use r3bl_rs_utils::redux::AsyncReducer;
use std::fmt::{Display, Formatter};

/// Action.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum AppAction {
  Startup,
  AddPop(i32),
  SubPop(i32),
  Clear,
  Noop,
}

impl Default for AppAction {
  fn default() -> Self {
    AppAction::Noop
  }
}

impl Display for AppAction {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

/// State.
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct AppState {
  pub stack: Vec<i32>,
}

impl Default for AppState {
  fn default() -> Self {
    Self { stack: vec![0] }
  }
}

impl Display for AppState {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "State {{ stack: {:?} }}", self.stack)
  }
}

/// Reducer.
#[derive(Default)]
pub struct AppReducer;

#[async_trait]
impl AsyncReducer<AppState, AppAction> for AppReducer {
  async fn run(&self, action: &AppAction, state: &AppState) -> AppState {
    let mut stack = state.stack.clone();

    match action {
      AppAction::AddPop(arg) => {
        if stack.is_empty() {
          stack.push(*arg)
        } else {
          let top = stack.pop().unwrap();
          let sum = top + arg;
          stack.push(sum);
        }
      }

      AppAction::SubPop(arg) => {
        if stack.is_empty() {
          stack.push(*arg)
        } else {
          let top = stack.pop().unwrap();
          let sum = top - arg;
          stack.push(sum);
        }
      }

      AppAction::Clear => stack = vec![],

      _ => {}
    }

    AppState { stack }
  }
}
