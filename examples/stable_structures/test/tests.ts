import { Test } from 'azle/test';
import {
    BlogPost,
    Reaction,
    User,
    _SERVICE
} from './dfx_generated/stable_structures/stable_structures.did';
import { ActorSubclass } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';

const HELLO_BYTES = [104, 101, 108, 108, 111];
const STABLE_MAP_KEYS = [
    0,
    0,
    0,
    {
        Happy: null
    },
    {
        username: 'username',
        blog_posts: [
            {
                title: 'MyBlogPost'
            }
        ]
    },
    ['world'],
    new BigUint64Array([1n, 2n, 3n, 4n, 5n]),
    null,
    false,
    0,
    10.23,
    0n,
    new Uint8Array(HELLO_BYTES),
    'hello'
];

export function get_set_value_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return [
        {
            name: 'stable_map_0 set and read',
            test: async () => {
                const set_result =
                    await stable_structures_canister.set_stable_map_0(
                        0,
                        'hello'
                    );

                const get_result =
                    await stable_structures_canister.get_stable_map_0(0);

                return {
                    ok: set_result === undefined && get_result[0] === 'hello'
                };
            }
        },
        {
            name: 'stable_map_1 set and read',
            test: async () => {
                const set_result =
                    await stable_structures_canister.set_stable_map_1(
                        0,
                        new Uint8Array(HELLO_BYTES)
                    );

                const get_result =
                    await stable_structures_canister.get_stable_map_1(0);

                return {
                    ok:
                        set_result === undefined &&
                        new Uint8Array(HELLO_BYTES).every(
                            (value, index) => value === get_result[0]?.[index]
                        )
                };
            }
        },
        {
            name: 'stable_map_2 set and read',
            test: async () => {
                const set_result =
                    await stable_structures_canister.set_stable_map_2(0, 2n);

                const get_result =
                    await stable_structures_canister.get_stable_map_2(0);

                return {
                    ok: set_result === undefined && get_result[0] === 2n
                };
            }
        },
        {
            name: 'stable_map_3 set and read',
            test: async () => {
                let key: Reaction = {
                    Happy: null
                };
                const set_result =
                    await stable_structures_canister.set_stable_map_3(key, 2n);

                const get_result =
                    await stable_structures_canister.get_stable_map_3(key);

                return {
                    ok: set_result === undefined && get_result[0] === 2n
                };
            }
        },
        {
            name: 'stable_map_4 set and read',
            test: async () => {
                let blog_post: BlogPost = {
                    title: 'MyBlogPost'
                };
                let key: User = {
                    username: 'username',
                    blog_posts: [blog_post]
                };
                const set_result =
                    await stable_structures_canister.set_stable_map_4(key, 4);

                const get_result =
                    await stable_structures_canister.get_stable_map_4(key);

                return {
                    ok: set_result === undefined && get_result[0] === 4
                };
            }
        },
        {
            name: 'stable_map_5 set and read',
            test: async () => {
                const set_result =
                    await stable_structures_canister.set_stable_map_5(
                        ['world'],
                        5
                    );

                const get_result =
                    await stable_structures_canister.get_stable_map_5([
                        'world'
                    ]);

                return {
                    ok: set_result === undefined && get_result[0] === 5
                };
            }
        },
        {
            name: 'stable_map_6 set and read',
            test: async () => {
                let key = new BigUint64Array([1n, 2n, 3n, 4n, 5n]);
                const set_result =
                    await stable_structures_canister.set_stable_map_6(
                        key,
                        true
                    );

                const get_result =
                    await stable_structures_canister.get_stable_map_6(key);

                return {
                    ok: set_result === undefined && get_result[0] === true
                };
            }
        },
        {
            name: 'stable_map_7 set and read',
            test: async () => {
                const set_result =
                    await stable_structures_canister.set_stable_map_7(
                        null,
                        null
                    );

                const get_result =
                    await stable_structures_canister.get_stable_map_7(null);

                return {
                    ok: set_result === undefined && get_result[0] === null
                };
            }
        },
        {
            name: 'stable_map_8 set and read',
            test: async () => {
                const set_result =
                    await stable_structures_canister.set_stable_map_8(
                        false,
                        null
                    );

                const get_result =
                    await stable_structures_canister.get_stable_map_8(false);

                return {
                    ok: set_result === undefined && get_result[0] === null
                };
            }
        },
        {
            name: 'stable_map_9 set and read',
            test: async () => {
                const set_result =
                    await stable_structures_canister.set_stable_map_9(0, [
                        'hello',
                        'world'
                    ]);

                const get_result =
                    await stable_structures_canister.get_stable_map_9(0);

                return {
                    ok:
                        get_result[0]?.length == 2 &&
                        get_result[0][0] === 'hello' &&
                        get_result[0][1] === 'world'
                };
            }
        },
        {
            name: 'stable_map_10 set and read',
            test: async () => {
                const set_result =
                    await stable_structures_canister.set_stable_map_10(10.23, [
                        true
                    ]);

                const get_result =
                    await stable_structures_canister.get_stable_map_10(10.23);

                return {
                    ok: set_result === undefined && get_result[0]?.[0] === true
                };
            }
        },
        {
            name: 'stable_map_11 set and read',
            test: async () => {
                let blog_post: BlogPost = {
                    title: 'BlagPost'
                };
                let user: User = {
                    username: 'username2',
                    blog_posts: [blog_post]
                };
                const set_result =
                    await stable_structures_canister.set_stable_map_11(
                        0n,
                        user
                    );

                const get_result =
                    await stable_structures_canister.get_stable_map_11(0n);

                return {
                    ok:
                        set_result === undefined &&
                        get_result[0]?.username === user.username &&
                        get_result[0]?.blog_posts?.[0]?.title ===
                            blog_post.title
                };
            }
        },
        {
            name: 'stable_map_12 set and read',
            test: async () => {
                let value: Reaction = {
                    Sad: null
                };
                const set_result =
                    await stable_structures_canister.set_stable_map_12(
                        new Uint8Array(HELLO_BYTES),
                        value
                    );

                const get_result =
                    await stable_structures_canister.get_stable_map_12(
                        new Uint8Array(HELLO_BYTES)
                    );

                return {
                    ok:
                        set_result === undefined &&
                        get_result[0] != undefined &&
                        'Sad' in get_result[0]
                };
            }
        },
        {
            name: 'stable_map_13 set and read',
            test: async () => {
                const set_result =
                    await stable_structures_canister.set_stable_map_13(
                        'hello',
                        Principal.fromText('aaaaa-aa')
                    );

                const get_result =
                    await stable_structures_canister.get_stable_map_13('hello');

                return {
                    ok:
                        set_result === undefined &&
                        get_result[0]?.toText() ===
                            Principal.fromText('aaaaa-aa').toText()
                };
            }
        }
    ];
}

