use mlua::{prelude::*, UserData};

use crate::prelude::*;


impl UserData for App {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(ms: &mut M) {
        ms.add_method("output", |_, this, geometry: Geometry3D| {
            Ok(this.run(geometry))
        });
    }
}


impl UserData for Geometry2D {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(ms: &mut M) {
        ms.add_method("extrude_linear", |_, this, extent: f64| {
            Ok(this.extrude_linear(extent))
        });
    }
}

impl<'lua> FromLua<'lua> for Geometry2D {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::UserData(ud) => {
                return ud.take();
            }
            _ => {
                Err(LuaError::RuntimeError(format!("value is not userdata")))
            }
        }
    }
}


impl UserData for Geometry3D {

}

impl<'lua> FromLua<'lua> for Geometry3D {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::UserData(ud) => {
                return ud.take();
            }
            _ => {
                Err(LuaError::RuntimeError(format!("value is not userdata")))
            }
        }
    }
}


pub fn lua(app: App) -> LuaResult<()> {
    let lua = Lua::new();
    let source = std::fs::read_to_string(&app.args.file).expect("Can't read that file :(");

    lua.globals().set("app", app)?;

    let f_circle = lua.create_function(|_, sides| Ok(Geometry2D::circle(sides)))?;
    let f_sphere = lua.create_function(|_, subdivisions| Ok(Geometry3D::sphere(subdivisions)))?;

    lua.globals().set("circle", f_circle)?;
    lua.globals().set("sphere", f_sphere)?;

    lua.load(source).exec()?;

    Ok(())
}
