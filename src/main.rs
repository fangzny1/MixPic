use clap::Parser;
use image::{GenericImage, RgbaImage, open};
use std::{fs::{self}, path::Path};
#[derive(Parser)]
struct Args {
    path: String,
    #[arg(short, long, default_value = "output.png")]
    output: String,
    #[arg(short, long, default_value = "")]
    put_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pic_vec: Vec<std::path::PathBuf> = vec![];
    let args = Args::parse();
    let file_user = args.output;
     let output_file=args.put_path;
    let dir = Path::new(&args.path);
    if output_file == String::from("h")|| fs::read_dir(&output_file).is_err()||output_file == String::from("help")
    {
         let temp =r"cargo run -- I:\spaxie -o output.png -p I:\spaxie".to_string();
        println!("第一个输入要合并文件夹的路径，记得提前把文件按照名称顺序排列即可");
        println!("第二参数输入要合并的图片的文件名");
        println!("第三个输入输出合并的文件的路径");
        println!("例如{}",temp);
       
        return  Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path: std::path::PathBuf = entry.path();
        if path.is_file() {
            println!(
                "检测到文件夹下的图片{} {}",
                path.display(),
                fs::metadata(&path)?.len()
            );
            pic_vec.push(path);
        }
    }
    println!("一共{}个文件开始处理", pic_vec.len());
    pic_vec.sort();
   
    copy_mix_one(pic_vec,file_user,output_file)?;
    Ok(())
}

fn copy_mix_one(
    file_path: Vec<std::path::PathBuf>,
    name:String,
    user_outpath:String
) -> Result<(), Box<dyn std::error::Error>> {
    let mut imgfile: Vec<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> = vec![];
    for p in file_path.iter() {
        imgfile.push(match open(p.clone()).ok() {
            Some(e)=>e.to_rgba8(),
            None=>{
                println!("找到非图片文件，跳过");
                continue},
        });
    }
    let mut w: u32 = 0;
    let mut h: u32 = 0;
    let mut out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
    for p in imgfile.iter() {
        w += p.width();
        h = h.max(p.height());
    }
    out = RgbaImage::new(w, h);
    let mut x = 0u32;

    for img_mix in imgfile.iter() {
        out.copy_from(img_mix, x, 0)?;
        x += img_mix.width();
    }
    if x==0||h==0{
        println!("目录内找不到可用的图片");
    }
 
 if user_outpath.is_empty()
 {
     out.save(&name)?;
 }
 else {
     let path_user =Path::new(&user_outpath).join(&name);
     out.save(path_user)?;
 }
  
    
    println!("合并完成，文件名:{}",name);

    Ok(())
}
