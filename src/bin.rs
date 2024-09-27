use feather::prelude::*;
use mlua::Result;


fn main() -> Result<()> {
    let app = App::new("feather");

    lua(app)?;

    Ok(())
}
