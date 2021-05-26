/*
 *   Copyright (c) 2020 Ludwig Bogsveen
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

use std::process::{Command};
use std::fs::*;
use std::io::prelude::*;
use std::collections::HashMap;

mod math;
mod perms;
mod cmds;
mod handler;
mod user;

use handler::Handler;
use perms::{Permission, PermsAccount};
use serenity::Client;
use user::User;

fn load_answers() -> HashMap<String, String> {
    let mut answers = HashMap::new();

    let mut content = Vec::new();
    let mut file = File::open("answers.yaml").unwrap();
    file.read_to_end(&mut content).unwrap();
    let content = &String::from_utf8(content).unwrap();

    //println!("{}",content);

    let yaml = yaml_rust::YamlLoader::load_from_str(content).unwrap();
    let yaml = yaml[0].as_hash().unwrap();
    for item in yaml {
        answers.insert(String::from(item.0.as_str().unwrap()), String::from(item.1.as_str().unwrap()));
    }

    answers
}

const LUDWIGS_SERVER: &str = "755492683417518281";
const KBM_SERVER:     &str = "436444150251126784";
const KRABBANS_SERVER:&str = "220112575801589760";

fn main() {
    let answers = load_answers();

    let mut users = HashMap::new();

    let mut owner = PermsAccount::new();
    owner.add(Permission::Owner);

    let mut admin = PermsAccount::new();
    admin.add(Permission::Admin);

    let mut reddit = PermsAccount::new();
    reddit.add(Permission::Reddit);

    let mut tor = User::new();
    tor.add_perms(String::from("*"), owner);

    let mut jt = User::new();
    jt.add_perms(String::from(KRABBANS_SERVER), admin.clone());

    let mut turban = User::new();
    turban.add_perms(String::from(KRABBANS_SERVER), admin.clone());
    
    let mut fia = User::new();
    fia.add_perms(String::from(KRABBANS_SERVER), admin.clone());

    let mut hadi = User::new();
    hadi.add_perms(String::from(KRABBANS_SERVER), admin.clone());
    
    users.insert("Ludwig#9656".to_string(), tor);
    users.insert("Krabban/MrCR4B#6604".to_string(), jt);
    users.insert("Turban#8907".to_string(), turban);
    users.insert("Fiaa#4523".to_string(), fia);
    users.insert("hadi190#6025".to_string(), hadi);



    let handler = Handler {
        answers,
        users,
    };

    let token = "NzU1NDM5ODA3MDM1MDgwODM1.X2DUJQ.RZhMd6hvnyqN5NE5TtGxhkoeec4";
    let mut client = Client::new(&token, handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}