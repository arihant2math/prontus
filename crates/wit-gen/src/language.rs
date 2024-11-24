pub trait WitComponent {
    fn to_wit(&self) -> String;
}

#[derive(Clone)]
pub struct WitIdent(pub String);

impl WitComponent for WitIdent {
    fn to_wit(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone)]
pub struct WitType {
    pub name: WitIdent,
    pub inner: Vec<WitType>
}

impl WitType {
    pub fn bool() -> WitType {
        WitType {
            name: WitIdent("bool".to_string()),
            inner: Vec::new()
        }
    }

    pub fn s8() -> WitType {
        WitType {
            name: WitIdent("s8".to_string()),
            inner: Vec::new()
        }
    }

    pub fn s16() -> WitType {
        WitType {
            name: WitIdent("s16".to_string()),
            inner: Vec::new()
        }
    }

    pub fn s32() -> WitType {
        WitType {
            name: WitIdent("s32".to_string()),
            inner: Vec::new()
        }
    }

    pub fn s64() -> WitType {
        WitType {
            name: WitIdent("s64".to_string()),
            inner: Vec::new()
        }
    }

    pub fn u8() -> WitType {
        WitType {
            name: WitIdent("u8".to_string()),
            inner: Vec::new()
        }
    }

    pub fn u16() -> WitType {
        WitType {
            name: WitIdent("u16".to_string()),
            inner: Vec::new()
        }
    }

    pub fn u32() -> WitType {
        WitType {
            name: WitIdent("u32".to_string()),
            inner: Vec::new()
        }
    }

    pub fn u64() -> WitType {
        WitType {
            name: WitIdent("u64".to_string()),
            inner: Vec::new()
        }
    }

    pub fn f32() -> WitType {
        WitType {
            name: WitIdent("f32".to_string()),
            inner: Vec::new()
        }
    }

    pub fn f64() -> WitType {
        WitType {
            name: WitIdent("f64".to_string()),
            inner: Vec::new()
        }
    }

    pub fn char() -> WitType {
        WitType {
            name: WitIdent("char".to_string()),
            inner: Vec::new()
        }
    }

    pub fn string() -> WitType {
        WitType {
            name: WitIdent("string".to_string()),
            inner: Vec::new()
        }
    }

    pub fn list(inner: WitType) -> WitType {
        WitType {
            name: WitIdent("list".to_string()),
            inner: vec![inner]
        }
    }

    pub fn option(inner: WitType) -> WitType {
        WitType {
            name: WitIdent("option".to_string()),
            inner: vec![inner]
        }
    }

    pub fn result(ok: WitType, err: WitType) -> WitType {
        WitType {
            name: WitIdent("result".to_string()),
            inner: vec![ok, err]
        }
    }

    pub fn tuple(inner: Vec<WitType>) -> WitType {
        WitType {
            name: WitIdent("tuple".to_string()),
            inner
        }
    }
}

impl WitComponent for WitType {
    fn to_wit(&self) -> String {
        if self.inner.is_empty() {
            return self.name.to_wit();
        }
        let inner = self.inner.iter().map(|i| i.to_wit()).collect::<Vec<String>>().join(", ");
        format!("{}<{}>", self.name.to_wit(), inner)
    }
}

#[derive(Clone)]
pub struct Field {
    pub name: WitIdent,
    pub ty: WitType
}

impl WitComponent for Field {
    fn to_wit(&self) -> String {
        format!("{}: {}", self.name.to_wit(), self.ty.to_wit())
    }
}

#[derive(Clone)]
pub struct Record {
    pub name: WitIdent,
    pub fields: Vec<Field>
}

impl WitComponent for Record {
    fn to_wit(&self) -> String {
        let fields = self.fields.iter().map(|f| f.to_wit()).collect::<Vec<String>>().join(",\n");
        let output = String::from("record ");
        output + &self.name.to_wit() + " {\n" + &fields + "\n}\n"
    }
}

#[derive(Clone)]
pub struct VariantField {
    pub name: WitIdent,
    pub ty: Option<WitType>
}

impl WitComponent for VariantField {
    fn to_wit(&self) -> String {
        if let Some(ref ty) = self.ty {
            format!("{}({})", self.name.to_wit(), ty.to_wit())
        } else {
            self.name.to_wit()
        }
    }
}

#[derive(Clone)]
pub struct Variant {
    pub name: WitIdent,
    pub fields: Vec<VariantField>
}

impl WitComponent for Variant {
    fn to_wit(&self) -> String {
        let fields = self.fields.iter().map(|f| f.to_wit()).collect::<Vec<String>>().join(",\n");
        let output = String::from("variant ");
        output + &self.name.to_wit() + " {\n" + &fields + "\n}\n"
    }
}

#[derive(Clone)]
pub struct Enum {
    name: WitIdent,
    variants: Vec<WitIdent>
}

impl WitComponent for Enum {
    fn to_wit(&self) -> String {
        let variants = self.variants.iter().map(|v| v.to_wit()).collect::<Vec<String>>().join(",\n");
        let output = String::from("enum ");
        output + &self.name.to_wit() + " {\n" + &variants + "\n}\n"
    }
}

pub struct Type {
    pub name: WitIdent,
    pub expr: WitType
}

impl WitComponent for Type {
    fn to_wit(&self) -> String {
        format!("type {} = {};\n", self.name.to_wit(), self.expr.to_wit())
    }
}

pub struct Interface {
    pub name: WitIdent,
    pub components: Vec<Box<dyn WitComponent>>
}

impl WitComponent for Interface {
    fn to_wit(&self) -> String {
        let components = self.components.iter().map(|c| c.to_wit()).collect::<Vec<String>>().join("\n");
        let output = String::from("interface ");
        output + &self.name.to_wit() + " {\n" + &components + "\n}\n"
    }
}

