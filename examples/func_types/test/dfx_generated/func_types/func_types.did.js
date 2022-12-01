export const idlFactory = ({ IDL }) => {
    const ComplexFunc = IDL.Rec();
    ComplexFunc.fill(
        IDL.Func(
            [
                IDL.Record({
                    id: IDL.Text,
                    basic_func: IDL.Func([IDL.Text], [IDL.Text], ['query']),
                    complex_func: ComplexFunc
                }),
                IDL.Variant({
                    Bad: IDL.Null,
                    ComplexFunc: ComplexFunc,
                    Good: IDL.Null,
                    BasicFunc: IDL.Func([IDL.Text], [IDL.Text], ['query'])
                })
            ],
            [IDL.Nat64],
            []
        )
    );
    const GetNotifierFromNotifiersCanisterResult = IDL.Variant({
        ok: IDL.Func([IDL.Vec(IDL.Nat8)], [], ['oneway']),
        err: IDL.Text
    });
    return IDL.Service({
        basic_func_param: IDL.Func(
            [IDL.Func([IDL.Text], [IDL.Text], ['query'])],
            [IDL.Func([IDL.Text], [IDL.Text], ['query'])],
            ['query']
        ),
        basic_func_param_array: IDL.Func(
            [IDL.Vec(IDL.Func([IDL.Text], [IDL.Text], ['query']))],
            [IDL.Vec(IDL.Func([IDL.Text], [IDL.Text], ['query']))],
            ['query']
        ),
        basic_func_return_type: IDL.Func(
            [],
            [IDL.Func([IDL.Text], [IDL.Text], ['query'])],
            ['query']
        ),
        basic_func_return_type_array: IDL.Func(
            [],
            [IDL.Vec(IDL.Func([IDL.Text], [IDL.Text], ['query']))],
            ['query']
        ),
        complex_func_param: IDL.Func(
            [
                IDL.Func(
                    [
                        IDL.Record({
                            id: IDL.Text,
                            basic_func: IDL.Func(
                                [IDL.Text],
                                [IDL.Text],
                                ['query']
                            ),
                            complex_func: ComplexFunc
                        }),
                        IDL.Variant({
                            Bad: IDL.Null,
                            ComplexFunc: ComplexFunc,
                            Good: IDL.Null,
                            BasicFunc: IDL.Func(
                                [IDL.Text],
                                [IDL.Text],
                                ['query']
                            )
                        })
                    ],
                    [IDL.Nat64],
                    []
                )
            ],
            [
                IDL.Func(
                    [
                        IDL.Record({
                            id: IDL.Text,
                            basic_func: IDL.Func(
                                [IDL.Text],
                                [IDL.Text],
                                ['query']
                            ),
                            complex_func: ComplexFunc
                        }),
                        IDL.Variant({
                            Bad: IDL.Null,
                            ComplexFunc: ComplexFunc,
                            Good: IDL.Null,
                            BasicFunc: IDL.Func(
                                [IDL.Text],
                                [IDL.Text],
                                ['query']
                            )
                        })
                    ],
                    [IDL.Nat64],
                    []
                )
            ],
            ['query']
        ),
        complex_func_return_type: IDL.Func(
            [],
            [
                IDL.Func(
                    [
                        IDL.Record({
                            id: IDL.Text,
                            basic_func: IDL.Func(
                                [IDL.Text],
                                [IDL.Text],
                                ['query']
                            ),
                            complex_func: ComplexFunc
                        }),
                        IDL.Variant({
                            Bad: IDL.Null,
                            ComplexFunc: ComplexFunc,
                            Good: IDL.Null,
                            BasicFunc: IDL.Func(
                                [IDL.Text],
                                [IDL.Text],
                                ['query']
                            )
                        })
                    ],
                    [IDL.Nat64],
                    []
                )
            ],
            ['query']
        ),
        get_notifier_from_notifiers_canister: IDL.Func(
            [],
            [GetNotifierFromNotifiersCanisterResult],
            []
        ),
        get_stable_func: IDL.Func(
            [],
            [IDL.Func([IDL.Text], [IDL.Text], ['query'])],
            ['query']
        )
    });
};
export const init = ({ IDL }) => {
    return [];
};