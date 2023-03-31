from kybra import InsertError, match, opt, query, StableBTreeMap, update, Variant
from kybra import nat64
from kybra import float64


class StableMap9InsertResult(Variant, total=False):
    Ok: opt[list[str]]
    Err: InsertError


stable_map9 = StableBTreeMap[float64, list[str]](
    memory_id=9, max_key_size=100, max_value_size=1_000
)


@query
def stable_map9_get(key: float64) -> opt[list[str]]:
    return stable_map9.get(key)


@update
def stable_map9_insert(key: float64, value: list[str]) -> StableMap9InsertResult:
    result = stable_map9.insert(key, value)

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


@update
def stable_map9_remove(key: float64) -> opt[list[str]]:
    return stable_map9.remove(key)


@query
def stable_map9_contains_key(key: float64) -> bool:
    return stable_map9.contains_key(key)


@query
def stable_map9_is_empty() -> bool:
    return stable_map9.is_empty()


@query
def stable_map9_keys() -> list[float64]:
    return stable_map9.keys()


@query
def stable_map9_values() -> list[list[str]]:
    return stable_map9.values()


@query
def stable_map9_items() -> list[tuple[float64, list[str]]]:
    return stable_map9.items()


@query
def stable_map9_len() -> nat64:
    return stable_map9.len()
