from kybra import (
    Async,
    CanisterResult,
    match,
    nat64,
    NotifyResult,
    opt,
    Principal,
    update,
    Variant,
)
from src.canister2.types import Account, AccountArgs, Canister2


class TransferResult(Variant, total=False):
    Ok: nat64
    Err: str


class BalanceResult(Variant, total=False):
    Ok: nat64
    Err: str


class AccountResult(Variant, total=False):
    Ok: opt[Account]
    Err: str


class AccountsResult(Variant, total=False):
    Ok: list[Account]
    Err: str


class TrapResult(Variant, total=False):
    Ok: str
    Err: str


canister2 = Canister2(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))


@update
def transfer(from_: str, to: str, amount: nat64) -> Async[TransferResult]:
    result: CanisterResult[nat64] = yield canister2.transfer(from_, to, amount)

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def balance(id: str) -> Async[BalanceResult]:
    result: CanisterResult[nat64] = yield canister2.balance(id)

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def account(args: AccountArgs) -> Async[AccountResult]:
    result: CanisterResult[opt[Account]] = yield canister2.account(args)

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def accounts() -> Async[AccountsResult]:
    result: CanisterResult[list[Account]] = yield canister2.accounts()

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def trap() -> Async[TrapResult]:
    result: CanisterResult[str] = yield canister2.trap()

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def send_notification() -> NotifyResult:
    return canister2.receive_notification("This is the notification").notify()
