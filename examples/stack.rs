//栈 后进先出
#[derive(Debug)]
struct Stack<T>{
    data:Vec<T>,
    top:usize,
    capacity:usize,
}

impl <T>Stack<T> {

   // fn new()->Self{
   fn new(size:usize)->Self{
        Stack{
            data:Vec::new(),
            top:0,
            capacity:size,
        }
    }
    //入栈
  // fn push(&mut self, value:T){
   fn push(&mut self, value:T) ->Result<(),String>{
        if self.top >= self.capacity{
            return Err("stack full".to_string());
        }
        self.data.push(value);
        self.top +=1;
        Ok(())
    }
    //出栈
    fn pop(&mut  self )->Option<T>{
        if self.top == 0 {
           None
        } else{
            self.top -=1;
            self.data.pop()
        }
    }
    fn top(&self)->usize{
        self.top()
    }
}
fn main(){
    let mut s = Stack::new(3);
    s.push(1);
    s.push(2);
    s.push(3);
    if let Err(e) = s.push(4){
        println!("err {}",e);
    }
    println!(" ++++++++push stack ++++++");
    println!("s:{:#?}",s);
    for _ in 0..4{
        let a = s.pop();
        match a{
            Some(value) =>println!("pop value :{}",value),
            None=>println!("stack empty"),
        }
    }
}