rustinr::rustrize()
devtools::install()
library(RNeo4j)
cypher(startGraph("neo4j://localhost:7687/"), "MATCH (n) WHERE n.name={value} RETURN n.name,n.age", value="Lee")
