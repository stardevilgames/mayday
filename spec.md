# Meide Specification

Version 1.0 of the Meide Client and Server Specification.

## Server Specification

Meide tries to be as fast and low-level as possible, as well as reliable.
Processes, or clients connected to Servers, generally handle requests from other processes.

Meide holds data of a Generic Ruby (we'll call it GenRuby for now,) which are processes themselves
managed by the Server, and handled by a Client.
Clients and the Server are separated in good cause.

Servers are a sort of middle and front end. They act as a frontend for Clients to send messages to processes. The difference between a GenRuby and a Client is that a GenRuby holds state but has no functionality while a Client holds *zero* state but has all the functionality.

While Clients may update attributes or fields on a GenRuby one at a time, this is highly discouraged
as it could be a massive slowdown. Instead use the `SAVE` opcode, which can be done like so on a client:

```json
{
    "op": 2,
    "pid": "...",
    "state": { ... }
}
```

* `state` should only contain updates. If a field in the State isn't updated,
don't update it
* To mark a field deleted, in the state set the name of the field to the value `"$M0"` meaning Meide Delete.
* Fields may not start with a `$` since it symbolizes a special Meide function.

Clients can subscribe to the data updates of a GenRuby. Although this should not be done
as GenRuby's can receive an infinite amount of messages per second.


### Process IDs

Process IDs are assigned to both Clients and GenRubys.
They are encoded into BASE64 and structured like so decoded:

```
{primary_key}:{unique_ulid}:{machine_id}
```

* `primary_key` is `None` for Client Process IDs.
* `machine_id` May be removed in the future to allow for GenRuby state replication.


### Op Codes

- 0: Hello code

    Sent as the first message after connection.

    **Message:**
    ```json
    {
        "op": 0
    }
    ```

- 1: CREATE (GenRuby-only)

    **Request:**
    ```json
    {
        "op": 1,
        "index": "my_index",
        "state": { ... }
    }
    ```
    **Success Return:**
    ```json
    {
        "op": 4,
        "pid": "..."
    }
    ```

- 2: SAVE (GenRuby-only)

    **Request:**
    ```json
    {
        "op": 2,
        "pid": "...",
        "state": { ... }
    }
    ```
    **Success Return:**
    ```json
    {
        "op": 4
    }
    ```

- 3: DELETE (GenRuby-only)

    **Request:**
    ```json
    {
        "op": 3,
        "pid": "..."
    }
    ```
    **Success Return:**
    ```json
    {
        "op": 4
    }
    ```

- 4: Success

- 5: Failure

    Failure messages contain a string field called "error" which is a best attempt
    to determine the error if it's an internal error from the server, or from a client
    error.

- 6: INDEX UPDATE (Client-only)

    Update which indexes this Client will handle requests for.

    **Request:**
    ```json
    {
        "op": 6,
        "indexes": [ "...", "..." ]
    }
    ```
    **Success Return:**
    ```json
    {
        "op": 4,
    }
    ```

- 7: SEND

    Send a message to a client. This message must be a map.

    **Request:**
    ```json
    {
        "op": 7,
        "pid": "..."
    }
    ```
    **Success Return:**
    ```json
    {
        "op": 4,
    }
    ```

- 8: RECEIVE

    A message sent from another Process.

    **Message:**
    ```json
    {
        "op": 8,
        "origin": "..."
    }
    ```

    * `origin` is a `pid`.


### GenRuby Sharding

GenRuby's are not infinitely scalable within a single server. GenRuby's may need to be sharded,
and this can be done. GenRuby's state must stay on a single server (this may change to support replication,) which should be the server on which the GenRuby was created.

In this sense, GenRuby's are automatically sharded if a client (based on least-recently-used) on a server supports the GenRuby index.
The Server will try to reach out to other servers (from least to most used) if the server cannot find
a suitable client on itself. If the Server still cannot find the GenRuby, it should give a generic "not found" error to indicate so.
