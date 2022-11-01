from kybra import ic, inspect_message, update;

@inspect_message
def inspect_message_():
    ic.print('inspect_message called')

    if ic.method_name() == 'accessible':
        ic.accept_message()
        return

    if ic.method_name() == 'inaccessible':
        return

    raise Exception('Method ' + ic.method_name() + ' is not allowed')

@update
def accessible() -> bool:
    return True

@update
def inaccessible() -> bool:
    return False

@update
def alsoInaccessible() -> bool:
    return False
