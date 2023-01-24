from kybra import InsertError, opt, Principal, query, StableBTreeMap, update, Variant
from kybra import nat64


class StableMap13InsertResult(Variant, total=False):
    ok: opt[Principal]
    err: InsertError


stable_map_13 = StableBTreeMap[str, Principal](
    memory_id=13, max_key_size=100, max_value_size=1_000)


@query
def stable_map_13_get(key: str) -> opt[Principal]:
    return stable_map_13.get(key)


@update
def stable_map_13_insert(key: str, value: Principal) -> StableMap13InsertResult:
    result = stable_map_13.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_13_remove(key: str) -> opt[Principal]:
    return stable_map_13.remove(key)


@query
def stable_map_13_contains_key(key: str) -> bool:
    return stable_map_13.contains_key(key)


@query
def stable_map_13_is_empty() -> bool:
    return stable_map_13.is_empty()


@query
def stable_map_13_keys() -> list[str]:
    return stable_map_13.keys()


@query
def stable_map_13_values() -> list[Principal]:
    return stable_map_13.values()


@query
def stable_map_13_items() -> list[tuple[str, Principal]]:
    return stable_map_13.items()


@query
def stable_map_13_len() -> nat64:
    return stable_map_13.len()