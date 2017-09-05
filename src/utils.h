#include <neo4j-client.h>

class Graph {
private:
    neo4j_connection_t* ptr;
public:
    Graph(neo4j_connection_t* ptr) : ptr(ptr) {}

    ~Graph() {
        neo4j_close(ptr);
    }

    neo4j_connection_t& operator*() {
        return *ptr;
    }

    neo4j_connection_t* operator->() {
        return ptr;
    }
};

template <class T> class CleanupWithErr {
private:
    typedef int (*cleanup_t)(T);

    T value;
    cleanup_t cleanup;
public:
    CleanupWithErr(T value, cleanup_t cleanup) : value(value), cleanup(cleanup) {}

    CleanupWithErr(const CleanupWithErr& other) : value(other.value) {}

    ~CleanupWithErr() {
        if (cleanup(value) != 0) {
            Rcpp::stop("Error when freeing: %i", errno);
        }
    }

    CleanupWithErr& operator=(const CleanupWithErr& other) {
        if (&other != this) {
            this.value = other.value;
        }
        return *this;
    }
};