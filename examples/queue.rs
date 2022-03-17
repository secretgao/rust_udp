#[derive(Debug)]
struct Queue<T>{
    qdata:Vec<T>,
    capacity:usize,
}

impl <T> Queue<T> {
    //初始化
    fn new(size:usize) -> Self{
        Queue{
            qdata:Vec::with_capacity(size),
            capacity:size,
        }
    }
    //入队
    fn enqueue(&mut self,value:T)->Result<(),String>{
        if self.qdata.len() == self.capacity{
            return Err("No space in queue".to_string());
        }
        self.qdata.push(value);
        Ok(())
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

    let mut q = Queue::new(2);
    q.enqueue(1);
    q.enqueue(2);
    if let Err(error) =q.enqueue(3){
        println!("queue error :{}",error);
    }

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