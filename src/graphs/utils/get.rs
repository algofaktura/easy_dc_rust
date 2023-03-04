use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs;

pub fn get_graph_path(order_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let orders_dir = Path::new("orders");
    let order_dir = orders_dir.join(order_name);
    let graph_dir = order_dir.join("graphs");
    let graph_data_file = graph_dir.join("graph_data.rs");

    let graph_data_module = {
        let contents = fs::read_to_string(&graph_data_file)?;
        let module_name = format!("{}_graph_data", order_name);
        let module_source = format!("mod {} {{\n{}\n}}", module_name, contents);
        let module = compile::run_compiler(&module_source, &graph_data_file)?;
        let module = module.into_inner();
        let module_name = module.name().to_string();
        let module_file = graph_dir.join(format!("{}.rs", module_name));
        fs::write(&module_file, &module.source)?;
        compile::load_module_from_path(&module_file)?
    };

    Ok(graph_data_module)
}

mod compile {
    use rustc_driver::{Callbacks, Compilation};
    use rustc_interface::{interface::Compiler, Queries};
    use rustc_session::{config::Input, Session};
    use rustc_span::edition::Edition;
    use std::path::PathBuf;
    use std::{fs, panic};

    pub fn run_compiler(module_source: &str, file_path: &PathBuf) -> Result<rustc_interface::interface::Module<'_>, Box<dyn std::error::Error>> {
        let span = rustc_span::edition::Edition::DEFAULT;
        let input = Input::Str(module_source.to_owned());
        let mut callbacks = Callbacks::new();
        let mut rustc_args = vec![
            "rustc".to_owned(),
            "--edition".to_owned(),
            span.to_string(),
            "--crate-name".to_owned(),
            "tmp".to_owned(),
            "--error-format".to_owned(),
            "json".to_owned(),
            "--emit".to_owned(),
            "metadata".to_owned(),
        ];
        let options = rustc_interface::interface::Config {
            input,
            crate_name: "tmp".to_owned(),
            error_format: None,
            requested_kind: None,
            edition: Edition::from_str(&span.to_string())?,
            maybe_sysroot: None,
            lint_caps: Default::default(),
            lint_opts: Default::default(),
            describe_defmt: false,
            make_codegen_backend: None,
            registry: None,
        };
        let mut session = Session::new(&rustc_args, None);
        session.parse_sess.span_diagnostic.set_continue_after_error(true);
        let compiler = Compiler::new(Box::new(callbacks));
        let compilation = compiler.enter(|queries| {
            queries.expansion().unwrap();
            queries.parse().unwrap();
            queries.analyze().unwrap();
            queries.generate_code().unwrap()
        })?;

        let module = compilation.modules.iter().next().unwrap();

        Ok(module)
    }

    pub fn load_module_from_path(path: &PathBuf) -> Result<rustc_interface::interface::Module<'_>, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let mut callbacks = Callbacks::new();
        let mut rustc_args = vec![
            "rustc".to_owned(),
            "--error-format".to_owned(),
            "json".to_owned(),
            "--emit".to_owned(),
            "dep-info".to_owned(),
            "--emit".to_owned(),
            "metadata".to_owned(),
            "--out-dir".to_owned(),
            "target/debug/deps".to_owned(),
            "--target".to_owned(),
            "x86_64-unknown-linux-gnu".to_owned(),
        ];
        let mut session = build_session_with_args(&rustc_args, path.parent().unwrap())?;
        callbacks.config.configure_from_args(&session.opts)?;
        let mut host_config = callbacks.host.into();
        host_config.features |= rustc_driver::features::Feature::rustc_private;
        let conf = interface::Config {
            host: host_config,
            ..Default::default()
        };
        let input = Input::Str {
            name: path.display().to_string(),
            input: contents,
        };
        let mut compiler = interface::Compiler::new(conf)?;
        let module = compiler.enter(|| compiler.run_compiler(&input, &mut callbacks))?;
        Ok(module)
    }
}