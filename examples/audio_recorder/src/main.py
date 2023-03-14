import math
import _random

from kybra import (
    blob,
    ic,
    InsertError,
    nat64,
    opt,
    Principal,
    query,
    Record,
    StableBTreeMap,
    update,
    Variant,
)


class User(Record):
    id: Principal
    created_at: nat64
    recording_ids: list[Principal]
    username: str


class Recording(Record):
    id: Principal
    audio: blob
    created_at: nat64
    name: str
    user_id: Principal


users = StableBTreeMap[Principal, User](
    memory_id=0, max_key_size=38, max_value_size=100_000
)
recordings = StableBTreeMap[Principal, Recording](
    memory_id=1, max_key_size=38, max_value_size=5_000_000
)


class CreateUserResult(Variant, total=False):
    ok: User
    err: InsertError


@update
def create_user(username: str) -> CreateUserResult:
    id = generate_id()
    user: User = {
        "id": id,
        "created_at": ic.time(),
        "recording_ids": [],
        "username": username,
    }

    result = users.insert(user["id"], user)

    if result.err is not None:
        return {"err": result.err}

    return {"ok": user}


@query
def read_users() -> list[User]:
    return users.values()


@query
def read_user_by_id(id: Principal) -> opt[User]:
    return users.get(id)


class DeleteUserResult(Variant, total=False):
    ok: User
    err: "DeleteUserErr"


class DeleteUserErr(Variant, total=False):
    UserDoesNotExist: Principal


@update
def delete_user(id: Principal) -> DeleteUserResult:
    user = users.get(id)

    if user is None:
        return {"err": {"UserDoesNotExist": id}}

    for recording_id in user["recording_ids"]:
        recordings.remove(recording_id)

    users.remove(user["id"])

    return {"ok": user}


class CreateRecordingResult(Variant, total=False):
    ok: Recording
    err: "CreateRecordingErr"


class CreateRecordingErr(Variant, total=False):
    InsertError: InsertError
    UserDoesNotExist: Principal


@update
def create_recording(
    audio: blob, name: str, user_id: Principal
) -> CreateRecordingResult:
    user = users.get(user_id)

    if user is None:
        return {"err": {"UserDoesNotExist": user_id}}

    id = generate_id()
    recording: Recording = {
        "id": id,
        "audio": audio,
        "created_at": ic.time(),
        "name": name,
        "user_id": user_id,
    }

    create_recording_result = recordings.insert(recording["id"], recording)

    if create_recording_result.err is not None:
        return {"err": {"InsertError": create_recording_result.err}}

    updated_user: User = {
        **user,
        "recording_ids": [*user["recording_ids"], recording["id"]],
    }

    update_user_result = users.insert(updated_user["id"], updated_user)

    if update_user_result.err is not None:
        return {"err": {"InsertError": update_user_result.err}}

    return {"ok": recording}


@query
def read_recordings() -> list[Recording]:
    return recordings.values()


@query
def read_recording_by_id(id: Principal) -> opt[Recording]:
    return recordings.get(id)


class DeleteRecordingResult(Variant, total=False):
    ok: Recording
    err: "DeleteRecordingError"


class DeleteRecordingError(Variant, total=False):
    InsertError: InsertError
    RecordingDoesNotExist: Principal
    UserDoesNotExist: Principal


@update
def delete_recording(id: Principal) -> DeleteRecordingResult:
    recording = recordings.get(id)

    if recording is None:
        return {"err": {"RecordingDoesNotExist": id}}

    user = users.get(recording["user_id"])

    if user is None:
        return {"err": {"UserDoesNotExist": recording["user_id"]}}

    updated_user: User = {
        **user,
        "recording_ids": list(
            filter(
                lambda recording_id: recording_id.to_str() != recording["id"].to_str(),
                user["recording_ids"],
            )
        ),
    }

    update_user_result = users.insert(updated_user["id"], updated_user)

    if update_user_result.err is not None:
        return {"err": {"InsertError": update_user_result.err}}

    recordings.remove(id)

    return {"ok": recording}


def generate_id() -> Principal:
    random_bytes = bytes(
        [math.floor(_random.Random().random() * 256) for _ in range(29)]
    )

    return Principal.from_hex(random_bytes.hex())