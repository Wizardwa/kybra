from kybra import (
    Async,
    Variant,
    blob,
    CanisterResult,
    ic,
    Record,
    update,
)
from kybra.canisters.management import management_canister, EcdsaPublicKeyResult, SignWithEcdsaResult


class PublicKey(Record):
    public_key: blob


class Signature(Record):
    signature: blob


class PublicKeyResult(Variant, total=False):
    ok: PublicKey
    err: str


class SignResult(Variant, total=False):
    ok: Signature
    err: str


@update
def public_key() -> Async[PublicKeyResult]:
    caller = ic.caller().bytes
    canister_result: CanisterResult[EcdsaPublicKeyResult] = yield management_canister.ecdsa_public_key(
        {
            'canister_id': None,
            'derivation_path': [caller],
            'key_id': {'curve': {'secp256k1': None}, 'name': 'dfx_test_key'}
        }
    )

    if canister_result.err is not None:
        return {'err': canister_result.err}

    return {'ok': canister_result.ok}


@update
def sign(message_hash: blob) -> Async[SignResult]:
    if len(message_hash) != 32:
        ic.trap('message_hash must be 32 bytes')

    caller = ic.caller().bytes

    canister_result: CanisterResult[SignWithEcdsaResult] = yield management_canister.sign_with_ecdsa({
        'message_hash': message_hash,
        'derivation_path': [caller],
        'key_id': {'curve': {'secp256k1': None}, 'name': 'dfx_test_key'}
    }).with_cycles(10_000_000_000)

    if canister_result.err is not None:
        return {'err': canister_result.err}

    return {'ok': canister_result.ok}