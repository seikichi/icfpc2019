use lib::task::Task;

fn main() {
    let s = "(0,0),(10,0),(10,10),(0,10)#(0,0)#(4,2),(6,2),(6,7),(4,7);(5,8),(6,8),(6,9),(5,9)#B(0,1);B(1,1);F(0,2);F(1,2);L(0,3);X(0,9)";
    let task = Task::from(s);
    println!("visualize {:?}", task);
}