export function get_contains_key_that_exists_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return [
        {
            name: 'stable_map_0 contains key that exists',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_0(
                        0
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_1 contains key that exists',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_1(
                        0
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_2 contains key that exists',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_2(
                        0
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_3 contains key that exists',
            test: async () => {
                let key: Reaction = {
                    Happy: null
                };
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_3(
                        key
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_4 contains key that exists',
            test: async () => {
                let blog_post: BlogPost = {
                    title: 'MyBlogPost'
                };
                let key: User = {
                    username: 'username',
                    blog_posts: [blog_post]
                };
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_4(
                        key
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_5 contains key that exists',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_5([
                        'world'
                    ]);

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_6 contains key that exists',
            test: async () => {
                let key = new BigUint64Array([1n, 2n, 3n, 4n, 5n]);
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_6(
                        key
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_7 contains key that exists',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_7(
                        null
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_8 contains key that exists',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_8(
                        false
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_9 contains key that exists',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_9(
                        0
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_10 contains key that exists',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_10(
                        10.23
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_11 contains key that exists',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_11(
                        0n
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_12 contains key that exists',
            test: async () => {
                let value: Reaction = {
                    Sad: null
                };
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_12(
                        new Uint8Array(HELLO_BYTES)
                    );

                return {
                    ok: set_result
                };
            }
        },
        {
            name: 'stable_map_13 contains key that exists',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_13(
                        'hello'
                    );

                return {
                    ok: set_result
                };
            }
        }
    ];
}

export function get_contains_key_that_doesnt_exists_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return [
        {
            name: 'stable_map_0 contains key that doesnt exits',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_0(
                        100
                    );

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_1 contains key that doesnt exits',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_1(
                        100
                    );

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_2 contains key that doesnt exits',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_2(
                        100
                    );

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_3 contains key that doesnt exits',
            test: async () => {
                let wrong_key: Reaction = {
                    Sad: null
                };
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_3(
                        wrong_key
                    );

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_4 contains key that doesnt exits',
            test: async () => {
                let blog_post: BlogPost = {
                    title: 'MyBlogPost'
                };
                let wrong_key: User = {
                    username: 'wrongusername',
                    blog_posts: [blog_post]
                };
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_4(
                        wrong_key
                    );

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_5 contains key that doesnt exits',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_5([
                        'wrongkey'
                    ]);

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_6 contains key that doesnt exits',
            test: async () => {
                let wrong_key = new BigUint64Array([10n, 20n, 30n, 40n, 50n]);
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_6(
                        wrong_key
                    );

                return {
                    ok: !set_result
                };
            }
        },
        // TODO I don't think we can do this test here?
        // {
        //     name: 'stable_map_7 contains key that doesnt exits',
        //     test: async () => {
        //         const set_result =
        //             await stable_structures_canister.contains_key_stable_map_7(
        //                 null
        //             );

        //         return {
        //             ok: !set_result
        //         };
        //     }
        // },
        {
            name: 'stable_map_8 contains key that doesnt exits',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_8(
                        true
                    );

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_9 contains key that doesnt exits',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_9(
                        100
                    );

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_10 contains key that doesnt exits',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_10(
                        100
                    );

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_11 contains key that doesnt exits',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_11(
                        100n
                    );

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_12 contains key that doesnt exits',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_12(
                        new Uint8Array([1, 2, 3])
                    );

                return {
                    ok: !set_result
                };
            }
        },
        {
            name: 'stable_map_13 contains key that doesnt exits',
            test: async () => {
                const set_result =
                    await stable_structures_canister.contains_key_stable_map_13(
                        'wrong_key'
                    );

                return {
                    ok: !set_result
                };
            }
        }
    ];
}

