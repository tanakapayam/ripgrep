use hay::SHERLOCK;
use util::{Dir, TestCommand, setup, sort_lines};

// See: https://github.com/BurntSushi/ripgrep/issues/16
rgtest!(r16, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir(".git");
    dir.create(".gitignore", "ghi/");
    dir.create_dir("ghi");
    dir.create_dir("def/ghi");
    dir.create("ghi/toplevel.txt", "xyz");
    dir.create("def/ghi/subdir.txt", "xyz");

    cmd.arg("xyz").assert_err();
});

// See: https://github.com/BurntSushi/ripgrep/issues/25
rgtest!(r25, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir(".git");
    dir.create(".gitignore", "/llvm/");
    dir.create_dir("src/llvm");
    dir.create("src/llvm/foo", "test");

    cmd.arg("test");
    eqnice!("src/llvm/foo:test\n", cmd.stdout());

    cmd.current_dir(dir.path().join("src"));
    eqnice!("llvm/foo:test\n", cmd.stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/30
rgtest!(r30, |dir: Dir, mut cmd: TestCommand| {
    dir.create(".gitignore", "vendor/**\n!vendor/manifest");
    dir.create_dir("vendor");
    dir.create("vendor/manifest", "test");

    eqnice!("vendor/manifest:test\n", cmd.arg("test").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/49
rgtest!(r49, |dir: Dir, mut cmd: TestCommand| {
    dir.create(".gitignore", "foo/bar");
    dir.create_dir("test/foo/bar");
    dir.create("test/foo/bar/baz", "test");

    cmd.arg("xyz").assert_err();
});

// See: https://github.com/BurntSushi/ripgrep/issues/50
rgtest!(r50, |dir: Dir, mut cmd: TestCommand| {
    dir.create(".gitignore", "XXX/YYY/");
    dir.create_dir("abc/def/XXX/YYY");
    dir.create_dir("ghi/XXX/YYY");
    dir.create("abc/def/XXX/YYY/bar", "test");
    dir.create("ghi/XXX/YYY/bar", "test");

    cmd.arg("xyz").assert_err();
});

// See: https://github.com/BurntSushi/ripgrep/issues/65
rgtest!(r65, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir(".git");
    dir.create(".gitignore", "a/");
    dir.create_dir("a");
    dir.create("a/foo", "xyz");
    dir.create("a/bar", "xyz");

    cmd.arg("xyz").assert_err();
});

// See: https://github.com/BurntSushi/ripgrep/issues/67
rgtest!(r67, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir(".git");
    dir.create(".gitignore", "/*\n!/dir");
    dir.create_dir("dir");
    dir.create_dir("foo");
    dir.create("foo/bar", "test");
    dir.create("dir/bar", "test");

    eqnice!("dir/bar:test\n", cmd.arg("test").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/87
rgtest!(r87, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir(".git");
    dir.create(".gitignore", "foo\n**no-vcs**");
    dir.create("foo", "test");

    cmd.arg("test").assert_err();
});

// See: https://github.com/BurntSushi/ripgrep/issues/90
rgtest!(r90, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir(".git");
    dir.create(".gitignore", "!.foo");
    dir.create(".foo", "test");

    eqnice!(".foo:test\n", cmd.arg("test").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/93
rgtest!(r93, |dir: Dir, mut cmd: TestCommand| {
    dir.create("foo", "192.168.1.1");

    eqnice!("foo:192.168.1.1\n", cmd.arg(r"(\d{1,3}\.){3}\d{1,3}").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/99
rgtest!(r99, |dir: Dir, mut cmd: TestCommand| {
    dir.create("foo1", "test");
    dir.create("foo2", "zzz");
    dir.create("bar", "test");

    eqnice!(
        sort_lines("bar\ntest\n\nfoo1\ntest\n"),
        sort_lines(&cmd.arg("-j1").arg("--heading").arg("test").stdout())
    );
});

// See: https://github.com/BurntSushi/ripgrep/issues/105
rgtest!(r105_part1, |dir: Dir, mut cmd: TestCommand| {
    dir.create("foo", "zztest");

    eqnice!("foo:1:3:zztest\n", cmd.arg("--vimgrep").arg("test").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/105
rgtest!(r105_part2, |dir: Dir, mut cmd: TestCommand| {
    dir.create("foo", "zztest");

    eqnice!("foo:1:3:zztest\n", cmd.arg("--column").arg("test").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/127
rgtest!(r127, |dir: Dir, mut cmd: TestCommand| {
    // Set up a directory hierarchy like this:
    //
    // .gitignore
    // foo/
    //   sherlock
    //   watson
    //
    // Where `.gitignore` contains `foo/sherlock`.
    //
    // ripgrep should ignore 'foo/sherlock' giving us results only from
    // 'foo/watson' but on Windows ripgrep will include both 'foo/sherlock' and
    // 'foo/watson' in the search results.
    dir.create_dir(".git");
    dir.create(".gitignore", "foo/sherlock\n");
    dir.create_dir("foo");
    dir.create("foo/sherlock", SHERLOCK);
    dir.create("foo/watson", SHERLOCK);

    let expected = "\
foo/watson:For the Doctor Watsons of this world, as opposed to the Sherlock
foo/watson:be, to a very large extent, the result of luck. Sherlock Holmes
";
    assert_eq!(expected, cmd.arg("Sherlock").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/128
rgtest!(r128, |dir: Dir, mut cmd: TestCommand| {
    dir.create_bytes("foo", b"01234567\x0b\n\x0b\n\x0b\n\x0b\nx");

    eqnice!("foo:5:x\n", cmd.arg("-n").arg("x").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/131
//
// TODO(burntsushi): Darwin doesn't like this test for some reason. Probably
// due to the weird file path.
#[cfg(not(target_os = "macos"))]
rgtest!(r131, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir(".git");
    dir.create(".gitignore", "TopÑapa");
    dir.create("TopÑapa", "test");

    cmd.arg("test").assert_err();
});

// See: https://github.com/BurntSushi/ripgrep/issues/137
//
// TODO(burntsushi): Figure out how to make this test work on Windows. Right
// now it gives "access denied" errors when trying to create a file symlink.
// For now, disable test on Windows.
#[cfg(not(windows))]
rgtest!(r137, |dir: Dir, mut cmd: TestCommand| {
    dir.create("sherlock", SHERLOCK);
    dir.link_file("sherlock", "sym1");
    dir.link_file("sherlock", "sym2");

    let expected = "\
./sherlock:For the Doctor Watsons of this world, as opposed to the Sherlock
./sherlock:be, to a very large extent, the result of luck. Sherlock Holmes
sym1:For the Doctor Watsons of this world, as opposed to the Sherlock
sym1:be, to a very large extent, the result of luck. Sherlock Holmes
sym2:For the Doctor Watsons of this world, as opposed to the Sherlock
sym2:be, to a very large extent, the result of luck. Sherlock Holmes
";
    cmd.arg("-j1").arg("Sherlock").arg("./").arg("sym1").arg("sym2");
    eqnice!(expected, cmd.stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/156
rgtest!(r156, |dir: Dir, mut cmd: TestCommand| {
    let expected = r#"#parse('widgets/foo_bar_macros.vm')
#parse ( 'widgets/mobile/foo_bar_macros.vm' )
#parse ("widgets/foobarhiddenformfields.vm")
#parse ( "widgets/foo_bar_legal.vm" )
#include( 'widgets/foo_bar_tips.vm' )
#include('widgets/mobile/foo_bar_macros.vm')
#include ("widgets/mobile/foo_bar_resetpw.vm")
#parse('widgets/foo-bar-macros.vm')
#parse ( 'widgets/mobile/foo-bar-macros.vm' )
#parse ("widgets/foo-bar-hiddenformfields.vm")
#parse ( "widgets/foo-bar-legal.vm" )
#include( 'widgets/foo-bar-tips.vm' )
#include('widgets/mobile/foo-bar-macros.vm')
#include ("widgets/mobile/foo-bar-resetpw.vm")
"#;
    dir.create("testcase.txt", expected);

    cmd.arg("-N");
    cmd.arg(r#"#(?:parse|include)\s*\(\s*(?:"|')[./A-Za-z_-]+(?:"|')"#);
    cmd.arg("testcase.txt");
    eqnice!(expected, cmd.stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/184
rgtest!(r184, |dir: Dir, mut cmd: TestCommand| {
    dir.create(".gitignore", ".*");
    dir.create_dir("foo/bar");
    dir.create("foo/bar/baz", "test");

    cmd.arg("test");
    eqnice!("foo/bar/baz:test\n", cmd.stdout());

    cmd.current_dir(dir.path().join("./foo/bar"));
    eqnice!("baz:test\n", cmd.stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/199
rgtest!(r199, |dir: Dir, mut cmd: TestCommand| {
    dir.create("foo", "tEsT");

    eqnice!("foo:tEsT\n", cmd.arg("--smart-case").arg(r"\btest\b").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/206
rgtest!(r206, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir("foo");
    dir.create("foo/bar.txt", "test");

    cmd.arg("test").arg("-g").arg("*.txt");
    eqnice!("foo/bar.txt:test\n", cmd.stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/210
#[cfg(unix)]
rgtest!(r210, |dir: Dir, mut cmd: TestCommand| {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let badutf8 = OsStr::from_bytes(&b"foo\xffbar"[..]);

    // APFS does not support creating files with invalid UTF-8 bytes.
    // https://github.com/BurntSushi/ripgrep/issues/559
    if dir.try_create(badutf8, "test").is_ok() {
        cmd.arg("-H").arg("test").arg(badutf8);
        assert_eq!(b"foo\xffbar:test\n".to_vec(), cmd.output().stdout);
    }
});

// See: https://github.com/BurntSushi/ripgrep/issues/228
rgtest!(r228, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir("foo");

    cmd.arg("--ignore-file").arg("foo").arg("test").assert_err();
});

// See: https://github.com/BurntSushi/ripgrep/issues/229
rgtest!(r229, |dir: Dir, mut cmd: TestCommand| {
    dir.create("foo", "economie");

    cmd.arg("-S").arg("[E]conomie").assert_err();
});

// See: https://github.com/BurntSushi/ripgrep/issues/251
rgtest!(r251, |dir: Dir, mut cmd: TestCommand| {
    dir.create("foo", "привет\nПривет\nПрИвЕт");

    let expected = "foo:привет\nfoo:Привет\nfoo:ПрИвЕт\n";
    eqnice!(expected, cmd.arg("-i").arg("привет").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/256
#[cfg(not(windows))]
rgtest!(r256, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir("bar");
    dir.create("bar/baz", "test");
    dir.link_dir("bar", "foo");

    eqnice!("foo/baz:test\n", cmd.arg("test").arg("foo").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/256
#[cfg(not(windows))]
rgtest!(r256_j1, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir("bar");
    dir.create("bar/baz", "test");
    dir.link_dir("bar", "foo");

    eqnice!("foo/baz:test\n", cmd.arg("-j1").arg("test").arg("foo").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/279
rgtest!(r279, |dir: Dir, mut cmd: TestCommand| {
    dir.create("foo", "test");

    eqnice!("", cmd.arg("-q").arg("test").stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/405
rgtest!(r405, |dir: Dir, mut cmd: TestCommand| {
    dir.create_dir("foo/bar");
    dir.create_dir("bar/foo");
    dir.create("foo/bar/file1.txt", "test");
    dir.create("bar/foo/file2.txt", "test");

    cmd.arg("-g").arg("!/foo/**").arg("test");
    eqnice!("bar/foo/file2.txt:test\n", cmd.stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/428
#[cfg(not(windows))]
rgtest!(r428_color_context_path, |dir: Dir, mut cmd: TestCommand| {
    dir.create("sherlock", "foo\nbar");
    cmd.args(&[
        "-A1", "-H", "--no-heading", "-N",
        "--colors=match:none", "--color=always",
        "foo",
    ]);

    let expected = format!(
        "{colored_path}:foo\n{colored_path}-bar\n",
        colored_path=
            "\x1b\x5b\x30\x6d\x1b\x5b\x33\x35\x6dsherlock\x1b\x5b\x30\x6d"
    );
    eqnice!(expected, cmd.stdout());
});

// See: https://github.com/BurntSushi/ripgrep/issues/428
rgtest!(r428_unrecognized_style, |_: Dir, mut cmd: TestCommand| {
    cmd.arg("--colors=match:style:").arg("Sherlock");
    cmd.assert_err();

    let output = cmd.cmd().output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let expected = "\
unrecognized style attribute ''. Choose from: nobold, bold, nointense, \
intense, nounderline, underline.
";
    eqnice!(expected, stderr);
});

// See: https://github.com/BurntSushi/ripgrep/issues/493
rgtest!(r493, |dir: Dir, mut cmd: TestCommand| {
    dir.create("input.txt", "peshwaship 're seminomata");

    cmd.arg("-o").arg(r"\b 're \b").arg("input.txt");
    assert_eq!(" 're \n", cmd.stdout());
});
