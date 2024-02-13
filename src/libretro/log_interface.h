#include "libretro.h"

const int MAX_LOG_SIZE = 4096;

// essa callback deve ser criada no rust e envida para configure_log_interface
typedef void(rs_cb_t)(enum retro_log_level level, const char *log);

// deve ser chamada para enviar fn core_log (rs_cb_t) para o CORE selecionado 
void configure_log_interface(rs_cb_t rs_cb, void *data);

void set_variable_value_as_null(void *data);

// deve ser usando em RETRO_ENVIRONMENT_GET_VARIABLE para atualizar as variáveis
bool set_new_value_variable(void *data, const char* new_value);