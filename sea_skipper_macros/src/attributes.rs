pub mod derive_attr {
    use bae::FromAttributes;

    /// Attributes for Models and ActiveModels
    #[derive(Default, FromAttributes)]
    pub struct SeaOrm {
        pub active_model: Option<syn::Ident>,
        pub column: Option<syn::Ident>,
        pub entity: Option<syn::Ident>,
        pub model: Option<syn::Ident>,
        pub primary_key: Option<syn::Ident>,
        pub relation: Option<syn::Ident>,
        pub schema_name: Option<syn::Lit>,
        pub table_name: Option<syn::Lit>,
        pub table_iden: Option<()>,
    }
}
