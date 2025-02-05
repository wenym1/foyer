//  Copyright 2023 MrCroxx
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use bitmaps::Bitmap;
use foyer_common::code::{Key, Value};

use std::{fmt::Debug, sync::Arc};

use crate::metrics::Metrics;

pub type Judges = Bitmap<64>;

#[allow(unused_variables)]
pub trait AdmissionPolicy: Send + Sync + 'static + Debug {
    type Key: Key;
    type Value: Value;

    fn judge(&self, key: &Self::Key, weight: usize, metrics: &Arc<Metrics>) -> bool;

    fn on_insert(&self, key: &Self::Key, weight: usize, metrics: &Arc<Metrics>, judge: bool);

    fn on_drop(&self, key: &Self::Key, weight: usize, metrics: &Arc<Metrics>, judge: bool);
}

pub mod rated_random;
