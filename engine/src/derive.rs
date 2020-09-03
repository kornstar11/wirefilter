use crate::scheme::Scheme;
use crate::{ExecutionContext, LhsValue, Type};
use crate::errors::Error;
use std::net::IpAddr;
///
/// Filterable trait is used to create a ExecutionContext against a particular Scheme, and then populate the ExecutionContext.
/// Idea is that users can use `#[derive(Filterable)]` and the macros will automagicly implement this trait.
///
pub trait Filterable {
    fn filter_context<'s>(&self, schema: &'s Scheme) -> Result<ExecutionContext<'s>, Error>;
}

///
/// GenContext is designed to be used on each attribute of a struct that has a derived Filterable.
///
pub trait GenContext {
    fn generate_context<'s>(&self, ctx: &mut ExecutionContext<'s>, field_name: &str) -> Result<(), Error>;
}

impl GenContext for String {
    fn generate_context<'s>(&self, ctx: &mut ExecutionContext<'s>, field_name: &str) -> Result<(), Error> {
        println!("gen_ctx String {} {}", field_name, self);
        let value: LhsValue = LhsValue::from(self.to_owned());
        ctx.set_field_value(field_name, value).map_err(Error::TypeMismatchError)?;
        Ok(())
    }
}

impl GenContext for usize {
    fn generate_context<'s>(&self, ctx: &mut ExecutionContext<'s>, field_name: &str) -> Result<(), Error>{
        println!("gen_ctx usize {}", self);
        ctx.set_field_value(field_name, LhsValue::Int(*self as _)).map_err(Error::TypeMismatchError)?;
        Ok(())
    }
}

impl GenContext for IpAddr {
    fn generate_context<'s>(&self, ctx: &mut ExecutionContext<'s>, field_name: &str) -> Result<(), Error> {
        println!("gen_ctx ipaddr {}", self.to_string());
        ctx.set_field_value(field_name, LhsValue::Ip(*self)).map_err(Error::TypeMismatchError)?;
        Ok(())
    }
}

impl<T: GenContext> GenContext for Option<T> {
    fn generate_context<'s>(&self, ctx: &mut ExecutionContext<'s>, field_name: &str) -> Result<(), Error> {
        if let Some(t) = self {
            t.generate_context(ctx, field_name);
        }
        Ok(())
    }
}

impl GenContext for Vec<(String, String)> {
    fn generate_context<'s>(&self, ctx: &mut ExecutionContext<'s>, field_name: &str) -> Result<(), Error> {
        let concat: String = self
            .iter()
            .flat_map(|(k, v)| {
                Some(format!("{}:{}\n", k, v))
            })
            .fold(String::new(), |acc, ele| acc + ele.as_str());
        if !concat.is_empty() {
            concat.generate_context(ctx, field_name)?
        }
        Ok(())
    }
}
///
/// Creates a Vec of fields that can be used to create a Scheme to be generated.
///
///
pub trait HasFields {
    fn fields() -> Vec<(String, Type)>;
}

//TODO could be part of the `declare_types` macro? Also, its pretty hacky atm, the duplication for Option is a real stinker.
pub trait GetType {
    fn ty() -> Type;
}

impl GetType for String {
    fn ty() -> Type {
        Type::Bytes
    }
}

impl GetType for Vec<(String, String)> {
    fn ty() -> Type {
        Type::Bytes
    }
}

impl GetType for usize {
    fn ty() -> Type {
        Type::Int
    }
}

impl GetType for IpAddr {
    fn ty() -> Type {
        Type::Ip
    }
}

impl GetType for Option<String> {
    fn ty() -> Type {
        Type::Bytes
    }
}

impl GetType for Option<usize> {
    fn ty() -> Type {
        Type::Int
    }
}

impl GetType for Option<IpAddr> {
    fn ty() -> Type {
        Type::Ip
    }
}

// pub fn get_type<T: GetType>() -> Type {
//
// }
