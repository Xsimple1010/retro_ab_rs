#include <stdarg.h>
#include <stdio.h>
#include "log_interface.h"

static void (*rs_cb)(enum retro_log_level level, const char *fmt);

void core_log_cb(enum retro_log_level level, const char *fmt, ...)
{
    va_list args;
    char buffer[4096];

    va_start(args, fmt);
    vsnprintf(buffer, sizeof(buffer), fmt, args);
    va_end(args);

    // envia o resultado para o rust
    rs_cb(level, buffer);
}

void configure(rs_cb_t rs_cb_log, void *data)
{
    rs_cb = rs_cb_log;
    
    struct retro_log_callback *cb = (struct retro_log_callback *)data;
    cb->log = core_log_cb;
}


void deinit() {
    free(rs_cb);
    rs_cb = NULL;

}