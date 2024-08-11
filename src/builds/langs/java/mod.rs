use std::{
    fs::{self, File},
    io::Write,
};

pub fn create_java_app(name: String) {
    fs::create_dir(format!("./{name}")).unwrap();
    fs::create_dir(format!("./{name}/lib")).unwrap();
    fs::create_dir(format!("./{name}/bin")).unwrap();
    fs::create_dir(format!("./{name}/command")).unwrap();
    fs::create_dir(format!("./{name}/src")).unwrap();
    fs::create_dir(format!("./{name}/src/program")).unwrap();

    let mut main_file = File::create_new(format!("./{name}/src/program/Main.java")).unwrap();
    main_file
        .write_all(
            b"package src.program;

public class Main {
    public static void main(String[] args){
        System.out.println(\"Hello World!\");
    }
}",
        )
        .unwrap();

    let mut run_command_file = File::create_new(format!("./{name}/command/run.sh")).unwrap();
    run_command_file
        .write_all(b"#/usr/bin/sh\n\njavac $(pwd)/**/**/*.java\njava src.program.Main")
        .unwrap();
}
