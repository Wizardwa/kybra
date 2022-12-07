from kybra import blob, ic, nat32, nat64, query, StableGrowResult, Stable64GrowResult, update

@query
def stable_size() -> nat32:
    return ic.stable_size()

@query
def stable64_size() -> nat64:
    return ic.stable64_size()

@update
def stable_grow(new_pages: nat32) -> StableGrowResult:
    return ic.stable_grow(new_pages)

@update
def stable64_grow(new_pages: nat64) -> Stable64GrowResult:
    return ic.stable64_grow(new_pages)

@update
def stable_write(offset: nat32, buf: blob):
    ic.stable_write(offset, buf)
    return True

@update
def stable64_write(offset: nat64, buf: blob) -> bool:
    ic.stable64_write(offset, buf)
    return True

@query
def stable_read(offset: nat32, length: nat32) -> blob:
    return ic.stable_read(offset, length)

@query
def stable64_read(offset: nat64, length: nat64) -> blob:
    return ic.stable64_read(offset, length)

@query
def stable_bytes() -> blob:
    return ic.stable_bytes()
