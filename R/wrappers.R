startGraph <- function(uri, username = "", password = "") {
    open_graph_internal(uri, username, password)
}

cypher <- function(graph, query, ...) {
    query_graph_internal(graph, query, list(...))
}
