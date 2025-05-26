use translate_dir_lib::translator::{translate_file, translate_file_to_file};
use translate_dir_lib::{self, Language};
fn main() {
    let mut res = translate_dir_lib::project::load(std::path::PathBuf::from(".")).unwrap();
    //translate_dir_lib::project::init("name", std::env::current_dir().unwrap());
    //let set_res = res.set_source_dir("test_dir", Language::French);
    //let add_res = res.add_lang(Language::English);
    //let rem_res = res.remove_lang(Language::French);
    //let sync_res = res.sync_files();
    //println!("{:?}", set_res);
    //println!("{:?}", add_res);
    //println!("{:?}", rem_res);
    //println!("{:?}", sync_res);
    //let make_res =
    //    res.make_untranslatable_file(std::path::PathBuf::from("test_dir/more_dirs/file.html"));
    //let make_res = res.make_translatable_file(std::path::PathBuf::from(
    //    "/Users/dobbikov/Desktop/stage/prototype/prototype/test_dir/lec9.tex",
    //));
    //println!("{:?}", make_res);
    println!("Hello, world!");
    //let res = translate_file(
    //    "/Users/dobbikov/Desktop/stage/comparing_results/into_python/french/file.md",
    //);
    //let res = translate_file_to_file(
    //    "/Users/dobbikov/Desktop/stage/comparing_results/into_python/french/file.md",
    //    "/Users/dobbikov/Desktop/stage/comparing_results/into_python/french/file_eng.md",
    //);
    //
    //res.make_untranslatable_file(std::path::PathBuf::from("test_dir/test_file.md"));
    //let files = res.get_translatable_files().unwrap();

    //println!("{:?}", res.set_source_dir("test_dir", Language::English));
    //println!("{:?}", res.add_lang(Language::French));

    // translating file
    let res_tr = res.translate_file(
        std::path::PathBuf::from("test_dir/lec9.tex"),
        Language::Ukrainian,
    );
    //let res_tr = res.translate_all(Language::French);
    println!("{:?}", res_tr);
    //println!("{:?}", files);
    println!("Done");
    //println!("{}", res);
}
