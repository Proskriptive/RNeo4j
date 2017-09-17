importSample <- function(graph, data, input = TRUE) {
  # TODO if input confirm clear with user
  cypher(graph, "MATCH (n) DETACH DELETE n")
  fpath = system.file("sampleData", paste(data, ".txt", sep = ""), package = "RNeo4j")
  query = readChar(fpath, file.info(fpath)$size)
  suppressMessages(cypher(graph, query))
}
