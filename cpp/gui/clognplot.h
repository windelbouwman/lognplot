#ifndef CLOGNPLOT_H
#define CLOGNPLOT_H

#ifdef __cplusplus
extern "C" {
#endif

// TSDB API:
typedef int TsDbHandle;
TsDbHandle* lognplot_tsdb_new();
void lognplot_tsdb_add_sample(TsDbHandle* db);
void lognplot_tsdb_query(TsDbHandle* db);

#ifdef __cplusplus
}
#endif

#endif
