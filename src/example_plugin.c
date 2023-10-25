#include <stdbool.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>
#include <device_connector.h>

#define N_ELEMENT 1
#define PLUGIN_NAME "example-plugin"

typedef struct {
    const char* text;
} ExamplePlugin;

void *example_plugin_new(const char *config);
DcElementResult example_plugin_next(void* element, DcPipeline *pipeline, DcMsgReceiver *msg_receiver);
bool example_plugin_finalizer(void *element, struct DcFinalizer *finalizer);
void example_plugin_free(void* element);

bool dc_load(DcPlugin *plugin) {
    dc_init(PLUGIN_NAME);

    plugin->version = "0.1.0";
    plugin->n_element = N_ELEMENT;

    DcElement *elements = (DcElement *)malloc(sizeof(DcElement) * N_ELEMENT);

    // Element settings
    elements[0].name = PLUGIN_NAME;
    elements[0].recv_ports = 0;
    elements[0].send_ports = 1;
    elements[0].acceptable_msg_types = NULL;
    elements[0].config_format = "json";
    elements[0].new_ = example_plugin_new;
    elements[0].next = example_plugin_next;
    elements[0].finalizer = example_plugin_finalizer;
    elements[0].free = example_plugin_free;

    plugin->elements = elements;
    return true;
}

void *example_plugin_new(const char *config) {
    ExamplePlugin *example_plugin = (ExamplePlugin *)malloc(sizeof(ExamplePlugin));
    example_plugin->text = "hello, world from plugin";
    return example_plugin;
}

DcElementResult example_plugin_next(void* element, DcPipeline *pipeline, DcMsgReceiver *msg_receiver) {
    ExamplePlugin *example_plugin = (ExamplePlugin *)element;

    if (!dc_pipeline_send_msg_type_checked(pipeline)) {
     DcMsgType msg_type;
     if (dc_msg_type_new("mime:text/plain", &msg_type)) {
         dc_pipeline_check_send_msg_type(pipeline, 0, msg_type);
     }
    }

    sleep(1);

    DcMsgBuf *msg_buf = dc_pipeline_msg_buf(pipeline);

    const uint8_t *data = (const uint8_t *)example_plugin->text;
    const size_t len = strlen(example_plugin->text);
    dc_msg_buf_write(msg_buf, data, len);

    return DcElementResult_MsgBuf;
}

bool example_plugin_finalizer(void *element, struct DcFinalizer *finalizer) {
    return true;
}

void example_plugin_free(void* element) {
    ExamplePlugin *example_plugin = (ExamplePlugin *)element;
    free(example_plugin);
}