import os
import json
from collections import OrderedDict

def process_file(filepath):
    with open(filepath, 'r') as file:
        data = json.load(file)

    updated = False
    for canister_name, canister_data in data.get('canisters', {}).items():
        if 'build' in canister_data and 'post_install' not in canister_data:
            new_canister_data = OrderedDict()
            for key, value in canister_data.items():
                new_canister_data[key] = value
                if key == 'build':
                    new_canister_data['post_install'] = f'.kybra/{canister_name}/post_install.sh'
            data['canisters'][canister_name] = new_canister_data
            updated = True

    if updated:
        with open(filepath, 'w') as file:
            json.dump(data, file, indent=4)

def find_and_process_dfx_files(directory):
    for root, _, files in os.walk(directory):
        for file in files:
            if file == 'dfx.json':
                filepath = os.path.join(root, file)
                process_file(filepath)

if __name__ == '__main__':
    directory = 'examples'
    find_and_process_dfx_files(directory)
