use proclamation::prelude::*;

pub fn dispatch_enum(r#trait: Path, r#enum: ItemEnum) -> Result<TokenStream> {
    let ItemEnum {
        ident, variants, ..
    } = r#enum;

    let variants = collect_variants(variants)?;

    Ok(quote! {
        #r#trait!{
            #ident {#(
                #variants,
            )*}
        }
    })
}

pub fn collect_variants(variants: impl IntoIterator<Item = Variant>) -> Result<Vec<Ident>> {
    let mut vars = Vec::new();

    for Variant { ident, fields, .. } in variants {
        let Fields::Unnamed(fields) = fields else {
            return Err(Error::new_spanned(
                fields,
                "only tuple variants with exactly one field are allowed",
            ));
        };
        if fields.unnamed.len() != 1 {
            return Err(Error::new_spanned(
                fields,
                "only tuple variants with exactly one field are allowed",
            ));
        }

        vars.push(ident);
    }

    Ok(vars)
}
