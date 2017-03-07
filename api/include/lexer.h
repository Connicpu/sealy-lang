#pragma once
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#else
#include <stdbool.h>
#endif

/// Constructs a lexer which points to the input data. You must ensure the input
/// string lives until after the last time you call next_token or clone_lexer on
/// any lexer using this input. Construction will fail (and return null) in
/// exactly 1 case: input is not valid UTF-8 data.
struct Lexer *     seal_create_lexer(const uint8_t *input, size_t len);

/// Creates a copy of the lexer in exactly the same state. Both lexers will produce
/// identical values on the nth subsequent call to next_token. Both lexers point to
/// the original input string.
struct Lexer *     seal_clone_lexer(const struct Lexer *lexer);

/// Frees the lexer object. The pointer is no longer valid after this call.
void               seal_free_lexer(struct Lexer *lexer);

/// Attempts to retrieve the next token from the stream. Different mapipulations will
/// be performed on `token` depending on the return value of this function:
///     Tok_None  => The stream has ended, token will not be modified
///     Tok_Token => The `tok` union field will be filled with data about the token
///     Tok_Error => The `err` union field will contain error information
///
/// Once None or Error has been returned, you should no longer call this function
/// or clone_lexer
enum TokNextResult seal_next_token(struct Lexer *lexer, union TokResult *token);

enum TokenType {
    TT_Identifier = 0,
    TT_Whitespace,
    TT_NewLine,

    TT_Comment,
    TT_DocComment,
    TT_ModuleDocComment,

    TT_IntLiteral,
    TT_HexLiteral,
    TT_OctLiteral,
    TT_BinLiteral,
    TT_FloatLiteral,
    TT_StringLiteral,
    TT_CharLiteral,
    TT_Label,

    TT_Break,
    TT_Const,
    TT_Continue,
    TT_Else,
    TT_Enum,
    TT_Extern,
    TT_Function,
    TT_For,
    TT_If,
    TT_Impl,
    TT_Impls,
    TT_In,
    TT_Let,
    TT_Loop,
    TT_Match,
    TT_Mod,
    TT_New,
    TT_Nil,
    TT_Return,
    TT_Struct,
    TT_Throw,
    TT_Trait,
    TT_Type,
    TT_Use,
    TT_While,

    TT_OpenCurly,
    TT_CloseCurly,
    TT_OpenParen,
    TT_CloseParen,
    TT_OpenBracket,
    TT_CloseBracket,
    TT_Semicolon,
    TT_Colon,
    TT_Comma,
    TT_Question,
    TT_Dot,
    TT_FatArrow,

    TT_RangeExclusive,
    TT_RangeInclusive,

    TT_Equal,
    TT_NotEqual,
    TT_LessThan,
    TT_LessOrEqual,
    TT_GreaterThan,
    TT_GreaterOrEqual,

    TT_Not,
    TT_Tilde,
    TT_At,

    TT_LogicalAnd,
    TT_LogicalOr,

    TT_LogicalAndAssign,
    TT_LogicalOrAssign,

    TT_Add,
    TT_Sub,
    TT_Mul,
    TT_Div,
    TT_Rem,
    TT_DivRem,

    TT_BitAnd,
    TT_BitOr,
    TT_BitXor,

    TT_Shl,
    TT_Shr,
    TT_LShr,

    TT_Assign,
    TT_AddAssign,
    TT_SubAssign,
    TT_MulAssign,
    TT_DivAssign,
    TT_RemAssign,
    TT_ModAssign,

    TT_BitAndAssign,
    TT_BitOrAssign,
    TT_BitXorAssign,

    TT_ShlAssign,
    TT_ShrAssign,
    TT_LShrAssign,

    TT_COUNT,
};

enum TokNextResult {
    Tok_None = 0,
    Tok_Token = 1,
    Tok_Error = 2,
};

enum TokErrorKind {
    Tok_UnexpectedCharacter = 0,
    Tok_TooManyCloseCurlies,
};

struct TokLocation {
    size_t line;
    size_t column;
    size_t index;
};

struct TokError {
    enum TokErrorKind error;
    struct TokLocation loc;
    uint32_t character;
};

struct Tok {
    struct TokLocation left;
    struct TokLocation right;

    enum TokenType tt;
    const uint8_t *span;
    size_t span_len;
};

union TokResult {
    struct Tok tok;
    struct TokError err;
};

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus

#if ((defined(_MSC_VER)) && (_MSC_VER >= 1910)) || __cplusplus >= 201700
#include <string_view>
inline Lexer *seal_create_lexer(const std::string_view &input)
{
    return seal_create_lexer((const uint8_t *)input.data(), input.length());
}
#else
#include <string>
inline Lexer *seal_create_lexer(const std::string &input)
{
    return seal_create_lexer((const uint8_t *)input.data(), input.length());
}
#endif

inline TokNextResult seal_next_token(Lexer *lexer, TokResult &token)
{
    return seal_next_token(lexer, &token);
}

#endif
