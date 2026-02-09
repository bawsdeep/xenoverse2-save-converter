#ifndef XENOVERSE2_CONVERTER_H
#define XENOVERSE2_CONVERTER_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Converts a PS4 save file to PC-ready format
 * @param data Pointer to the input data
 * @param data_len Length of the input data
 * @param input_path Path to the input file (for reference)
 * @param dir Directory path
 * @param[out] out_len Length of the output data
 * @return Pointer to the converted data (must be freed with free_buffer)
 */
uint8_t* ps4_to_pcready_c(const uint8_t* data, size_t data_len, 
                          const char* input_path, const char* dir, 
                          size_t* out_len);

/**
 * Converts a PC-ready save file to PS4 format
 * @param data Pointer to the input data
 * @param data_len Length of the input data
 * @param input_path Path to the input file (for reference)
 * @param dir Directory path
 * @param has_leftovers_flag Flag indicating if leftovers exist
 * @param[out] out_len Length of the output data
 * @return Pointer to the converted data (must be freed with free_buffer)
 */
uint8_t* pcready_to_ps4_c(const uint8_t* data, size_t data_len, 
                          const char* input_path, const char* dir, 
                          bool has_leftovers_flag, 
                          size_t* out_len);

/**
 * Automatically detects the format and converts accordingly
 * @param data Pointer to the input data
 * @param data_len Length of the input data
 * @param input_path Path to the input file (for reference)
 * @param dir Directory path
 * @param[out] out_len Length of the output data
 * @return Pointer to the converted data (must be freed with free_buffer)
 */
uint8_t* convert_auto_c(const uint8_t* data, size_t data_len, 
                        const char* input_path, const char* dir, 
                        size_t* out_len);

/**
 * Frees memory allocated by the conversion functions
 * @param ptr Pointer to the memory to free
 */
void free_buffer(uint8_t* ptr);

#ifdef __cplusplus
}
#endif

#endif // XENOVERSE2_CONVERTER_H