use std::{borrow::Cow, marker::PhantomData};

use mlua::{FromLua, FromLuaMulti, Function, Lua, ToLua, ToLuaMulti, Value};

use crate::{NamePart, TealMultiValue, TypeName};

///A typed wrapper around [mlua::Function]
#[derive(Debug)]
pub struct TypedFunction<'lua, Params, Response>
where
    Params: TealMultiValue,
    Response: TealMultiValue,
{
    inner_function: mlua::Function<'lua>,
    _p: PhantomData<Params>,
    _r: PhantomData<Response>,
}

impl<'lua, Params, Response> mlua::FromLua<'lua> for TypedFunction<'lua, Params, Response>
where
    Params: TealMultiValue,
    Response: TealMultiValue,
{
    fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        Ok(Self {
            inner_function: FromLua::from_lua(lua_value, lua)?,
            _p: PhantomData,
            _r: PhantomData,
        })
    }
}

impl<'lua, Params, Response> ToLua<'lua> for TypedFunction<'lua, Params, Response>
where
    Params: TealMultiValue,
    Response: TealMultiValue,
{
    #[allow(clippy::wrong_self_convention)]
    fn to_lua(self, _: &'lua Lua) -> mlua::Result<Value<'lua>> {
        Ok(Value::Function(self.inner_function))
    }
}
impl<'lua, Params, Response> TypeName for TypedFunction<'lua, Params, Response>
where
    Params: TealMultiValue,
    Response: TealMultiValue,
{
    fn collect_children(generics: &mut Vec<crate::TealType>) {
        let params = Params::get_types().into_iter().filter_map(|v| match v {
            NamePart::Symbol(_) => None,
            NamePart::Type(x) => Some(x),
        });

        generics.extend(params);
    }
    fn get_type_kind() -> crate::KindOfType {
        crate::KindOfType::Builtin
    }

    fn get_type_parts() -> Cow<'static, [crate::NamePart]> {
        let params = Params::get_types();
        let returns = Response::get_types();
        let mut v = vec!["function(".into()];
        v.extend(params);
        v.push("):(".into());
        v.extend(returns);
        v.push(")".into());
        Cow::Owned(v)
    }
}
impl<'lua, Params, Response> TypedFunction<'lua, Params, Response>
where
    Params: ToLuaMulti<'lua> + TealMultiValue,
    Response: FromLuaMulti<'lua> + TealMultiValue,
{
    ///Same as [mlua::Function::call](mlua::Function#method.call). Calls the function with the given parameters.
    pub fn call(&self, params: Params) -> mlua::Result<Response> {
        self.inner_function.call(params)
    }
    ///Calls the function with the given parameters. Panics if something has gone wrong.
    pub fn force_call(&self, params: Params) -> Response {
        self.inner_function.call(params).unwrap()
    }
}
impl<'lua, Params, Response> Clone for TypedFunction<'lua, Params, Response>
where
    Params: TealMultiValue,
    Response: TealMultiValue,
{
    fn clone(&self) -> Self {
        Self {
            inner_function: self.inner_function.clone(),
            _p: PhantomData,
            _r: PhantomData,
        }
    }
}
impl<'lua, Params, Response> From<TypedFunction<'lua, Params, Response>> for Function<'lua>
where
    Params: TealMultiValue,
    Response: TealMultiValue,
{
    fn from(fun: TypedFunction<'lua, Params, Response>) -> Self {
        fun.inner_function
    }
}
impl<'lua, Params, Response> TypedFunction<'lua, Params, Response>
where
    Params: FromLuaMulti<'lua> + TealMultiValue,
    Response: ToLuaMulti<'lua> + TealMultiValue,
{
    ///make a typed function directly from a Rust one.
    pub fn from_rust<
        Func: 'static + crate::mlu::MaybeSend + Fn(&'lua Lua, Params) -> mlua::Result<Response>,
    >(
        func: Func,
        lua: &'lua Lua,
    ) -> mlua::Result<Self> {
        Ok(Self {
            inner_function: lua.create_function(func)?,
            _p: PhantomData,
            _r: PhantomData,
        })
    }
    ///make a typed function directly from a Rust one.
    pub fn from_rust_mut<
        Func: 'static + crate::mlu::MaybeSend + FnMut(&'lua Lua, Params) -> mlua::Result<Response>,
    >(
        func: Func,
        lua: &'lua Lua,
    ) -> mlua::Result<Self> {
        Ok(Self {
            inner_function: lua.create_function_mut(func)?,
            _p: PhantomData,
            _r: PhantomData,
        })
    }
}
impl<'lua, Params, Response> TypedFunction<'lua, Params, Response>
where
    Params: ToLuaMulti<'lua> + TealMultiValue,
    Response: TealMultiValue,
{
    ///call a function without trying to convert it to a rust type.
    pub fn call_as_lua(&self, params: Params) -> mlua::Result<mlua::Value<'lua>> {
        self.inner_function.call(params)
    }
}
