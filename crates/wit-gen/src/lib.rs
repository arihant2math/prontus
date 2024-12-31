use crate::language::WitComponent;
use convert_case::{Case, Casing};
use std::path::PathBuf;
use syn::{GenericArgument, ItemEnum, ItemStruct, PathArguments, Type};

mod language;

fn find_structs(file: syn::File) -> Vec<ItemStruct> {
    let mut structs = Vec::new();
    for item in file.items {
        if let syn::Item::Struct(item) = item {
            structs.push(item);
        }
    }
    structs
}

fn find_enums(file: syn::File) -> Vec<syn::ItemEnum> {
    let mut enums = Vec::new();
    for item in file.items {
        if let syn::Item::Enum(item) = item {
            enums.push(item);
        }
    }
    enums
}

fn convert_type(ty: &Type) -> language::WitType {
    match ty {
        Type::Array(array) => {
            let inner = convert_type(&*array.elem);
            language::WitType {
                name: language::WitIdent("list".to_string()),
                inner: vec![inner],
            }
        }
        Type::Path(path) => {
            let working_type = path.path.segments.last().unwrap();
            let inner = match &working_type.arguments {
                PathArguments::AngleBracketed(inner) => inner
                    .args
                    .iter()
                    .map(|arg| match arg {
                        GenericArgument::Type(ty) => convert_type(ty),
                        _ => unimplemented!(),
                    })
                    .collect(),
                _ => Vec::new(),
            };
            let name = working_type.ident.to_string();
            match &*name {
                "bool" => language::WitType::bool(),
                "s8" => language::WitType::s8(),
                "s16" => language::WitType::s16(),
                "s32" => language::WitType::s32(),
                "s64" => language::WitType::s64(),
                "u8" => language::WitType::u8(),
                "u16" => language::WitType::u16(),
                "u32" => language::WitType::u32(),
                "u64" => language::WitType::u64(),
                "f32" => language::WitType::f32(),
                "f64" => language::WitType::f64(),
                "char" => language::WitType::char(),
                "String" => language::WitType::string(),
                "Vec" => language::WitType::list(inner[0].clone()),
                "Option" => language::WitType::option(inner[0].clone()),
                "Result" => language::WitType::result(inner[0].clone(), inner[1].clone()),

                s => language::WitType {
                    name: language::WitIdent(s.to_case(Case::Kebab)),
                    inner,
                },
            }
        }
        Type::Paren(paren) => convert_type(&*paren.elem),
        _ => unimplemented!(),
    }
}

fn convert_struct(item_struct: ItemStruct) -> language::Record {
    let name = item_struct.ident.to_string();

    let mut fields = vec![];
    for field in item_struct.fields {
        let name = field.ident.unwrap().to_string();
        let ty = convert_type(&field.ty);
        fields.push(language::Field {
            name: language::WitIdent(name.to_case(Case::Kebab)),
            ty,
        });
    }

    language::Record {
        name: language::WitIdent(name.to_case(Case::Kebab)),
        fields,
    }
}

fn convert_enum(item_enum: ItemEnum) -> language::Variant {
    // TODO: support WIT enums
    let name = item_enum.ident.to_string().to_case(Case::Kebab);
    let mut variant_fields = vec![];
    for variant in item_enum.variants.iter() {
        let name = variant.ident.to_string().to_case(Case::Kebab);
        let ty = match &variant.fields {
            syn::Fields::Unit => None,
            syn::Fields::Named(_) => {
                unimplemented!()
            }
            syn::Fields::Unnamed(fields) => Some(convert_type(&fields.unnamed[0].ty)),
        };
        variant_fields.push(language::VariantField {
            name: language::WitIdent(name),
            ty,
        });
    }
    language::Variant {
        name: language::WitIdent(name.to_case(Case::Kebab)),
        fields: variant_fields,
    }
}

pub struct WitGenerator {
    pub files: Vec<PathBuf>,
    pub interface_name: String,
    pub output: PathBuf,
}

impl WitGenerator {
    fn generate(&self) -> String {
        let mut all_structs = Vec::new();
        let mut all_enums = Vec::new();
        for file in self.files.iter() {
            let contents = std::fs::read_to_string(file).unwrap();
            // to token stream
            let file = syn::parse_file(&contents).unwrap();
            let structs: Vec<_> = find_structs(file.clone())
                .into_iter()
                .map(convert_struct)
                .collect();
            all_structs.extend(structs);

            let enums: Vec<_> = find_enums(file).into_iter().map(convert_enum).collect();
            all_enums.extend(enums);
        }
        let mut components: Vec<Box<dyn WitComponent>> = all_structs
            .into_iter()
            .map(|r| Box::new(r) as Box<dyn WitComponent>)
            .collect();
        let enum_components: Vec<Box<dyn WitComponent>> = all_enums
            .into_iter()
            .map(|r| Box::new(r) as Box<dyn WitComponent>)
            .collect();
        components.extend(enum_components);
        let interface = language::Interface {
            name: language::WitIdent(self.interface_name.clone()),
            components,
        };
        let output = interface.to_wit();
        output
    }

    pub fn run(&self) {
        let output = self.generate();
        // create parent dir of output
        std::fs::create_dir_all(self.output.parent().unwrap()).unwrap();
        std::fs::write(&self.output, output).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let gen = super::WitGenerator {
            files: vec![std::path::PathBuf::from(
                "D:/Documents/Programming/prontus/crates/settings/src/lib.rs",
            )],
            interface_name: "settings".into(),
            output: std::path::PathBuf::from("settings.wit"),
        };
        let text = gen.generate();
        println!("{}", text);
    }
}
