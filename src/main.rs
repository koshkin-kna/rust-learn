extern crate learn;
extern crate image;

//use std::fs::File;
//use image::GenericImage;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
//use learn::step_2;
//use learn::step_1::util_file;

fn main() {
    let dir = Path::new("./src/papka");
    let _ = visit_dirs(dir, &|t| {
        //println!("{:?}", &t);
        //let ext = Path::new(&t.path()).extension();
        //println!("{:?}", ext);
        let dir_puthbuff = t.path();
        let ext = dir_puthbuff.as_path().extension();
        println!("{:?}", ext);
    });

    fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }
    /*
    let img = image::open("./src/test.jpg").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());
    let ref mut fout = File::create("test.png").unwrap();
    img.write_to(fout, image::PNG).unwrap();
    */
}


// Посчитать гласные – введите строку, и программа подсчитывает количество гласных букв в тексте.
// Для дополнительной сложности у него есть отчет о сумме каждой найденной гласной.

// Узнать атомного времени от сети часы – эта программа позволит получить истинное атомное время с атомными часами в Интернете.
// Есть различные часы по всему миру. Выполните поиск списка из них.

// Групповое переименование файлов и органайзер – данная программа будет брать файлы и
// переименовывает их с определенным именем фильтров, введенных пользователем.
// Например, если пользователь вводит myimage###.jpg будет переименовать все файлы с "минимум"
// из трех чисел, как “myimage001.jpg", " myimage145.jpg "или даже" myimage1987.jpg "
// с 1987 года имеет как минимум три номера.

// На YouTube загрузчик – программа, которая может скачивать видео на ваш жесткий диск
// из youtube.com. Сохраните файлы в различные форматы, включая FLV и AVI.

// программы захвата экрана – сделать программу, которая будет просто захватывать кадр с
// веб-камеры. Для дополнительной сложности см., если вы можете также построить в
// функциональности электронной почты.

// Заставка – сделайте заставку программы, которая будет запускаться когда компьютер
// находится в состоянии простоя. Чтобы сделать простой использовать некоторые стандартные
// фотографии, а затем для дополнительной сложности попробовать 3D-объект, который вращается
// вокруг экрана и отскакивает от сторон.