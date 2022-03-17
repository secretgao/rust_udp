//用链表实现栈
#[derive(Debug)]
struct StackNode<T>{
    data:T,
    next:Option<Box<StackNode<T>>>,
}
#[derive(Debug)]
struct Stack<T>{
    top:Option<Box<StackNode<T>>>,
}

impl <T> Stack<T> {
    fn new()->Stack<T>{
        Stack{ top:None }
    }
    fn push(&mut self,data:T){
        let mut node = StackNode{data:data,next:None};
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }
    fn pop(&mut self) ->Option<T>{
        let data = self.top.take();
        match data {
            Some(mut v)=>{
                self.top = v.next.take();
                Some(v.data)
            },
            None=>None,
        }
    }
}
fn main(){
    let mut s = Stack::new();
    s.push(1);
    s.push(3);
    s.push(5);
    println!("s :{:#?}",s);
    println!("================");
    for _ in 0..4{
        if let Some(data) = s.pop(){
            println!("data :{}",data);
        } else {
            println!("stack empty");
        }
    }
}