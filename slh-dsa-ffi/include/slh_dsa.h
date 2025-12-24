#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


#ifndef SLH_DSA_H
#define SLH_DSA_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum SlhDsaStatus {
  Ok = 0,
  NullPtr = 1,
  InvalidParam = 2,
  InvalidLength = 3,
  DecodeError = 4,
  VerifyFailed = 5,
} SlhDsaStatus;

const char *slh_dsa_parameter_name(ParameterSetId param);

uintptr_t slh_dsa_signing_key_len(ParameterSetId param);

uintptr_t slh_dsa_verifying_key_len(ParameterSetId param);

uintptr_t slh_dsa_signature_len(ParameterSetId param);

enum SlhDsaStatus slh_dsa_keypair_generate(ParameterSetId param,
                                           uint8_t *signing_key_out,
                                           uintptr_t signing_key_len,
                                           uint8_t *verifying_key_out,
                                           uintptr_t verifying_key_len);

enum SlhDsaStatus slh_dsa_sign(ParameterSetId param,
                               const uint8_t *signing_key,
                               uintptr_t signing_key_len,
                               const uint8_t *msg,
                               uintptr_t msg_len,
                               const uint8_t *ctx,
                               uintptr_t ctx_len,
                               uint8_t *signature_out,
                               uintptr_t signature_len);

enum SlhDsaStatus slh_dsa_sign_deterministic(ParameterSetId param,
                                             const uint8_t *signing_key,
                                             uintptr_t signing_key_len,
                                             const uint8_t *msg,
                                             uintptr_t msg_len,
                                             const uint8_t *ctx,
                                             uintptr_t ctx_len,
                                             uint8_t *signature_out,
                                             uintptr_t signature_len);

enum SlhDsaStatus slh_dsa_verifying_key_from_signing_key(ParameterSetId param,
                                                         const uint8_t *signing_key,
                                                         uintptr_t signing_key_len,
                                                         uint8_t *verifying_key_out,
                                                         uintptr_t verifying_key_len);

enum SlhDsaStatus slh_dsa_verify(ParameterSetId param,
                                 const uint8_t *verifying_key,
                                 uintptr_t verifying_key_len,
                                 const uint8_t *msg,
                                 uintptr_t msg_len,
                                 const uint8_t *ctx,
                                 uintptr_t ctx_len,
                                 const uint8_t *signature,
                                 uintptr_t signature_len);

const char *slh_dsa_status_string(enum SlhDsaStatus status);

bool slh_dsa_parameter_set_is_valid(uint32_t raw);

ParameterSetId slh_dsa_parameter_set_from_u32(uint32_t raw);

uint32_t slh_dsa_parameter_set_to_u32(ParameterSetId param);

bool slh_dsa_signature_verify_result_to_bool(enum SlhDsaStatus status);

void slh_dsa_zeroize(uint8_t *ptr, uintptr_t len);

#endif  /* SLH_DSA_H */
