use std::fs::create_dir_all;

#[test]
fn easy_good() {
    assert!(!dir_diff::is_different("tests/easy/good/dir1", "tests/easy/good/dir2").unwrap());
}

#[test]
fn easy_bad() {
    assert!(dir_diff::is_different("tests/easy/bad/dir1", "tests/easy/bad/dir2").unwrap());
}

#[test]
fn binary_good() {
    assert!(!dir_diff::is_different("tests/binary/good/dir1", "tests/binary/good/dir2").unwrap());
}

#[test]
fn binary_bad() {
    assert!(dir_diff::is_different("tests/binary/bad/dir1", "tests/binary/bad/dir2").unwrap());
}

#[test]
fn fileanddir() {
    assert!(dir_diff::is_different("tests/fileanddir/dir1", "tests/fileanddir/dir2").unwrap());
}

#[test]
fn oneempty() {
    assert!(dir_diff::is_different("tests/oneempty/dir1", "tests/oneempty/dir2").unwrap());
}

#[test]
#[should_panic]
fn firstmissing() {
    assert!(dir_diff::is_different("does_not_exist", "tests/easy/good/dir1").unwrap());
}

#[test]
#[should_panic]
fn secondmissing() {
    assert!(dir_diff::is_different("tests/easy/good/dir1", "does_not_exist").unwrap());
}

#[test]
#[should_panic]
fn bothmissing() {
    assert!(dir_diff::is_different("does_not_exist", "also_does_not_exist").unwrap());
}

#[test]
fn reflexive() {
    assert!(dir_diff::is_different("tests/reflexive/dir1", "tests/reflexive/dir2").unwrap());
}

#[test]
fn dirs_differ() {
    assert!(dir_diff::is_different("tests/dirs_differ/dir1", "tests/dirs_differ/dir2").unwrap());
}

#[test]
fn filedepth() {
    create_dir_all("tests/filedepth/asc/dir2/a").unwrap();
    create_dir_all("tests/filedepth/desc/dir1/b").unwrap();

    assert!(
        dir_diff::is_different("tests/filedepth/asc/dir1", "tests/filedepth/asc/dir2").unwrap()
    );
    assert!(
        dir_diff::is_different("tests/filedepth/desc/dir1", "tests/filedepth/desc/dir2").unwrap()
    );
}

#[test]
fn line_endings() {
    use std::fs::{create_dir_all, write};
    let dir1 = "tests/line_endings/dir1";
    let dir2 = "tests/line_endings/dir2";
    create_dir_all(dir1).unwrap();
    create_dir_all(dir2).unwrap();
    write(format!("{dir1}/file.txt"), "{\n  \"directory\": \"examples/filters\",\n  \"answers\": {\n    \"project_name\": \"project name is filters\"\n  }\n}").unwrap();
    write(format!("{dir2}/file.txt"), "{\r\n  \"directory\": \"examples/filters\",\r\n  \"answers\": {\r\n    \"project_name\": \"project name is filters\"\r\n  }\r\n}").unwrap();
    assert!(dir_diff::is_different(dir1, dir2).unwrap());
}
