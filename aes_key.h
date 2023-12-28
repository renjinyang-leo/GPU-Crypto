#include <memory>

#define AES_KEY_SIZE 240*sizeof(uint8_t)

struct AesKey
{
    uint8_t *key = nullptr;
};

extern void create_aes_key(uint8_t *seed, AesKey &aes_key);