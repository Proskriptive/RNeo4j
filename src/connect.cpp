#include <Rcpp.h>
#include <neo4j-client.h>
#include <errno.h>
#include <string>

#include "utils.h"

using namespace std::literals::string_literals;
using namespace Rcpp;

// [[Rcpp::export]]
void initNeo4j() {
    neo4j_client_init();
}

// [[Rcpp::export]]
void cleanupNeo4j() {
    neo4j_client_cleanup();
}

// [[Rcpp::export]]
SEXP startGraph(const StringVector &url, const StringVector &username="neo4j", const StringVector &password="neo4j") {
    neo4j_config_t* conf = neo4j_new_config();
    if (neo4j_config_set_username(conf, as<const char*>(username)) != 0) {
        stop("Failed to set username (errno %d)", errno);
    }
    if (neo4j_config_set_password(conf, as<const char*>(password)) != 0) {
        stop("Failed to set password (errno %d)", errno);
    }
    neo4j_connection_t* conn = neo4j_connect(as<const char*>(url), conf, NEO4J_INSECURE);
    neo4j_config_free(conf);
    if (conn == NULL) {
        stop("Connection failed (errno %d)", errno);
    }
    XPtr<Graph> ptr(new Graph(conn), true);

    return ptr;
}