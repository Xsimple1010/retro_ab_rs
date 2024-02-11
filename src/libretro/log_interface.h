#include "libretro.h"

const int MAX_LOG_SIZE = 4096;

// essa callback deve ser criada no rust e envida para set_rs_cb
typedef void(rs_cb_t)(enum retro_log_level level, const char *log);


// essa é a primeira fn que deve ser chamada
void configure(rs_cb_t rs_cb, void *data);

// deve ser chamada quando o CORE nao estive mais sendo usando
void deinit();