startGraph <- function(uri, username = "", password = "") {
    open_graph_internal(uri, username, password)
}

cypher <- function(graph, query, ...) {
    params = list(...)
    if (length(params) == 0) {
        params = NULL
    } else if (length(params) == 1 && is.list(params[[1]])) {
        params = params[[1]]
    }
    df = query_graph_internal(graph, query, params)
    if (!is.data.frame(df)) {
        stop("You must query for tabular results when using this function.")
    }
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