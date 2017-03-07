#pragma once
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#else
#include <stdbool.h>
#endif

/// Parse a seal module. Consumes the lexer, pass a clone if you would
/// like to use it after this function call. Returns true if parsing
/// was successful. If parsing is successful, `ast` field is set, otherwise
/// `err` is set. In either case, ensure you free the appropriate pointer.
bool seal_parse(struct seal_lexer *lexer,
                struct seal_sym_table *syms,
                union seal_parse_result *result);
void seal_free_ast(struct seal_ast *ast);
void seal_free_parse_error(struct seal_parse_error *err);

void seal_print_ast(const struct seal_ast *ast);

union seal_parse_result {
    struct seal_ast *ast;
    struct seal_parse_error *err;
};

struct seal_parse_error {
    const uint8_t *message;
    size_t message_len;
};

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus

inline bool seal_parse(seal_lexer *lexer, seal_sym_table *syms, seal_parse_result &result) {
    return seal_parse(lexer, syms, &result);
}

#if ((defined(_MSC_VER)) && (_MSC_VER >= 1910)) || __cplusplus >= 201700
#include "lexer.h"
#include "symbols.h"

namespace seal
{
    struct ast_free
    {
        void operator()(seal_ast *ptr)
        {
            seal_free_ast(ptr);
        }
    };

    struct parse_error_free
    {
        void operator()(seal_parse_error *ptr)
        {
            seal_free_parse_error(ptr);
        }
    };

    struct ast
    {
        std::unique_ptr<seal_ast, ast_free> ptr;

        void print() const
        {
            seal_print_ast(ptr.get());
        }
    };

    struct parse_error
    {
        std::unique_ptr<seal_parse_error, parse_error_free> ptr;

        std::string_view message() const
        {
            return{ (const char *)ptr->message, ptr->message_len };
        }
    };

    std::variant<ast, parse_error> parse(lexer &&lexer, sym_table &syms)
    {
        seal_parse_result result;
        if (seal_parse(lexer.ptr.release(), syms.ptr.get(), result))
        {
            ast a;
            a.ptr.reset(result.ast);
            return a;
        }
        else
        {
            parse_error e;
            e.ptr.reset(result.err);
            return e;
        }
    }
}

#endif

#endif
