### Installation

- `pip install -r requirements.txt`
- `cd haversine_c && python3 ./setup.py install && cd -`
- `cd ant_colony_rs && maturin build -r -i $(basename $(realpath $(which python))) && pip install ./target/wheels/*.whl && cd -`
- `cd haversine_rs && maturin build -r -i $(basename $(realpath $(which python))) && pip install ./target/wheels/*.whl && cd -`