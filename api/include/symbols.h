#pragma once
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#else
#include <stdbool.h>
#endif

typedef uint32_t seal_sym;

/// Creates an empty symbol table. Never fails.
struct seal_sym_table *seal_create_sym_table();
/// Free the symbol table. Pointer is no longer valid after this call
void                   seal_free_sym_table(struct seal_sym_table *table);

/// Intern a string in the symbol table, passing the symbol id out in the
/// sym parameter. Fails to intern string and returns false if the string
/// is not valid UTF-8. `sym` will be unmodified in this case.
bool                   seal_intern_sym(struct seal_sym_table *table,
                                       const uint8_t *str, size_t len,
                                       seal_sym *sym);
/// Attempts to get the UTF-8 string data for an interned string. Returns
/// false and modifies no parameters if the symbol does not exist.
bool                   seal_lookup_sym(const struct seal_sym_table *table,
                                       seal_sym sym,
                                       const uint8_t **str, size_t *len);

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
#if ((defined(_MSC_VER)) && (_MSC_VER >= 1910)) || __cplusplus >= 201700

#include <optional>
#include <stdexcept>
#include <string_view>
#include <memory>

namespace seal
{
    using sym = seal_sym;

    struct sym_table_free
    {
        void operator()(seal_sym_table *ptr)
        {
            seal_free_sym_table(ptr);
        }
    };

    struct sym_table
    {
        std::unique_ptr<seal_sym_table, sym_table_free> ptr;

        sym_table()
            : ptr(seal_create_sym_table())
        {
        }

        sym intern(std::string_view str)
        {
            sym s;
            if (!seal_intern_sym(ptr.get(), (const uint8_t *)str.data(), str.size(), &s))
                throw std::logic_error("Bad UTF-8 data");
            return s;
        }

        std::optional<std::string_view> lookup(sym s) const
        {
            const uint8_t *str;
            size_t len;
            if (!seal_lookup_sym(ptr.get(), s, &str, &len))
                return std::nullopt;
            return{ { (const char *)str, len } };
        }
    };
}

#endif
#endif
