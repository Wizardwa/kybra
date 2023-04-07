from kybra import (
    InsertError,
    match,
    opt,
    query,
    Record,
    StableBTreeMap,
    Tuple,
    update,
    Variant,
    Vec,
)
from kybra import nat64
from kybra import float32


class BlogPost(Record):
    title: str


class User(Record):
    username: str
    posts: Vec["BlogPost"]


class StableMap4InsertResult(Variant, total=False):
    Ok: opt[float32]
    Err: InsertError


stable_map4 = StableBTreeMap[User, float32](
    memory_id=4, max_key_size=100, max_value_size=1_000
)


@query
def stable_map4_get(key: User) -> opt[float32]:
    return stable_map4.get(key)


@update
def stable_map4_insert(key: User, value: float32) -> StableMap4InsertResult:
    result = stable_map4.insert(key, value)

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


@update
def stable_map4_remove(key: User) -> opt[float32]:
    return stable_map4.remove(key)


@query
def stable_map4_contains_key(key: User) -> bool:
    return stable_map4.contains_key(key)


@query
def stable_map4_is_empty() -> bool:
    return stable_map4.is_empty()


@query
def stable_map4_keys() -> Vec[User]:
    return stable_map4.keys()


@query
def stable_map4_values() -> Vec[float32]:
    return stable_map4.values()


@query
def stable_map4_items() -> Vec[Tuple[User, float32]]:
    return stable_map4.items()


@query
def stable_map4_len() -> nat64:
    return stable_map4.len()
