from kybra import (
    InsertError,
    match,
    nat64,
    nat8,
    Opt,
    query,
    StableBTreeMap,
    Tuple,
    update,
    Variant,
    Vec,
)


class StableMap0InsertResult(Variant, total=False):
    Ok: Opt[str]
    Err: InsertError


stable_map0 = StableBTreeMap[nat8, str](
    memory_id=0, max_key_size=100, max_value_size=100
)


@query
def stable_map0_get(key: nat8) -> Opt[str]:
    return stable_map0.get(key)


@update
def stable_map0_insert(key: nat8, value: str) -> StableMap0InsertResult:
    result = stable_map0.insert(key, value)

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


@update
def stable_map0_remove(key: nat8) -> Opt[str]:
    return stable_map0.remove(key)


@query
def stable_map0_contains_key(key: nat8) -> bool:
    return stable_map0.contains_key(key)


@query
def stable_map0_is_empty() -> bool:
    return stable_map0.is_empty()


@query
def stable_map0_keys() -> Vec[nat8]:
    return stable_map0.keys()


@query
def stable_map0_values() -> Vec[str]:
    return stable_map0.values()


@query
def stable_map0_items() -> Vec[Tuple[nat8, str]]:
    return stable_map0.items()


@query
def stable_map0_len() -> nat64:
    return stable_map0.len()