function generate_pre_value_tests(
    num: number,
    key: any,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} initial read auto generated`,
        test: async () => {
            const get_result = (stable_structures_canister as any)[
                `get_stable_map_${num}`
            ](key);

            return {
                ok: get_result[0] === undefined
            };
        }
    };
}

export function get_pre_value_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((value, index) =>
        generate_pre_value_tests(index, value, stable_structures_canister)
    );
    return [
        {
            name: 'stable_map_0 initial read',
            test: async () => {
                const thing =
                    stable_structures_canister[`get_stable_map_${0}`](0);
                const get_result =
                    await stable_structures_canister.get_stable_map_0(0);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_1 initial read',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_1(0);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_2 initial read',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_2(0);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_3 initial read',
            test: async () => {
                let key: Reaction = {
                    Happy: null
                };
                const get_result =
                    await stable_structures_canister.get_stable_map_3(key);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_4 initial read',
            test: async () => {
                let blog_post: BlogPost = {
                    title: 'MyBlogPost'
                };
                let key: User = {
                    username: 'username',
                    blog_posts: [blog_post]
                };
                const get_result =
                    await stable_structures_canister.get_stable_map_4(key);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_5 initial read',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_5([
                        'world'
                    ]);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_6 initial read',
            test: async () => {
                let key = new BigUint64Array([1n, 2n, 3n, 4n, 5n]);

                const get_result =
                    await stable_structures_canister.get_stable_map_6(key);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_7 initial read',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_7(null);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_8 initial read',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_8(false);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_9 initial read',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_9(0);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_10 initial read',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_10(10.23);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_11 initial read',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_11(0n);

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_12 initial read',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_12(
                        new Uint8Array(HELLO_BYTES)
                    );

                return {
                    ok: get_result[0] === undefined
                };
            }
        },
        {
            name: 'stable_map_13 initial read',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_13('hello');

                return {
                    ok: get_result[0] === undefined
                };
            }
        }
    ];
}

export function get_get_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return [
        {
            name: 'stable_map_0 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_0(0);

                return {
                    ok: get_result[0] === 'hello'
                };
            }
        },
        {
            name: 'stable_map_1 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_1(0);

                return {
                    ok: new Uint8Array(HELLO_BYTES).every(
                        (value, index) => value === get_result[0]?.[index]
                    )
                };
            }
        },
        {
            name: 'stable_map_2 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_2(0);

                return {
                    ok: get_result[0] === 2n
                };
            }
        },
        {
            name: 'stable_map_3 get test',
            test: async () => {
                let key: Reaction = {
                    Happy: null
                };
                const get_result =
                    await stable_structures_canister.get_stable_map_3(key);

                return {
                    ok: get_result[0] === 2n
                };
            }
        },
        {
            name: 'stable_map_4 get test',
            test: async () => {
                let blog_post: BlogPost = {
                    title: 'MyBlogPost'
                };
                let key: User = {
                    username: 'username',
                    blog_posts: [blog_post]
                };
                const get_result =
                    await stable_structures_canister.get_stable_map_4(key);

                return {
                    ok: get_result[0] === 4
                };
            }
        },
        {
            name: 'stable_map_5 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_5([
                        'world'
                    ]);

                return {
                    ok: get_result[0] === 5
                };
            }
        },
        {
            name: 'stable_map_6 get test',
            test: async () => {
                let key = new BigUint64Array([1n, 2n, 3n, 4n, 5n]);

                const get_result =
                    await stable_structures_canister.get_stable_map_6(key);

                return {
                    ok: get_result[0] === true
                };
            }
        },
        {
            name: 'stable_map_7 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_7(null);

                return {
                    ok: get_result[0] === null
                };
            }
        },
        {
            name: 'stable_map_8 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_8(false);

                return {
                    ok: get_result[0] === null
                };
            }
        },
        {
            name: 'stable_map_9 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_9(0);

                return {
                    ok:
                        get_result[0]?.length == 2 &&
                        get_result[0][0] === 'hello' &&
                        get_result[0][1] === 'world'
                };
            }
        },
        {
            name: 'stable_map_10 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_10(10.23);

                return {
                    ok: get_result[0]?.[0] === true
                };
            }
        },
        {
            name: 'stable_map_11 get test',
            test: async () => {
                let blog_post: BlogPost = {
                    title: 'BlagPost'
                };
                let user: User = {
                    username: 'username2',
                    blog_posts: [blog_post]
                };

                const get_result =
                    await stable_structures_canister.get_stable_map_11(0n);

                return {
                    ok:
                        get_result[0]?.username === user.username &&
                        get_result[0]?.blog_posts?.[0]?.title ===
                            blog_post.title
                };
            }
        },
        {
            name: 'stable_map_12 get test',
            test: async () => {
                let value: Reaction = {
                    Sad: null
                };

                const get_result =
                    await stable_structures_canister.get_stable_map_12(
                        new Uint8Array(HELLO_BYTES)
                    );

                return {
                    ok: get_result[0] != undefined && 'Sad' in get_result[0]
                };
            }
        },
        {
            name: 'stable_map_13 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_13('hello');

                return {
                    ok:
                        get_result[0]?.toText() ===
                        Principal.fromText('aaaaa-aa').toText()
                };
            }
        }
    ];
}
