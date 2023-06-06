use rune::{Context, Diagnostics, Source, Sources, Vm, Error};
use rune::runtime::{Value, GuardedArgs};
use rune::termcolor::{ColorChoice, StandardStream};
use std::path::Path;
use std::sync::Arc;

/// Call a rune function at the given path and convert the result to a string.
pub fn call_and_get_string<ArgT>(path: &Path, function_name: &str, arguments: ArgT) -> Result<String, Error>
where ArgT: GuardedArgs {
    let value = call(path, function_name, arguments)?;
    match value {
        Value::Integer(_) => Ok(value.into_integer()?.to_string()),
        Value::String(_) => Ok(value.into_string()?.take()?),
        _ => Err(rune::Error::msg("no handling for type returned from script")),
    }
}

/// Call a rune function at the given path.
fn call<ArgT>(path: &Path, function_name: &str, arguments: ArgT) -> Result<Value, Error>
where ArgT: GuardedArgs {
    let context = Context::with_default_modules()?;
    let runtime = Arc::new(context.runtime());

    let mut sources = Sources::new();
    sources.insert(Source::from_path(path)?);

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let unit = result?;
    let mut vm = Vm::new(runtime, Arc::new(unit));

    let output = vm.call([function_name], arguments)?;
    Ok(output)
}