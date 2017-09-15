library(RNeo4j)

graph = startGraph("neo4j://localhost:7687/")

test_that("cypher doesn't error", {
    cypher(graph, "MATCH (n) DETACH DELETE n")
    cypher(graph, "CREATE (n:Color {name:{value},rgb:0xff0000})", value="red")
    cypher(graph, "CREATE (n:Color {name:{name},rgb:{rgb}})", name="blue", rgb=0x0000ff)
})

test_that("cypher returns results correctly", {
    ret = cypher(graph, "MATCH (n: Color) WHERE n.name={value} RETURN n.name,n.rgb", value="blue")
    expect_equal(colnames(ret), c("n.name", "n.rgb"))
    expect_equal(nrow(ret), 1)
    expect_equal(ncol(ret), 2)
    expect_equal(ret[[1, 1]], "blue")
    expect_equal(ret[[1, 2]], 0x0000ff)
})
