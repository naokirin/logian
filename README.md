# Logian

[![Build Status](https://travis-ci.org/naokirin/logian.svg?branch=master)](https://travis-ci.org/naokirin/logian) [![Build status](https://ci.appveyor.com/api/projects/status/aspww5vy8oyb5vo9/branch/master?svg=true)](https://ci.appveyor.com/project/naokirin/logian/branch/master)

Logian is a log format code generator (implemented in rust).  

Releases, see [Release page](https://github.com/naokirin/logian/releases).

## Commands

`logian` command generates files:

* log schema
* default log schema
* user-defined type
* log output code

```
# Note: a type name with `?` means nullable.

# Generate log schema directory
$ logian init --log-label=log_type --schema_dir=./schemas

# Generate a log schema.
$ logian generate log log_name column1:string column2:integer? --schema_dir=./schemas

# Generate a default log schema.
# A default log schema adds column to front and back for all logs.
$ logian generate default-log --front "front_column1:string, front_column2:integer"  --back "back_column1:string, front_column2:integer" --schema_dir=./schemas

# Generate a user-defined type.
$ logian generate type type_name column1:string column2:integer --schema_dir=./schemas

# Generate rust code.
$ logian output rust ./output_dir --schema-dir=./schemas
```

For details about options, see `logian --help` command.

## Types

Logian supports primitive types, user-defined types and type attributes.

### Primitive Types

* string
* integer
* float
* boolean
* datetime
* timestamp

### Type Attributes

* nullable

### User-defined types

Limitation: a user-defined type doesn't includes a user-defined type.

## Template plugins

### Default plugins

* rust

### Custom plugins

* TBA

