cypher <- function(graph, query, ...) {
    args = list(...)
    cypherInternal(graph, query, args, names(args))
}