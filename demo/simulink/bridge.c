
#include <stdio.h>
#include "lognplot.h"

// Include your model here:

#include "SimulinkDemo.h"
#include "SimulinkDemo_capi.h"

/* Given a model mapping info struct, sample all signals and forward them to lognplot.
*/
static void log_signals(lognplot_client_t* client, double t, const rtwCAPI_ModelMappingInfo* mmi) {
    const rtwCAPI_Signals *signals = rtwCAPI_GetSignals(mmi);
    const rtwCAPI_DataTypeMap* data_types = rtwCAPI_GetDataTypeMap(mmi);
    void** data_address_map = rtwCAPI_GetDataAddressMap(mmi);

    for (int i = 0; i < rtwCAPI_GetNumSignals(mmi); i++) {
        char full_signal_path[200];
        snprintf(full_signal_path, 200, "%s/%s", signals[i].blockPath, signals[i].signalName);
        const rtwCAPI_DataTypeMap* data_type = &data_types[signals[i].dataTypeIndex];
        const void* address = data_address_map[signals[i].addrMapIndex];
        // printf("t=%f, Signal: %s @ %X %s\n", t, data_type->cDataName, address, full_signal_path);

        switch (data_type->slDataId) {
            case SS_DOUBLE:
                {
                    const double* value_ptr = address;
                    double value = *value_ptr;
                    // printf("Value=%f\n", value);
                    lognplot_client_send_sample(client, full_signal_path, t, value);
                }
                break;
        }
    }
}

void main() {
    lognplot_client_t* client = lognplot_client_new("127.0.0.1:12345");

    if (client) {
        printf("Connected!\n");
        double t = 0;
        const double dt = 0.001;  // 1kHz ?
        SimulinkDemo_initialize();
        while (1) {
            SimulinkDemo_step();
            log_signals(client, t, &SimulinkDemo_M->DataMapInfo.mmi);
            t += dt;
        }
    } else {
        printf("lognplot Connection failed.\n");
    }
}
