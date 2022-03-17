#[derive(Debug)]
struct Queue<T>{
    qdata:Vec<T>,
}

impl <T> Queue<T> {
    //初始化
    fn new() -> Self{
        Queue{qdata:Vec::new()}
    }
    //入队
    fn enqueue(&mut self,value:T){
        self.qdata.push(value);
    }
    //出队
    fn dequeue(&mut self,)->Option<T>{
        let size = self.qdata.len();
        if size > 0{
            let v = self.qdata.remove(0);
            Some(v)
        } else {
            None
        }
    }
    //求长度
    fn size(&self) ->usize{
        self.qdata.len()
    }
}

fn main(){

    let mut q = Queue::new();
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);

    let len = q.size();
    println!("queue len:{} queue{:?}",len,q);

    for _ in 0..len+1{
        if let Some(data) = q.dequeue(){
            println!("data :{}",data);
        } else {
            println!("data empty");
        }
    }
}