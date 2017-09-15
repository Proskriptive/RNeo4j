startGraph <- function(uri, username = "", password = "") {
    open_graph_internal(uri, username, password)
}

cypher <- function(graph, query, ...) {
    params = list(...)
    if (length(params) == 0) {
        params = NULL
    }
    query_graph_internal(graph, query, params)
}
