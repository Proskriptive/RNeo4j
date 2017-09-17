# From RNeo4j
check_nested_depth <- function(col) {
    max(unlist(sapply(col, function(x) {sapply(x, length)})))
}

parseParams <- function(...) {
    params = list(...)
    if (length(params) == 0) {
        params = NULL
    } else if (length(params) == 1 && is.list(params[[1]])) {
        params = params[[1]]
    }
    params
}

clear <- function(graph, input = TRUE) {
    # TODO if input confirm clear with user
    cypher(graph, "MATCH (n) DETACH DELETE n")
}
