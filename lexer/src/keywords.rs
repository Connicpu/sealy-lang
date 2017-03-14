use TokenType as TT;

pub fn match_keyword(ident: &str) -> Option<TT> {
    KEYWORDS.binary_search_by_key(&ident, |tup| tup.1).ok().map(|i| KEYWORDS[i].0)
}

static KEYWORDS: &'static [(TT, &'static str)] = &[(TT::Break, "break"),
                                                   (TT::Const, "const"),
                                                   (TT::Continue, "continue"),
                                                   (TT::Else, "else"),
                                                   (TT::Enum, "enum"),
                                                   (TT::Extern, "extern"),
                                                   (TT::Function, "fn"),
                                                   (TT::For, "for"),
                                                   (TT::If, "if"),
                                                   (TT::Impl, "impl"),
                                                   (TT::Impls, "impls"),
                                                   (TT::In, "in"),
                                                   (TT::Let, "let"),
                                                   (TT::Loop, "loop"),
                                                   (TT::Match, "match"),
                                                   (TT::Mod, "mod"),
                                                   (TT::New, "new"),
                                                   (TT::Nil, "nil"),
                                                   (TT::Return, "return"),
                                                   (TT::SelfKw, "self"),
                                                   (TT::Struct, "struct"),
                                                   (TT::Throw, "throw"),
                                                   (TT::Trait, "trait"),
                                                   (TT::Type, "type"),
                                                   (TT::Use, "use"),
                                                   (TT::While, "while")];