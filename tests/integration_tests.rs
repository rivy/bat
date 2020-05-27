use assert_cmd::Command;
use std::path::Path;
use std::str::from_utf8;

const EXAMPLES_DIR: &str = "tests/examples";

fn bat_with_config() -> Command {
    let mut cmd = Command::cargo_bin("bat").unwrap();
    cmd.current_dir("tests/examples");
    cmd.env_remove("PAGER");
    cmd.env_remove("BAT_PAGER");
    cmd.env_remove("BAT_CONFIG_PATH");
    cmd.env_remove("BAT_STYLE");
    cmd.env_remove("BAT_THEME");
    cmd.env_remove("BAT_TABS");
    cmd
}

fn bat() -> Command {
    let mut cmd = bat_with_config();
    cmd.arg("--no-config");
    cmd
}

#[test]
fn basic() {
    let assert = bat()
        .arg("test.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    println!("stdout={:#?}", stdout);
    println!("stderr={:#?}", stderr);
    assert
        .success()
        .stdout("hello world\n")
        .stderr("");
}

#[test]
fn stdin() {
    let assert = bat()
        .write_stdin("foo\nbar\n")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("foo\nbar\n");
}

#[test]
fn concatenate() {
    let assert = bat()
        .arg("test.txt")
        .arg("test.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("hello world\nhello world\n");
}

#[test]
fn concatenate_stdin() {
    let assert = bat()
        .arg("test.txt")
        .arg("-")
        .arg("test.txt")
        .write_stdin("stdin\n")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("hello world\nstdin\nhello world\n");
}

#[test]
fn concatenate_empty_first() {
    let assert = bat()
        .arg("empty.txt")
        .arg("test.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("hello world\n");
}

#[test]
fn concatenate_empty_last() {
    let assert = bat()
        .arg("test.txt")
        .arg("empty.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("hello world\n");
}

#[test]
fn concatenate_empty_both() {
    let assert = bat()
        .arg("empty.txt")
        .arg("empty.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("");
}

#[test]
fn concatenate_empty_between() {
    let assert = bat()
        .arg("test.txt")
        .arg("empty.txt")
        .arg("test.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("hello world\nhello world\n");
}

#[test]
fn concatenate_empty_first_and_last() {
    let assert = bat()
        .arg("empty.txt")
        .arg("test.txt")
        .arg("empty.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("hello world\n");
}

#[test]
fn concatenate_single_line() {
    let assert = bat()
        .arg("single-line.txt")
        .arg("single-line.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("Single LineSingle Line");
}

#[test]
fn concatenate_single_line_empty() {
    let assert = bat()
        .arg("single-line.txt")
        .arg("empty.txt")
        .arg("single-line.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("Single LineSingle Line");
}

#[test]
fn line_numbers() {
    let assert = bat()
        .arg("multiline.txt")
        .arg("--style=numbers")
        .arg("--decorations=always")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("   1 line 1\n   2 line 2\n   3 line 3\n   4 line 4\n");
}

#[test]
fn line_range_2_3() {
    let assert = bat()
        .arg("multiline.txt")
        .arg("--line-range=2:3")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("line 2\nline 3\n");
}

#[test]
fn line_range_first_two() {
    let assert = bat()
        .arg("multiline.txt")
        .arg("--line-range=:2")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("line 1\nline 2\n");
}

#[test]
fn line_range_last_3() {
    let assert = bat()
        .arg("multiline.txt")
        .arg("--line-range=2:")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("line 2\nline 3\nline 4\n");
}

#[test]
fn line_range_multiple() {
    let assert = bat()
        .arg("multiline.txt")
        .arg("--line-range=1:2")
        .arg("--line-range=4:4")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("line 1\nline 2\nline 4\n");
}

#[test]
fn tabs_numbers() {
    let assert = bat()
        .arg("tabs.txt")
        .arg("--tabs=4")
        .arg("--style=numbers")
        .arg("--decorations=always")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout(
            "   1     1   2   3   4
   2 1   ?
   3 22  ?
   4 333 ?
   5 4444    ?
   6 55555   ?
   7 666666  ?
   8 7777777 ?
   9 88888888    ?
",
        );
}

#[test]
fn tabs_passthrough_wrapped() {
    let assert = bat()
        .arg("tabs.txt")
        .arg("--tabs=0")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout(
            "	1	2	3	4
1	?
22	?
333	?
4444	?
55555	?
666666	?
7777777	?
88888888	?
",
        );
}

#[test]
fn tabs_4_wrapped() {
    let assert = bat()
        .arg("tabs.txt")
        .arg("--tabs=4")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout(
            "    1   2   3   4
1   ?
22  ?
333 ?
4444    ?
55555   ?
666666  ?
7777777 ?
88888888    ?
",
        );
}

#[test]
fn tabs_8_wrapped() {
    let assert = bat()
        .arg("tabs.txt")
        .arg("--tabs=8")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout(
            "        1       2       3       4
1       ?
22      ?
333     ?
4444    ?
55555   ?
666666  ?
7777777 ?
88888888        ?
",
        );
}

#[test]
fn tabs_passthrough() {
    let assert = bat()
        .arg("tabs.txt")
        .arg("--tabs=0")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout(
            "	1	2	3	4
1	?
22	?
333	?
4444	?
55555	?
666666	?
7777777	?
88888888	?
",
        );
}

#[test]
fn tabs_4() {
    let assert = bat()
        .arg("tabs.txt")
        .arg("--tabs=4")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout(
            "    1   2   3   4
1   ?
22  ?
333 ?
4444    ?
55555   ?
666666  ?
7777777 ?
88888888    ?
",
        );
}

#[test]
fn tabs_8() {
    let assert = bat()
        .arg("tabs.txt")
        .arg("--tabs=8")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout(
            "        1       2       3       4
1       ?
22      ?
333     ?
4444    ?
55555   ?
666666  ?
7777777 ?
88888888        ?
",
        );
}

#[test]
fn fail_non_existing() {
    bat().arg("non-existing-file").assert().failure();
}

#[test]
fn fail_directory() {
    bat().arg("sub_directory").assert().failure();
}

#[test]
fn do_not_exit_directory() {
    let assert = bat()
        .arg("sub_directory")
        .arg("test.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .stdout("hello world\n")
        .failure();
}

#[test]
fn pager_basic() {
    let assert = bat()
        .env("PAGER", "printf pager-output")
        .arg("--paging=always")
        .arg("test.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("pager-output");
}

#[test]
fn pager_overwrite() {
    let assert = bat()
        .env("PAGER", "printf other-pager")
        .env("BAT_PAGER", "printf pager-output")
        .arg("--paging=always")
        .arg("test.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("pager-output");
}

#[test]
fn pager_disable() {
    let assert = bat()
        .env("PAGER", "printf other-pager")
        .env("BAT_PAGER", "")
        .arg("--paging=always")
        .arg("test.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("hello world\n");
}

#[test]
fn config_location_test() {
    let assert = bat_with_config()
        .env("BAT_CONFIG_PATH", "bat.conf")
        .arg("--config-file")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("bat.conf\n");
}

#[test]
fn config_read_arguments_from_file() {
    let assert = bat_with_config()
        .env("BAT_CONFIG_PATH", "bat.conf")
        .arg("test.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("dummy-pager-from-config");
}

#[test]
fn utf16() {
    // The output will be converted to UTF-8 with a leading UTF-8 BOM
    let assert = bat()
        .arg("--plain")
        .arg("--decorations=always")
        .arg("test_UTF-16LE.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout(std::str::from_utf8(b"\xEF\xBB\xBFhello world\n").unwrap());
}

#[test]
fn can_print_file_named_cache() {
    let assert = bat_with_config()
        .arg("cache")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    println!("stdout={:#?}", stdout);
    println!("stderr={:#?}", stderr);
    assert
        .success()
        .stdout("test\n")
        .stderr("");
}

#[test]
fn can_print_file_named_cache_with_additional_argument() {
    let assert = bat_with_config()
        .arg("cache")
        .arg("test.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    println!("stdout={:#?}", stdout);
    println!("stderr={:#?}", stderr);
    assert
        .success()
        .stdout("test\nhello world\n")
        .stderr("");
}

#[test]
fn can_print_file_starting_with_cache() {
    let assert = bat_with_config()
        .arg("cache.c")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    println!("stdout={:#?}", stdout);
    println!("stderr={:#?}", stderr);
    assert
        .success()
        .stdout("test\n")
        .stderr("");
}

#[test]
fn does_not_print_unwanted_file_named_cache() {
    bat_with_config().arg("cach").assert().failure();
}

#[test]
fn unicode_wrap() {
    let assert = bat_with_config()
        .arg("unicode-wrap.txt")
        .arg("--style=numbers,snip")
        .arg("--decorations=always")
        .arg("--terminal-width=40")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout(
            "   1 ビタミンA  ビタミンD  ビタミンE  ビ
     タミンK  ビタミンB1  ビタミンB2  ナ
     イアシン  パントテン酸  ビタミンB6 
      ビタミンB12  葉酸  ビオチン  ビタ
     ミンC
   2 
   3 고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이
   4 
   5 1 บวก 2 บวก 3 บวก 4 บวก 5 บวก 6 บวก
      7 บวก 8 บวก 9 บวก 10 บวก 11 บวก 12
      บวก 13 บวก 14 บวก 15 บวก 16 บวก 17
      บวก 18 บวก 19 บวก 20
   6 
   7 Бельгия Болгария Чехия Дания Герман
     ия Эстония Ирландия Греция Испания 
     Франция Хорватия Италия Кипр Латвия
      Литва Люксембург Венгрия Мальта Ни
     дерланды Австрия Польша Португалия 
     Румыния Словения Словакия Финляндия
      Швеция Великобритания
",
        );
}

#[test]
fn snip() {
    let assert = bat()
        .arg("multiline.txt")
        .arg("--style=numbers,snip")
        .arg("--decorations=always")
        .arg("--line-range=1:2")
        .arg("--line-range=4:")
        .arg("--terminal-width=80")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout(
            "   1 line 1
   2 line 2
 ...─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ 8< ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
   4 line 4
",
        );
}

#[test]
fn empty_file_leads_to_empty_output_with_grid_enabled() {
    let assert = bat()
        .arg("empty.txt")
        .arg("--style=grid")
        .arg("--decorations=always")
        .arg("--terminal-width=80")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("");
}

#[test]
fn filename_basic() {
    let assert = bat()
        .arg("test.txt")
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("-r=0:0")
        .arg("--file-name=foo")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    println!("stdout={:#?}", stdout);
    println!("stderr={:#?}", stderr);
    assert
        .success()
        .stdout("File: foo\n")
        .stderr("");
}

#[test]
fn filename_binary() {
    let assert = bat()
        .arg("test.binary")
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("-r=0:0")
        .arg("--file-name=foo")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    println!("stdout={:#?}", stdout);
    println!("stderr={:#?}", stderr);
    assert
        .success()
        .stdout("File: foo   <BINARY>\n")
        .stderr("");
}

#[test]
fn filename_stdin() {
    let assert = bat()
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("-r=0:0")
        .arg("-")
        .write_stdin("stdin\n")
        .arg("--file-name=foo")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    println!("stdout={:#?}", stdout);
    println!("stderr={:#?}", stderr);
    assert
        .success()
        .stdout("File: foo\n")
        .stderr("");
}

#[test]
fn filename_stdin_binary() {
    let vec = vec![0; 1];
    let assert = bat_with_config()
        .arg("--decorations=always")
        .arg("--style=header")
        .write_stdin(vec)
        .arg("--file-name=foo")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    println!("stdout={:#?}", stdout);
    println!("stderr={:#?}", stderr);
    assert
        .success()
        .stdout("File: foo   <BINARY>\n")
        .stderr("");
}

#[test]
fn filename_multiple_ok() {
    let assert = bat()
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("-r=0:0")
        .arg("test.txt")
        .arg("--file-name=foo")
        .arg("single-line.txt")
        .arg("--file-name=bar")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    println!("stdout={:#?}", stdout);
    println!("stderr={:#?}", stderr);
    assert
        .success()
        .stdout("File: foo\n\nFile: bar\n")
        .stderr("");
}

#[test]
fn filename_multiple_err() {
    bat()
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("-r=0:0")
        .arg("test.txt")
        .arg("--file-name=foo")
        .arg("single-line.txt")
        .assert()
        .failure();
}

#[test]
fn header_padding() {
    let assert = bat()
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("test.txt")
        .arg("single-line.txt")
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    println!("stdout={:#?}", stdout);
    println!("stderr={:#?}", stderr);
    assert
        .stdout("File: test.txt\nhello world\n\nFile: single-line.txt\nSingle Line\n")
        .stderr("");
}

#[cfg(target_os = "linux")]
#[test]
fn file_with_invalid_utf8_filename() {
    use std::ffi::OsStr;
    use std::fs::File;
    use std::io::Write;
    use std::os::unix::ffi::OsStrExt;

    use tempdir::TempDir;

    let tmp_dir = TempDir::new("bat_test").expect("can create temporary directory");
    let file_path = tmp_dir
        .path()
        .join(OsStr::from_bytes(b"test-invalid-utf8-\xC3(.rs"));
    {
        let mut file = File::create(&file_path).expect("can create temporary file");
        writeln!(file, "dummy content").expect("can write to file");
    }

    let assert = bat()
        .arg(file_path.as_os_str())
        .assert();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    println!("stdout={:#?}", stdout);
    assert
        .success()
        .stdout("dummy content\n");
}

#[test]
fn do_not_panic_regression_tests() {
    for filename in &[
        "issue_28.md",
        "issue_190.md",
        "issue_314.hs",
        "issue_914.rb",
        "issue_915.vue",
    ] {
        bat()
            .arg("--color=always")
            .arg(&format!("regression_tests/{}", filename))
            .assert()
            .success();
    }
}

#[test]
fn do_not_detect_different_syntax_for_stdin_and_files() {
    let file = "regression_tests/issue_985.js";

    let assert_cmd_for_file = bat()
        .arg("--color=always")
        .arg("--map-syntax=*.js:Markdown")
        .arg(&format!("--file-name={}", file))
        .arg("--style=plain")
        .arg(file)
        .assert();
    let stdout_cmd_for_file = &assert_cmd_for_file.get_output().stdout;
    println!("stdout_cmd_for_file={:#?}", String::from_utf8_lossy(stdout_cmd_for_file));
    assert_cmd_for_file
        .success();

    let assert_cmd_for_stdin = bat()
        .arg("--color=always")
        .arg("--map-syntax=*.js:Markdown")
        .arg("--style=plain")
        .arg(&format!("--file-name={}", file))
        .pipe_stdin(Path::new(EXAMPLES_DIR).join(file))
        .unwrap()
        .assert();
    let stdout_cmd_for_stdin = &assert_cmd_for_stdin.get_output().stdout;
    println!("stdout_cmd_for_stdin={:#?}", String::from_utf8_lossy(stdout_cmd_for_stdin));
    assert_cmd_for_stdin
        .success();

    assert_eq!(
        from_utf8(stdout_cmd_for_file).expect("output is valid utf-8"),
        from_utf8(stdout_cmd_for_stdin).expect("output is valid utf-8")
    );
}
