use proclamation::prelude::*;

mod r#enum;
mod iterator;
mod r#trait;

pub fn dispatch(args: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let parsed_item: Item = parse2(item.clone())?;

    let tokens = match parsed_item {
        Item::Enum(item_enum) => r#enum::dispatch_enum(parse2(args)?, item_enum)?,
        Item::Trait(item_trait) => {
            if !args.is_empty() {
                return Err(Error::new(
                    Span::call_site(),
                    "dispatch on trait does not allow arguments",
                ));
            }
            r#trait::dispatch_trait(item_trait)?
        }
        _ => {
            return Err(Error::new(
                Span::call_site(),
                "invalid item for dispatch, only enum and trait items are allowed",
            ));
        }
    };

    Ok(quote! {
        #tokens
        #item
    })
}
