initSidebarItems({"struct":[["DoRead","Flag for `ReaderPolicy` methods to signal whether or not `BufReader` should read into the buffer."],["FlushAmt","Flag for `WriterPolicy` methods to tell `BufWriter` how many bytes to flush to the underlying reader."],["FlushAtLeast","Flush the buffer if it contains at least the given number of bytes."],["FlushExact","Only ever flush exactly the given number of bytes, until the writer is empty."],["FlushOn","Flush the buffer if it contains the given byte."],["FlushOnNewline","Flush the buffer if it contains a newline (`\\n`)."],["MinBuffered","A policy for `BufReader` which ensures there is at least the given number of bytes in  the buffer, failing this only if the reader is at EOF."],["StdPolicy","Default policy for both `BufReader` and `BufWriter` that reproduces the behaviors of their `std::io` counterparts:"]],"trait":[["ReaderPolicy","Trait that governs `BufReader`’s behavior."],["WriterPolicy","A trait which tells `BufWriter` when to flush."]]});