#include <stddef.h>
#include <stdint.h>

typedef struct Person Person;

int32_t lib_greet_greet(const uint8_t *ptr, size_t len, uint8_t **out_ptr, size_t *out_len);

/**
 * # Safety
 * Caller must ensure `person_ptr` is valid.
 * `person_ptr` will be set to point to a `Person` struct on the heap, caller must manually free memory using  the `free_person` function.
 */
int32_t lib_greet_create_new_person(struct Person **instance_ptr,
                                    const uint8_t *ptr,
                                    size_t len);

/**
 * # Safety
 * Caller must ensure `instance_ptr` is a valid address provided by the function `create_new_person`.
 * Caller must ensure `ptr` and `len` provide valid information for a bytes buffer that contains an encoded `Greetings` proto message.
 * Caller must ensure `out_ptr` and `out_len` are valid.
 * The `out_ptr` and `out_len` will set the information needed to read a bytes buffer containing an encoded `Response` proto message.
 */
int32_t lib_greet_person_greet(struct Person *instance_ptr,
                               const uint8_t *ptr,
                               size_t len,
                               uint8_t **out_ptr,
                               size_t *out_len);

/**
 * # Safety
 * Caller must ensure there are no other references to the structure.
 * `ptr` must be an address provided by the function `create_new_person`
 */
void lib_greet_free_person(struct Person *ptr);

/**
 * # Safety
 * Function must be called after a function that has an output.
 * With the same address and len the output was pointed to.
 */
void lib_greet_free_buffer(uint8_t *ptr, size_t len);
