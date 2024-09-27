use std::sync::{Arc, RwLock};

use mlua::prelude::*;

use feather::*;


enum Geometry {
    G2D(Geometry2D),
    G3D(Geometry3D),
}


fn main() -> LuaResult<()> {
    let lua = Lua::new();

    let app = App::new("feather");
    let source = std::fs::read_to_string(&app.args.file).expect("Can't read that file :(");

    let storage: Arc<RwLock<Vec<Geometry>>> = Arc::new(RwLock::new(Vec::new()));

    let f_circle = {
        let storage = storage.clone();
        lua.create_function(move |_, sides| {
            let g = Geometry2D::circle(sides);
            let mut storage = storage.write().unwrap();
            storage.push(Geometry::G2D(g));
            Ok(storage.len() - 1)
        }).unwrap()
    };

    let f_sphere = {
        let storage = storage.clone();
        lua.create_function(move |_, subdivisions| {
            let g = Geometry3D::sphere(subdivisions);
            let mut storage = storage.write().unwrap();
            storage.push(Geometry::G3D(g));
            Ok(storage.len() - 1)
        }).unwrap()
    };

    let f_extrude_linear = {
        let storage = storage.clone();
        lua.create_function(move |_, (object, extent): (usize, f64)| {
            let mut storage = storage.write().unwrap();
            if let Geometry::G2D(geometry) = &storage[object] {
                let extruded = geometry.extrude_linear(extent);
                storage.push(Geometry::G3D(extruded));
                Ok(storage.len() - 1)
            } else {
                Err(LuaError::external("Can't extrude a 3D object"))
            }
        }).unwrap()
    };

    let f_output = {
        let storage = storage.clone();
        lua.create_function(move |_, object: usize| {
            let storage = storage.read().unwrap();
            if let Geometry::G3D(geometry) = &storage[object] {
                app.run(geometry.clone());
                Ok(())
            } else {
                Err(LuaError::external("Can't export/display a 2D object"))
            }
        }).unwrap()
    };

    lua.globals().set("circle", f_circle)?;
    lua.globals().set("sphere", f_sphere)?;
    lua.globals().set("extrude_linear", f_extrude_linear)?;
    lua.globals().set("output", f_output)?;

    lua.load(source).exec()?;

    Ok(())
}
