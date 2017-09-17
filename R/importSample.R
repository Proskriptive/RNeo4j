importSample <- function(graph, data, input = TRUE) {
  clear(graph, input)
  fpath = system.file("sampleData", paste(data, ".txt", sep = ""), package = "RNeo4j")
  query = readChar(fpath, file.info(fpath)$size)
  suppressMessages(cypher(graph, query))
}
