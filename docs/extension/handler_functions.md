# Handler functions

Handler functions are the building blocks of Creo.
They are small, web-framework-agnostic functions that implement a self-contained piece of business logic.
We use handler functions to assign computation cost to generated endpoints.
That is, we parse the expected function inputs from the incoming network request and call the handler function with
this input. The endpoint then responds with the return value of the handler function.

The following provides a guide on how to extend Creo with new handler functions.
Please note that this guide assumes that the programming language of the handler function is already supported.
If you want to add a new programming language to Creo, please follow the instructions [here](./programming_language.md).
The implementation of a handler function comprises three parts:

1. The implementation source code of the function
2. A `definition` file providing meta-data about the handler function, such as the signature
3. A `utilization` file specifying the labels of the handler function

## Handler Function Implementation

A handler function may implement arbitrary, self-contained business logic.
The function signature, i.e., the function arguments and return value, must adhere to the following constraints.
We will describe these constraints in detail with the following examples.

### Handler Functions with Primitive Arguments

In the simplest case, a handler function may take any number of primitive-typed function arguments.
A primitive type is one of the following JSON types:

- `string`
- `number`
- `integer`
- `boolean`

For instance consider the following `go` function, that selects a random number inside a given range.

```go
func RandomNumberFromRange(min int64, max int64) int64 {
    // Implementation details here
    ...
}
```

As we can see from the function signature, the handler function takes two 64-bit integer values `min` and `max` and
returns a single 64-bit integer value. A handler function may have any number and combination of primitive-typed
function arguments.

### Handler Functions with Complex Arguments

Now consider, that we want the above handler function to take a `MinMax` object as a single argument to ensure that
the order of the primitive arguments is not reversed accidentally.

```go
type MinMax struct {
    Min int64
    Max int64
}
```

The following JSON types are considered complex:

- `object`
- `array`

In our framework, a handler function cannot accept complex-typed function arguments directly.
Instead, the function must accept a byte string representing the complex-typed argument as JSON
(_Note_: According to [RFC8259 Section 8.1](https://www.rfc-editor.org/rfc/rfc8259#section-8.1) it is safe
to assume the byte string of a JSON payload is valid UTF-8, in the case you need to decode the byte string
into a regular string). Given this byte string, the handler function must first decode the complex-typed
function argument. To illustrate this, our changed example may look like this:

```go
import "encoding/json"

type MinMax struct {
    Min int64
    Max int64
}

func RandomNumberFromRange(jsonData []byte) int64 {
    var minMax MinMax
    if err := json.Unmarshal(json_data, &minMax); err != nil {
        panic(err)
    }
    // Implementation details here
    ...
}
```

The implementation of a handler function may expect at most **one** complex-typed function argument.
However, the function may take any number of additional primitive-typed arguments.
For example:

```go
import "encoding/json"

type MinMax struct {
    Min int64
    Max int64
}

// `count` specifies the number of random numbers to draw
func RandomNumbersFromRange(jsonData []byte, count int64) []int64 {
    var minMax MinMax
    if err := json.Unmarshal(json_data, &minMax); err != nil {
        panic(err)
    }
    // Implementation details here
    ...
}
```

In the case a function requires multiple complex-typed arguments, the implementation should aggregate the arguments
into a single top-level object comprising the function argument as properties. Consider the following example:

```go
import "encoding/json"

type MinMax struct {
    Min int64
    Max int64
}

type MinMaxOpts struct {
    IsInclusiveMin bool
    IsInclusiveMax bool
}

type RandomArgs struct {
    MinMax MinMax
    Opts   MinMaxOpts
}

func RandomNumberFromRange(jsonData []byte) []int64 {
    var args RandomArgs
    if err := json.Unmarshal(json_data, &args); err != nil {
        panic(err)
    }
    minMax := args.MinMax
    opts := args.Opts
    // Implementation details here
    ...
}
```

This example illustrates that handler functions may still require multiple complex-typed arguments by wrapping the
arguments into a single, top-level object. With this technique, complex arguments may also be arbitrarily nested to
represent larger, more complex data structures.

## `definition` Configuration File

To integrate a handler function into Creo, the framework requires a `definition` file containing important meta-data.
The file may be in JSON or YAML, however, we will use the YAML format in the following example.

```yaml
import_path: github.com/DescartesResearch/minmax
is_async: false
returns: true
signature:
  function: RandomNumbersFromRange
  parameters:
    - type: object
      arg: 0
      properties:
        min:
          type: integer
          format: int64
        max:
          type: integer
          format: int64
      additionalProperties: false
      required:
        - min
        - max
    - type: integer
      arg: 1
      format: int64
```

The above `definition` corresponds to the example handler function from above that generates a given number of
random numbers between a specified minimum and maximum value.
The following describes each key in more detail:

- `import_path`: specifies the import path of the module/file from which the handler function can be imported.
- `is_async`: flag that indicates whether the handler function is async. This ensures that the result of asynchronous
  functions is properly _awaited_.
- `returns`: flag that indicates whether the handler function returns a value.
- `signature`: specifies the signature of the handler function.

Additionally, the following optional keys may be specified:

- `description`: A short description of the handler function
- `depends_on`: Specifies any external dependencies (e.g., databases) the handler function may depend on. To learn more
  about dependencies, please refer to our [dependency](./dependency.md) guide.

### Signature Definition

The signature definition of a handler function requires the following keys:

- `function`: specifies the function name.
- `parameters`: defines the list of expected function arguments.

### Parameter Definition

The function parameter definition requires the following keys:

- `arg`: specifies the argument position in the function signature. The position is 0-index, i.e. the first argument is
  at position 0. If the targeted programming language supports keyword arguments (e.g. Python), the value may also be a
  string containing the name of the argument.

Any additional keys may be any valid JSON Schema Type keywords. The following shortly explains the most common keys:

- `type`: specifies the argument type, e.g. `string`, `integer`, or `object`.
- `format`: specifies additional format information for the `type` keyword, e.g. `int64` or `int32` for `integer`
  arguments

Please refer to the [JSON Schema Reference](https://json-schema.org/understanding-json-schema/reference/type) for
type specific keywords.

## `utilization` File

The `utilization` file of a handler function specifies the values of the handler functions labels.
Currently, Creo supports labels describing the resource usage of the handler function.
For instructions on how to acquire these labels, please refer to our [profiling](./profiling.md) guide.
