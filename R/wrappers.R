startGraph <- function(uri, username = "", password = "") {
    open_graph_internal(uri, username, password)
}

cypher <- function(graph, query, ...) {
    params = parseParams(...)

    df = query_graph_internal(graph, query, params, TRUE)
    if (nrow(df) == 0) {
        return(df)
    }

    is.na(df) <- df == "NULL"

    # From RNeo4j
    # Converts list(item) into item
    if(all(sapply(df, class) == "list")) {
        for(i in 1:ncol(df)) {
            if(check_nested_depth(df[i]) == 1) {
                df[i] = unlist(df[i])
            } 
        }
    }

    row.names(df) = NULL
    df
}

cypherToList <- function (graph, query, ...) {
    params = parseParams(...)

    list = query_graph_internal(graph, query, params, FALSE)
    list
}
