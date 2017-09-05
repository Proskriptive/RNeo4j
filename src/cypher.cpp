#include <Rcpp.h>
#include <neo4j-client.h>
#include <errno.h>
#include <string>
#include <vector>

#include "utils.h"

using namespace std::literals::string_literals;
using namespace Rcpp;

// [[Rcpp::export]]
DataFrame cypherInternal(SEXP graphSexp, const char* query, List args, StringVector names) {
    XPtr<Graph> graph(graphSexp);
    Vector<STRSXP>::iterator it = names.begin();
    std::vector<neo4j_map_entry_t> neo4jArgs;
    int i = 0;
    while (it != names.end()) {
        const char* key = as<const char*>(*it);
        // TODO there should be a way to get the value by key, not index
        SEXP valueSexp = args[i];
        neo4j_value_t value;
        int type = TYPEOF(valueSexp);
        if (type == STRSXP) {
            value = neo4j_string(as<const char*>(valueSexp));
        } else if (type == INTSXP) {
            value = neo4j_int(as<int>(valueSexp));
        } else if (type == REALSXP) {
            value = neo4j_float(as<float>(valueSexp));
        } else if (type == LGLSXP) {
            value = neo4j_bool(as<bool>(valueSexp));
        } else {
            // TODO print out the name of the type, not the number
            stop("Unsupported parameter type: %i", type);
        }
        neo4jArgs.push_back(neo4j_map_entry(key, value));
        i++;
        it++;
    }
    neo4j_value_t params = neo4j_map(neo4jArgs.data(), neo4jArgs.size());
    neo4j_result_stream_t* resultStream = neo4j_run(&**graph, query, params);
    CleanupWithErr<neo4j_result_stream_t*> resultStreamCleaner(resultStream, neo4j_close_results);
    std::vector<neo4j_result_t*> resultVector;
    neo4j_result_t* nextResult;
    while ((nextResult = neo4j_fetch_next(resultStream)) != NULL) {
        neo4j_retain(nextResult);
        resultVector.push_back(nextResult);
    }
    int neo4jErr = neo4j_check_failure(resultStream);
    if (neo4jErr != 0) {
        for (size_t i = 0; i < resultVector.size(); i++) {
            neo4j_release(resultVector[i]);
        }
        if (neo4jErr == NEO4J_STATEMENT_EVALUATION_FAILED) {
            stop("Error fetching results: %s", neo4j_error_message(resultStream));
        } else {
            stop("Error fetching results: %i", neo4jErr);
        }
    }
    unsigned int nFields = neo4j_nfields(resultStream);
    size_t nResults = resultVector.size();
    List resultList(nResults);
    for (size_t r = 0; r < nResults; r++) {
        neo4j_result_t* neo4jResult = resultVector[r];
        List resultRow(nFields);
        for (unsigned int c = 0; c < nFields; c++) {
            neo4j_value_t item = neo4j_result_field(neo4jResult, c);
            neo4j_type_t type = neo4j_type(item);
            if (type == NEO4J_STRING) {
                unsigned int len = neo4j_string_length(item) + 1;
                char* buf = (char*) malloc(len);
                neo4j_string_value(item, buf, len);
                resultRow[c] = std::string(buf);
                // std::string copies
                free(buf);
            } else if (type == NEO4J_INT) {
                resultRow[c] = neo4j_int_value(item);
            } else if (type == NEO4J_FLOAT) {
                resultRow[c] = neo4j_float_value(item);
            } else if (type == NEO4J_BOOL) {
                resultRow[c] = neo4j_bool_value(item);
            } else {
                for (; r < nResults; r++) {
                    neo4j_release(resultVector[r]);
                }
                stop("Returned unsupported Neo4J type: %s", neo4j_typestr(type));
            }
        }
        neo4j_release(neo4jResult);
        resultList[r] = resultRow;
    }
    DataFrame resultDF(resultList);
    if (nResults > 0) {
        CharacterVector fieldNamesRVec(nFields);
        for (unsigned int i = 0; i < nFields; i++) {
            const char* name = neo4j_fieldname(resultStream, i);
            fieldNamesRVec[i] = name;
        }
        resultDF.attr("names") = fieldNamesRVec;
    }
    return resultDF;
}
