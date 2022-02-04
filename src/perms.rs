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

use std::{sync::{Arc, Mutex}, collections::HashMap};

use serenity::{prelude::TypeMapKey};

use crate::user::User;

pub struct PermsContainer;

impl TypeMapKey for PermsContainer {
    type Value = Arc<Mutex<HashMap<String, User>>>;
}

 #[derive(Clone, Copy, PartialEq, Debug)]
pub enum Permission {
    Owner,
    Admin,
    Reddit,
    Math,
    Conversation,
    Help,
}

impl From<&str> for Permission {
    fn from(s: &str) -> Self {
        match s { 
            "Owner" => Permission::Owner,
            "Admin" => Permission::Admin,
            "Reddit"=> Permission::Reddit,
            "Conversation"=> Permission::Conversation,
            "Math"=> Permission::Math,
            "Help"=> Permission::Help,
            _=>panic!("INVALID PERMISSION!!!"),
        }
    }
}

impl From<Permission> for &str {
    fn from(permission: Permission) -> Self {
        match permission { 
            Permission::Owner => "Owner",
            Permission::Admin => "Admin",
            Permission::Reddit => "Reddit",
            Permission::Conversation => "Conversation",
            Permission::Math => "Math",
            Permission::Help => "Help",
        }
    }
}

#[derive(Clone, Debug)]
pub struct PermsAccount {
    pub perms: Vec<Permission>,
}

impl PermsAccount {
    pub fn new() -> PermsAccount{
        PermsAccount {
            perms: Vec::new(),
        }
    }

    pub fn add(&mut self, perm: Permission) -> &mut Self {
        if !self.has(perm) {
            self.perms.push(perm);
        }
        self
    }

    pub fn has(&self, perm: Permission) -> bool {
        for p in &self.perms {
            if perm == *p {
                return true;
            }
        }
        false
    }
}
