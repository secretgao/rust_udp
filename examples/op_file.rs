use std::fs;
use std::io;
use std::path::Path;

fn visit_dirs(dir:&Path) -> io::Result<()>{
    if dir.is_dir(){
        for entry in fs::read_dir(dir) ? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                visit_dirs(&path);
            } else {
                println!("path:{:#?}",&path);
                let c = fs::read_to_string(path).unwrap();
                println!("file content:{}",c);
            }
        }
    }
    Ok(())
}
fn main(){
    /*
    let context = fs::read("./test/hello.txt").unwrap();
    println!("{:#?}",context);
    let context = fs::read_to_string("./test/hello.txt").unwrap();
    println!("{:#?}",context);

     */
    visit_dirs(Path::new("./test"));
}