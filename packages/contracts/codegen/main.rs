fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let schema: schemars::schema::RootSchema = serde_json::from_slice(&std::fs::read(&args[1])?)?;
    let mut space = typify::TypeSpace::default();
    space.add_root_schema(schema)?;
    let code = prettyplease::unparse(&syn::parse2(space.to_stream())?);
    std::fs::write(&args[2], format!("// Generated. Do not edit.\n{code}"))?;
    Ok(())
}
