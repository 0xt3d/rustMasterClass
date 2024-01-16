//println!("Hello, world!");
#[derive(Debug)]  
use std::io;
struct task{
    id: u32,
    name: String,
    completed: bool
}
fn main() {
 let mut tlist : Vec<task> = Vec::new();

 let task_1: task = task{
    id: 1,
    name: String::from("Buy Coffee"),
    completed: false
 };
 let task_2: task = task{
    id: 2,
    name: String::from("Buy Gums"),
    completed: false
 };
 tlist.push(task_1);
 tlist.push(task_2);

 println!("{:?}", tlist);
}

fn update_task(tlist: &mut task){
    tlist.completed = true;
}

fn display_task(tlist: &Vec<task>){
    for tlist in tlist{
println!("{} - {} ({})", tlist.id, tlist.name, tlist.completed);
    }
}
