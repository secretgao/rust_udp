use std::net::{UdpSocket,ToSocketAddrs};
use std::{thread,io};

fn main() {
    println!("udp char app");
    let mut cnt = 0;
    let (mut src,mut tar) = (None,None);
    let mut name = String::new();
    for arg in std::env::args()
    {
        if 1 == cnt {
            src = Some(arg);
        }  else if 2== cnt {
            tar = Some(arg);
        }  else if 3 == cnt {
            name = String::from(arg);
        }
        cnt +=1;
    }
    if let (Some(v1),Some(v2)) = (src,tar)
    {
        func_create(v1,v2,name);
    }
}

fn func_create<A>(str :A,tar:A,name:String)  where A:ToSocketAddrs + std::fmt::Debug{
    println!("[func_create] create a char app {:#?},{:#?},{}",str,tar,name);
    let socket = UdpSocket::bind(str);
    if socket.is_err(){
        return;
    }
    let socket = socket.unwrap();
    if socket.connect(tar).is_err(){
        return;
    }
    let lis_tk = socket.try_clone().unwrap();let n1 = name.clone();
    let rep_tk = socket.try_clone().unwrap();let n2 = name.clone();
    let  handle_lis = thread::spawn(move ||{ func_listen(lis_tk,n1); });
    let  handle_rep = thread::spawn(move ||{ func_replys(rep_tk,n2); });
    handle_lis.join();
    handle_rep.join();
}

fn func_listen(socket:UdpSocket,name:String)
{
    loop {
        let mut buf=[0u8;256];
        let (amt,src) = socket.recv_from(&mut buf).unwrap();
        let buf  = &mut buf[..amt];
        let mut info = String::new();
        for v in buf
        {
            if '\n' == (*v as char) || '\r' == (*v as char)
            {
                continue
            }
            info.push(*v as char)
        }
        println!("[{}]recv info :{}",name,info);
    }
}

fn func_replys(socket:UdpSocket,name:String)
{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let  sinfo = format!("[{}]:{}",name,input);
        if socket.send(sinfo.as_bytes()).is_err(){
            continue;
        }
        println!("[{}]send info :{}",name,input);
    }
}