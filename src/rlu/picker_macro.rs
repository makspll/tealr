///Creates a new type that is a union of the types you gave.
///
///It gets translated to a [union](https://github.com/teal-language/tl/blob/master/docs/tutorial.md#union-types) type in `teal`
///and an enum on Rust.
///
///# Warning:
///`teal` has a few restrictions on what it finds a valid union types. `tealr` does ***NOT*** check if the types you put in are a valid combination
///
///# Example
///```no_run
///# use tealr::create_union_rlua;
///create_union_rlua!(pub enum YourPublicType = String | f64 | bool);
///create_union_rlua!(pub enum YourType = String | f64 | bool);
///```
#[macro_export]
macro_rules! create_union_rlua {
    ($visibility:vis $(Derives($($derives:ident), +))? enum $type_name:ident = $($sub_types:ident) | +) => {
        #[derive(Clone,$($($derives ,)*)*)]
        #[allow(non_camel_case_types)]
        $visibility enum $type_name {
            $($sub_types($sub_types) ,)*
        }
        impl<'lua> $crate::rlu::rlua::ToLua<'lua> for $type_name {
            fn to_lua(self, lua: $crate::rlu::rlua::Context<'lua>) -> ::std::result::Result<$crate::rlu::rlua::Value<'lua>, $crate::rlu::rlua::Error> {
                match self {
                    $($type_name::$sub_types(x) => x.to_lua(lua),)*
                }
            }
        }
        impl<'lua> $crate::rlu::rlua::FromLua<'lua> for $type_name {
            fn from_lua(value: $crate::rlu::rlua::Value<'lua>, lua: $crate::rlu::rlua::Context<'lua>) -> ::std::result::Result<Self, $crate::rlu::rlua::Error> {
                $(match $sub_types::from_lua(value.clone(),lua) {
                    Ok(x) => return Ok($type_name::$sub_types(x)),
                    Err($crate::rlu::rlua::Error::FromLuaConversionError{from:_,to:_,message:_}) => {}
                    Err(x) => return Err(x)
                };)*
                Err($crate::rlu::rlua::Error::FromLuaConversionError{
                    to: stringify!( $($sub_types)|* ),
                    from: $crate::rlu::get_type_name(&value),
                    message: None
                })
            }
        }
        impl $crate::TypeName for $type_name {
            fn get_type_parts() -> ::std::borrow::Cow<'static,[$crate::NamePart]> {
                let mut name = Vec::new();
                $(
                    name.append(&mut $sub_types::get_type_parts().to_vec());
                    name.push(" | ".into());
                )*
                name.pop();
                std::borrow::Cow::Owned(name)
            }
            fn collect_children(v: &mut Vec<$crate::TealType>) {
                use $crate::TealMultiValue;
                $(
                    v.extend(
                        ($sub_types::get_types(
                        )
                        .into_iter()
                        ).filter_map(|v| {
                            if let $crate::NamePart::Type(x) = v {
                                Some(x)
                            } else {
                                None
                            }
                        })
                    );
                )*
            }
        }
    };
}
