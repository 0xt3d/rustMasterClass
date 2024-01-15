//println!("Hello, world!");
    
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
