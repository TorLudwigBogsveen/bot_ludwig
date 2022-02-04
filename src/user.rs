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

use std::{collections::{HashMap}, fs::File, io::Read, io::Write};



use crate::perms::{PermsAccount, Permission};


 #[derive(Clone, Debug)]
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

    pub fn _server_perms(&self, server_id: &str) -> Option<&PermsAccount> {
        self.perm_accs.get(server_id)
    }

    pub fn server_perms_mut(&mut self, server_id: &str) -> Option<&mut PermsAccount> {
        self.perm_accs.get_mut(server_id)
    }
}

pub fn load_users() -> HashMap<String, User> {
    let mut users = HashMap::<String, User>::new();

    let mut content = Vec::new();
    let mut file = File::open("perms.yaml").unwrap();
    file.read_to_end(&mut content).unwrap();
    let content = &String::from_utf8(content).unwrap();

    //println!("{}",content);

    let yaml = yaml_rust::YamlLoader::load_from_str(content).unwrap();
    let yaml = yaml[0].as_hash().unwrap();
    for item in yaml {
        let server = item.0.as_str().unwrap();
        for item in item.1.as_hash().unwrap() {
            let name = item.0.as_str().unwrap();
            let mut user = if users.contains_key("") {
                users.get(name).unwrap().clone()
            } else {
                User::new()
            };

            let perms = if let Some(user) = user.server_perms_mut(server) {
                user
            } else {
                user.add_perms(String::from(server), PermsAccount::new());
                user.server_perms_mut(server).unwrap()
            };

            for item in item.1.as_vec().unwrap() {
                let permission = Permission::from(item.as_str().unwrap());
                perms.add(permission);
               
            }
            users.insert(String::from(name), user);
        }
    }

    users
}

pub fn save_users(users: &HashMap<String, User>) {
    let mut servers = yaml_rust::yaml::Hash::new();

    for (name, user) in users {
        let name = yaml_rust::Yaml::String(name.clone());
        for (server_name, perms) in &user.perm_accs {
            let yaml = yaml_rust::Yaml::Array(perms.perms.iter().map(|e| yaml_rust::Yaml::String(String::from(<&str>::from(*e)))).collect());
            let server_name = yaml_rust::Yaml::String(String::from(server_name));
            if let Some(server) = servers.get_mut(&server_name) {
                let mut server = server.as_hash().unwrap().clone();
                server.insert(name.clone(), yaml);
                servers.insert(server_name, yaml_rust::Yaml::Hash(server));
            } else {
                let mut server = yaml_rust::yaml::Hash::new();
                server.insert(name.clone(), yaml);
                servers.insert(server_name, yaml_rust::Yaml::Hash(server));
            };
        }
    }

    let yaml = yaml_rust::Yaml::Hash(servers);
    let mut output = String::new();
    let mut emitter = yaml_rust::YamlEmitter::new(&mut output);
    emitter.dump(&yaml).unwrap();

    let mut file = File::create("perms.yaml").unwrap();
    write!(file, "{}", output).unwrap();
}