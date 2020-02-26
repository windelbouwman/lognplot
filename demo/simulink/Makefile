
SIMULINK_FOLDER=SimulinkDemo/SimulinkDemo_grt_rtw
CFLAGS=-I../c -L../../target/release
CFLAGS+=-IR2018a/rtw/c/src -IR2018a/simulink/include -IR2018a/extern/include
CFLAGS+=-I${SIMULINK_FOLDER}
SIMULINK_SRC=${SIMULINK_FOLDER}/SimulinkDemo_capi.c \
 ${SIMULINK_FOLDER}/SimulinkDemo_data.c \
 ${SIMULINK_FOLDER}/SimulinkDemo.c \
 ${SIMULINK_FOLDER}/rt_zcfcn.c \
 ${SIMULINK_FOLDER}/rtGetNaN.c \
 ${SIMULINK_FOLDER}/rt_nonfinite.c \
 ${SIMULINK_FOLDER}/rtGetInf.c

MATLAB_SRC=R2018a/rtw/c/src/rt_logging.c
LIBS=-lclognplot -lpthread -lm -ldl

demo: bridge.c ${SIMULINK_SRC} ${MATLAB_SRC}
	gcc ${CFLAGS} bridge.c ${SIMULINK_SRC} ${MATLAB_SRC} -o demo ${LIBS}
