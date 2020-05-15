use argh::FromArgs;
use std::{fs, io::Write, path::PathBuf, process::Stdio};

#[derive(FromArgs)]
/// Generate Rust bindings from winmd files, using winmd crate
struct Args {
    /// namespaces to generate for
    #[argh(positional)]
    namespaces: Vec<String>,

    /// which file to write
    #[argh(option)]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();

    println!("Reading types...");
    let tr = winmd::TypeReader::from_os();

    println!("Adding type limits...");
    let mut tl: winmd::TypeLimits = Default::default();
    println!("Namespaces: {:#?}", args.namespaces);
    for ns in &args.namespaces {
        tl.insert(&tr, ns);
    }

    println!("Generating tree...");
    let timer = took::Timer::new();
    let ts = winmd::TypeStage::from_limits(&tr, &tl);
    let tt = ts.into_tree();
    println!("Took {}", timer.took());

    let path = &args.output;
    fs::create_dir_all(path.parent().unwrap())?;
    let mut f = fs::File::create(path)?;

    println!("Doing codegen and formatting......");
    let timer = took::Timer::new();
    let mut cmd = std::process::Command::new("rustfmt");
    cmd.arg("--emit").arg("stdout");
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    {
        let child = cmd.spawn()?;
        let mut stdin = child.stdin.unwrap();
        let mut stdout = child.stdout.unwrap();

        crossbeam_utils::thread::scope(|s| {
            s.spawn(move |_| {
                // workaround for, well, overflowing literals
                writeln!(&mut stdin, "#![allow(overflowing_literals)]").unwrap();

                let tokens = tt.to_tokens();
                writeln!(&mut stdin, "{}", tokens).unwrap();
            });

            s.spawn(move |_| {
                std::io::copy(&mut stdout, &mut f).unwrap();
            });
        })
        .unwrap();
    }

    let status = cmd.status()?;
    assert!(status.success());
    println!("Took {}", timer.took());
    println!("Wrote to: {:?}", args.output);

    Ok(())
}
