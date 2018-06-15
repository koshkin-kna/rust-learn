extern crate learn;



fn main() {
    use learn::learn::util_file;
    match util_file("src/text.txt".to_string()) {
        Ok(_) => println!("Файл успешно обработан"),
        Err(err) => println!("Ошибка: {}", err)
    }
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