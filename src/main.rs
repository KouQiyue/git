use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Contact { name: String, phone: String, email: String }
type Contacts = HashMap<u32, Contact>;
const FILE: &str = "contacts.json";

fn main() {
    let mut db: Contacts = fs::read_to_string(FILE).ok().and_then(|d| serde_json::from_str(&d).ok()).unwrap_or_default();
    loop {
        println!("\n1.列表 2.添加 3.删除 4.更新 5.搜索 6.退出");
        print!("> "); io::stdout().flush().unwrap();
        let mut s = String::new(); io::stdin().read_line(&mut s).unwrap();
        match s.trim() {
            "1" => list(&db),
            "2" => add(&mut db),
            "3" => del(&mut db),
            "4" => upd(&mut db),
            "5" => search(&db),
            "6" => { fs::write(FILE, serde_json::to_string_pretty(&db).unwrap()).unwrap(); return; }
            _ => continue,
        }
    }
}

fn inp(p: &str) -> String { print!("{}", p); io::stdout().flush().unwrap(); let mut s=String::new(); io::stdin().read_line(&mut s).unwrap(); s.trim().to_string() }

fn list(db: &Contacts) {
    if db.is_empty() { println!("空"); return; }
    let mut v: Vec<_> = db.iter().collect(); v.sort_by_key(|k| k.0);
    for (id,c) in v { println!("{}: {} {} {}", id, c.name, c.phone, c.email); }
}

fn add(db: &mut Contacts) {
    let id = db.keys().max().unwrap_or(&0)+1;
    db.insert(id, Contact { name: inp("姓名:"), phone: inp("电话:"), email: inp("邮箱:") });
}

fn del(db: &mut Contacts) {
    list(db);
    let id: u32 = inp("ID:").parse().unwrap_or(0);
    db.remove(&id);
}

fn upd(db: &mut Contacts) {
    list(db);
    let id: u32 = inp("ID:").parse().unwrap_or(0);
    if let Some(c) = db.get_mut(&id) {
        let n = inp(&format!("姓名({}):",c.name)); if !n.is_empty() { c.name=n; }
        let p = inp(&format!("电话({}):",c.phone)); if !p.is_empty() { c.phone=p; }
        let e = inp(&format!("邮箱({}):",c.email)); if !e.is_empty() { c.email=e; }
    }
}

fn search(db: &Contacts) {
    let kw = inp("关键词:").to_lowercase();
    for (_,c) in db {
        if c.name.to_lowercase().contains(&kw)||c.phone.contains(&kw)||c.email.to_lowercase().contains(&kw)
           { println!("{} {} {}", c.name, c.phone, c.email); }
    }
}
