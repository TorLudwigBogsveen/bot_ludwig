use std::collections::{HashMap, HashSet};

use serenity::model::id::GuildId;

use crate::perms::{self, PermsAccount};

/*
 *   Copyright (c) 2021 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

pub struct User {
    perm_accs: HashMap<String, PermsAccount>,
}

impl User {
    pub fn new() -> User {
        User {
            perm_accs: HashMap::new(),
        }
    }

    pub fn add_perms(&mut self, server_id: String, perms_acc: PermsAccount) {
        self.perm_accs.insert(server_id, perms_acc);
    }

    pub fn server_perms(&self, server_id: &str) -> Option<&PermsAccount> {
        self.perm_accs.get(server_id)
    }
}