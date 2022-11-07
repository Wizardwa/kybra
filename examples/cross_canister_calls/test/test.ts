import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/cross_canister_calls/test/tests';
import { execSync } from 'child_process';
import { createActor as createActorCanister1 } from './dfx_generated/canister1';
import { createActor as createActorCanister2 } from './dfx_generated/canister2';

const canister1 = createActorCanister1('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const canister2 = createActorCanister2('ryjl3-tyaaa-aaaaa-aaaba-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});
const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code canister1 || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx canister uninstall-code canister2 || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy`, {
                stdio: 'inherit'
            });
        }
    },
    // TODO add this back in once notify is supported
    ...get_tests(canister1, canister2).filter(
        (test) => !test.name.includes('notification')
    )
];

run_tests(tests);