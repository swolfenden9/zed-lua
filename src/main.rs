use mlua::prelude::*;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() -> LuaResult<()> {
    // Create a new Lua state
    let lua = Lua::new();

    // Expose Rust function 'add' to Lua
    let add_fn = lua.create_function(|_, (a, b): (i32, i32)| Ok(add(a, b)))?;

    // Expose Rust function 'greet' to Lua
    let greet_fn = lua.create_function(|_, name: String| Ok(greet(&name)))?;

    // Add the functions to Lua's global table
    let globals = lua.globals();
    globals.set("add", add_fn)?;
    globals.set("greet", greet_fn)?;

    // Lua script that uses the exposed Rust functions
    let lua_script = r#"
        print("Sum of 10 + 5: ", add(10, 5))
        print(greet("Lua"))
    "#;

    // Execute the Lua script
    lua.load(lua_script).exec()?;

    Ok(())
}
