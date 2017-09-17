# From RNeo4j
check_nested_depth = function(col) {
  max(unlist(sapply(col, function(x) {sapply(x, length)})))
}
